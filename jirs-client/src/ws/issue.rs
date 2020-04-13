use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{Model, PageContent, ProjectPage};

pub fn drag_started(issue_id: IssueId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };
    project_page.dragged_issue_id = Some(issue_id);

    mark_dirty(issue_id, project_page);
}

pub fn exchange_position(issue_bellow_id: IssueId, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };
    if project_page.dragged_issue_id == Some(issue_bellow_id)
        || project_page.last_drag_exchange_id == Some(issue_bellow_id)
    {
        return;
    }
    let dragged_id = match project_page.dragged_issue_id {
        Some(id) => id,
        _ => return,
    };

    let mut below = None;
    let mut dragged = None;
    let mut issues = vec![];
    std::mem::swap(&mut issues, &mut model.issues);

    for issue in issues.into_iter() {
        match issue.id {
            id if id == issue_bellow_id => below = Some(issue),
            id if id == dragged_id => dragged = Some(issue),
            _ => model.issues.push(issue),
        };
    }

    let mut below = match below {
        Some(below) => below,
        _ => return,
    };
    let mut dragged = match dragged {
        Some(issue) => issue,
        _ => {
            model.issues.push(below);
            return;
        }
    };
    if dragged.status != below.status {
        let mut issues = vec![];
        std::mem::swap(&mut issues, &mut model.issues);
        for mut c in issues.into_iter() {
            if c.status == below.status && c.list_position > below.list_position {
                c.list_position += 1;
                mark_dirty(c.id, project_page);
            }
            model.issues.push(c);
        }
        dragged.list_position = below.list_position + 1;
        dragged.status = below.status.clone();
    }
    std::mem::swap(&mut dragged.list_position, &mut below.list_position);

    mark_dirty(dragged.id, project_page);
    mark_dirty(below.id, project_page);

    model.issues.push(below);
    model.issues.push(dragged);
    model
        .issues
        .sort_by(|a, b| a.list_position.cmp(&b.list_position));
    project_page.last_drag_exchange_id = Some(issue_bellow_id);
}

pub fn dropped(_status: IssueStatus, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };

    for issue in model.issues.iter() {
        if !project_page.dirty_issues.contains(&issue.id) {
            continue;
        }

        let payload = UpdateIssuePayload {
            title: issue.title.clone(),
            issue_type: issue.issue_type.clone(),
            status: issue.status.clone(),
            priority: issue.priority.clone(),
            list_position: issue.list_position,
            description: issue.description.clone(),
            description_text: issue.description_text.clone(),
            estimate: issue.estimate,
            time_spent: issue.time_spent,
            time_remaining: issue.time_remaining,
            project_id: issue.project_id,
            reporter_id: issue.reporter_id,
            user_ids: issue.user_ids.clone(),
        };
        project_page.dragged_issue_id = None;
        send_ws_msg(WsMsg::IssueUpdateRequest(issue.id, payload));
        project_page.last_drag_exchange_id = None;
    }
}

pub fn change_status(status: IssueStatus, model: &mut Model) {
    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };

    let issue_id = match project_page.dragged_issue_id.as_ref().cloned() {
        Some(issue_id) => issue_id,
        _ => return,
    };

    let mut old: Vec<Issue> = vec![];
    let mut pos = 0;
    let mut found: Option<Issue> = None;
    std::mem::swap(&mut old, &mut model.issues);
    old.sort_by(|a, b| a.list_position.cmp(&b.list_position));

    for mut issue in old.into_iter() {
        if issue.status == status {
            issue.list_position = pos;
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

    if issue.status == status {
        model.issues.push(issue);
        return;
    }
    issue.status = status.clone();
    issue.list_position = pos + 1;
    model.issues.push(issue);

    mark_dirty(issue_id, project_page);
}

#[inline]
fn mark_dirty(id: IssueId, project_page: &mut ProjectPage) {
    if !project_page.dirty_issues.contains(&id) {
        project_page.dirty_issues.push(id);
    }
}
