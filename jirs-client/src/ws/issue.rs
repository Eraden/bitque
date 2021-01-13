use seed::prelude::Orders;
use seed::*;

use jirs_data::*;

use crate::model::{Model, PageContent};
use crate::ws::send_ws_msg;
use crate::Msg;

pub fn drag_started(issue_id: IssueId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };
    project_page.issue_drag.drag(issue_id);
}

pub fn exchange_position(below_id: IssueId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };
    let dragged_id = match project_page.issue_drag.dragged_id.as_ref().cloned() {
        Some(id) => id,
        _ => return error!("Nothing is dragged"),
    };
    if below_id == dragged_id {
        return;
    }
    let below_idx = model
        .issues
        .iter()
        .position(|issue| issue.id == below_id)
        .unwrap_or(model.issues.len());
    let dragged = model
        .issues
        .iter()
        .position(|issue| issue.id == dragged_id)
        .map(|idx| model.issues.remove(idx))
        .unwrap();
    let epic_id = dragged.epic_id;
    model.issues.insert(below_idx, dragged);
    let changed: Vec<(IssueId, i32)> = model
        .issues
        .iter_mut()
        .filter(|issue| issue.epic_id == epic_id)
        .enumerate()
        .map(|(idx, issue)| {
            issue.list_position = idx as i32;
            (issue.id, issue.list_position)
        })
        .collect();
    for (id, pos) in changed {
        if let Some(iss) = model.issues_by_id.get_mut(&id) {
            iss.list_position = pos;
        }
    }

    // let dragged_pos = match model.issues_by_id.get(&dragged_id) {
    //     Some(i) => i.list_position,
    //     _ => return,
    // };
    // let below_pos = match model.issues_by_id.get(&below_id) {
    //     Some(i) => i.list_position,
    //     _ => return,
    // };
    // use seed::*;
    // log!(format!(
    //     "exchange dragged {} {} below {} {}",
    //     dragged_id, dragged_pos, below_id, below_pos
    // ));
    // for issue in model.issues_by_id.values_mut() {
    //     if issue.id == below_id {
    //         issue.list_position = dragged_pos;
    //     } else if issue.id == dragged_id {
    //         issue.list_position = below_pos;
    //     }
    // }
    //
    // for issue in model.issues.iter_mut() {
    //     if issue.id == below_id {
    //         issue.list_position = dragged_pos;
    //     } else if issue.id == dragged_id {
    //         issue.list_position = below_pos;
    //     }
    // }
    // model
    //     .issues
    //     .sort_by(|a, b| a.list_position.cmp(&b.list_position));
    if let PageContent::Project(project_page) = &mut model.page_content {
        project_page.rebuild_visible(
            &model.epics,
            &model.issue_statuses,
            &model.issues,
            &model.user,
        );
        project_page.issue_drag.mark_dirty(dragged_id);
        project_page.issue_drag.mark_dirty(below_id);
    }
}

pub fn sync(model: &mut Model, orders: &mut impl Orders<Msg>) {
    // log!("------------------------------------------------------------------");
    // log!("|                SYNC                                            |");
    // log!("------------------------------------------------------------------");
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };

    for issue in model.issues.iter() {
        if !project_page.issue_drag.dirty.contains(&issue.id) {
            continue;
        }

        send_ws_msg(
            WsMsg::IssueUpdate(
                issue.id,
                IssueFieldId::IssueStatusId,
                PayloadVariant::I32(issue.issue_status_id),
            ),
            model.ws.as_ref(),
            orders,
        );
        send_ws_msg(
            WsMsg::IssueUpdate(
                issue.id,
                IssueFieldId::ListPosition,
                PayloadVariant::I32(issue.list_position),
            ),
            model.ws.as_ref(),
            orders,
        );
    }
    project_page.issue_drag.clear();
}

pub fn change_status(status_id: IssueStatusId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };

    let issue_id = match project_page.issue_drag.dragged_id.as_ref().cloned() {
        Some(issue_id) => issue_id,
        _ => return error!("Nothing is dragged"),
    };

    let mut old: Vec<Issue> = vec![];
    let mut pos = 0;
    let mut found: Option<Issue> = None;
    std::mem::swap(&mut old, &mut model.issues);
    old.sort_by(|a, b| a.list_position.cmp(&b.list_position));

    for mut issue in old.into_iter() {
        if issue.issue_status_id == status_id {
            if issue.list_position != pos {
                issue.list_position = pos;
                project_page.issue_drag.mark_dirty(issue.id);
            }
            pos += 1;
        }
        if issue.id != issue_id {
            model.issues.push(issue);
        } else {
            found = Some(issue);
        }
    }

    let mut issue = match found {
        Some(i) => i,
        _ => {
            return;
        }
    };

    if issue.issue_status_id == status_id {
        model.issues.push(issue);
    } else {
        issue.issue_status_id = status_id;
        issue.list_position = pos + 1;
        model.issues.push(issue);
        project_page.issue_drag.mark_dirty(issue_id);
    }
}
