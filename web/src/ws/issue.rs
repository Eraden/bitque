use jirs_data::msg::WsMsgIssue;
use jirs_data::*;
use seed::prelude::Orders;
use seed::*;

use crate::model::{Model, PageContent};
use crate::pages::project_page::ProjectPage;
use crate::ws::send_ws_msg;
use crate::Msg;

pub fn drag_started(issue_id: EpicId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };
    project_page.issue_drag.drag(issue_id);
}

pub fn change_position(below_id: EpicId, model: &mut Model) {
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
        .issues()
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
        .issues_mut()
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
        model.issues_mut().push(issue);
    }

    let visible = ProjectPage::visible_issues(
        crate::match_page!(model, Project),
        model.epics(),
        model.issue_statuses(),
        model.issues(),
        model.user(),
    );
    if let PageContent::Project(project_page) = &mut model.page_content {
        project_page.visible_issues = visible;
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
        WsMsg::Issue(WsMsgIssue::IssueSyncListPosition(changes)),
        model.ws.as_ref(),
        orders,
    );
    crate::match_page_mut!(model, Project).issue_drag.clear();
}

pub fn change_status(status_id: IssueStatusId, model: &mut Model) {
    let dragged_id = match crate::match_page!(model, Project)
        .issue_drag
        .dragged_id
        .as_ref()
        .cloned()
    {
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

    let mut issues = {
        let mut h = std::mem::take(&mut model.issues_by_id);
        h.keys()
            .filter_map(|id| h.remove(id))
            .collect::<Vec<Issue>>()
    }
    .drain_filter(|issue| {
        if issue.id == dragged_id {
            issue.issue_status_id = status_id;
        }
        issue.issue_status_id == status_id && issue.epic_id == epic_id
    })
    .collect::<Vec<Issue>>();

    issues.sort_by(|a, b| a.list_position.cmp(&b.list_position));

    let mut dirty = vec![];
    for mut issue in issues {
        if issue.id == dragged_id {
            issue.issue_status_id = status_id;
            if let Some(iss) = model.issues_by_id.get_mut(&issue.id) {
                iss.issue_status_id = status_id;
            }
        }

        dirty.push(issue.id);
        model.issue_ids.push(issue.id);
        model.issues_by_id.insert(issue.id, issue);
    }
    {
        let project_page = crate::match_page_mut!(model, Project);
        for id in dirty {
            project_page.issue_drag.mark_dirty(id);
        }
    }

    let visible = ProjectPage::visible_issues(
        crate::match_page!(model, Project),
        model.epics(),
        model.issue_statuses(),
        model
            .issue_ids
            .iter()
            .filter_map(|id| model.issues_by_id.get(id)),
        model.user(),
    );

    crate::match_page_mut!(model, Project).visible_issues = visible;
}
