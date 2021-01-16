use {
    crate::{
        model::{Model, PageContent},
        ws::send_ws_msg,
        Msg,
    },
    jirs_data::*,
    seed::{prelude::Orders, *},
};

pub fn drag_started(issue_id: EpicId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };
    project_page.issue_drag.drag(issue_id);
}

pub fn exchange_position(below_id: EpicId, model: &mut Model) {
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

    let (issue_status_id, epic_id) = model
        .issues
        .iter()
        .find_map(|issue| {
            if issue.id == dragged_id {
                Some((issue.issue_status_id, issue.epic_id))
            } else {
                None
            }
        })
        .unwrap_or_default();

    let mut issues: Vec<Issue> = model
        .issues
        .drain_filter(|issue| issue.issue_status_id == issue_status_id && issue.epic_id == epic_id)
        .collect();
    issues.sort_by(|a, b| a.list_position.cmp(&b.list_position));

    let below_idx = issues
        .iter()
        .position(|issue| issue.id == below_id)
        .unwrap_or_default();
    let dragged_idx = issues
        .iter()
        .position(|issue| issue.id == dragged_id)
        .unwrap_or_default();

    let dragged = issues.remove(dragged_idx);
    issues.insert(below_idx, dragged);

    let mut changed = Vec::with_capacity(issues.len());
    for (idx, mut issue) in issues.into_iter().enumerate() {
        issue.list_position = idx as i32;
        if let Some(iss) = model.issues_by_id.get_mut(&issue.id) {
            iss.list_position = issue.list_position;
        }
        changed.push((issue.id, issue.list_position));
        model.issues.push(issue);
    }

    if let PageContent::Project(project_page) = &mut model.page_content {
        project_page.rebuild_visible(
            &model.epics,
            &model.issue_statuses,
            &model.issues,
            &model.user,
        );
        for (id, _) in changed.iter() {
            project_page.issue_drag.mark_dirty(*id);
        }
    }
}

pub fn sync(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let dirty = match &mut model.page_content {
        PageContent::Project(project_page) => std::mem::take(&mut project_page.issue_drag.dirty),
        _ => return,
    };

    let changes: Vec<(EpicId, ListPosition, IssueStatusId, Option<EpicId>)> = dirty
        .into_iter()
        .filter_map(|id| {
            model.issues_by_id.get(&id).map(|issue| {
                (
                    issue.id,
                    issue.list_position,
                    issue.issue_status_id,
                    issue.epic_id,
                )
            })
        })
        .collect();

    send_ws_msg(
        WsMsg::IssueSyncListPosition(changes),
        model.ws.as_ref(),
        orders,
    );
    if let PageContent::Project(project_page) = &mut model.page_content {
        project_page.issue_drag.clear()
    };
}

pub fn change_status(status_id: IssueStatusId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };

    let dragged_id = match project_page.issue_drag.dragged_id.as_ref().cloned() {
        Some(issue_id) => issue_id,
        _ => return error!("Nothing is dragged"),
    };
    let (issue_status_id, epic_id) = model
        .issues_by_id
        .get(&dragged_id)
        .map(|issue| (issue.issue_status_id, issue.epic_id))
        .unwrap_or_default();
    if status_id == issue_status_id {
        return;
    }

    let mut issues: Vec<Issue> = model
        .issues
        .drain_filter(|issue| {
            if issue.id == dragged_id {
                issue.issue_status_id = status_id;
            }
            issue.issue_status_id == status_id && issue.epic_id == epic_id
        })
        .collect();

    issues.sort_by(|a, b| a.list_position.cmp(&b.list_position));

    for mut issue in issues {
        if issue.id == dragged_id {
            issue.issue_status_id = status_id;
            if let Some(iss) = model.issues_by_id.get_mut(&issue.id) {
                iss.issue_status_id = status_id;
            }
        }
        project_page.issue_drag.mark_dirty(issue.id);
        model.issues.push(issue);
    }

    project_page.rebuild_visible(
        &model.epics,
        &model.issue_statuses,
        &model.issues,
        &model.user,
    );
}
