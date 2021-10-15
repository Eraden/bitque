use jirs_data::msg::{IssueSync, WsMsgIssue};
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
        .issues_by_id
        .get(&dragged_id)
        .map(|issue| (issue.issue_status_id, issue.epic_id))
        .unwrap_or_default();

    let mut issues = model
        .issue_ids
        .iter()
        .filter_map(|id| {
            model
                .issues_by_id
                .get(id)
                .filter(|issue| {
                    issue.issue_status_id == issue_status_id && issue.epic_id == epic_id
                })
                .map(|issue| (issue.id, issue.list_position))
        })
        .collect::<Vec<(i32, i32)>>();
    issues.sort_by(|(_, a), (_, b)| a.cmp(b));

    {
        let below_idx = issues
            .iter()
            .position(|(id, _)| *id == below_id)
            .unwrap_or_default();
        let dragged_idx = issues
            .iter()
            .position(|(id, _)| *id == dragged_id)
            .unwrap_or_default();
        let dragged = issues.remove(dragged_idx);
        issues.insert(below_idx, dragged);
    }

    {
        for (idx, (id, _)) in issues.into_iter().enumerate() {
            if let Some(issue) = model.issues_by_id.get_mut(&id) {
                issue.list_position = idx as i32;
            }
            crate::match_page_mut!(model, Project)
                .issue_drag
                .mark_dirty(id);
        }
    }

    change_visible(model);
}

pub fn sync(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let dirty = match &mut model.page_content {
        PageContent::Project(project_page) => std::mem::take(&mut project_page.issue_drag.dirty),
        _ => return,
    };

    let changes = dirty
        .into_iter()
        .filter_map(|id| {
            model.issues_by_id.get(&id).map(|issue| IssueSync {
                id: issue.id,
                list_position: issue.list_position,
                issue_status_id: issue.issue_status_id,
                epic_id: issue.epic_id,
            })
        })
        .collect::<Vec<IssueSync>>();

    send_ws_msg(
        WsMsg::Issue(WsMsgIssue::IssueSyncListPosition(changes)),
        model.ws.as_ref(),
        orders,
    );
    crate::match_page_mut!(model, Project).issue_drag.clear();
}

pub fn change_status(status_id: IssueStatusId, model: &mut Model) -> bool {
    let dragged_id = match crate::match_page!(model, Project, false)
        .issue_drag
        .dragged_id
        .as_ref()
        .cloned()
    {
        Some(issue_id) => issue_id,
        _ => {
            error!("Nothing is dragged");
            return false;
        }
    };
    let (issue_status_id, epic_id) = model
        .issues_by_id
        .get(&dragged_id)
        .map(|issue| (issue.issue_status_id, issue.epic_id))
        .unwrap_or_default();
    if status_id == issue_status_id {
        return false;
    }

    let epic_and_status_issues_count = model
        .issue_ids
        .iter()
        .filter_map(|id| {
            model
                .issues_by_id
                .get(id)
                .filter(|issue| same_epic_and_status(status_id, epic_id, issue))
        })
        .count();
    {
        if let Some(issue) = model.issues_by_id.get_mut(&dragged_id) {
            issue.issue_status_id = status_id;
            issue.list_position = epic_and_status_issues_count as ListPosition;
        }
    }

    let issues_in_column = model
        .issue_ids
        .iter()
        .filter_map(|id| {
            model
                .issues_by_id
                .get(id)
                .filter(|issue| same_epic_and_status(status_id, epic_id, issue))
                .map(|issue| issue.id)
        })
        .collect::<Vec<i32>>();

    {
        let project_page = crate::match_page_mut!(model, Project, false);
        for id in issues_in_column {
            project_page.issue_drag.mark_dirty(id);
        }
    }

    change_visible(model);
    true
}

pub fn change_visible(model: &mut Model) {
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

fn same_epic_and_status(status_id: IssueStatusId, epic_id: Option<EpicId>, issue: &&Issue) -> bool {
    issue.issue_status_id == status_id && issue.epic_id == epic_id
}
