use std::collections::HashMap;

use actix::Addr;
use actix_web::web::Data;

use jirs_data::WsMsg;

use crate::db::issue_assignees::LoadAssignees;
use crate::db::issues::{LoadProjectIssues, UpdateIssue};
use crate::db::DbExecutor;
use crate::ws::{current_user, WsResult};

pub async fn update_issue(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    issue_id: i32,
    payload: jirs_data::UpdateIssuePayload,
) -> WsResult {
    current_user(user)?;
    let mut issue: jirs_data::Issue = match db
        .send(UpdateIssue {
            issue_id,
            title: Some(payload.title),
            issue_type: Some(payload.issue_type),
            status: Some(payload.status),
            priority: Some(payload.priority),
            list_position: Some(payload.list_position),
            description: Some(payload.description),
            description_text: Some(payload.description_text),
            estimate: Some(payload.estimate),
            time_spent: Some(payload.time_spent),
            time_remaining: Some(payload.time_remaining),
            project_id: Some(payload.project_id),
            user_ids: Some(payload.user_ids),
            reporter_id: Some(payload.reporter_id),
        })
        .await
    {
        Ok(Ok(issue)) => issue.into(),
        _ => return Ok(None),
    };

    let assignees = match db
        .send(LoadAssignees {
            issue_id: issue.id.clone(),
        })
        .await
    {
        Ok(Ok(v)) => v,
        _ => vec![],
    };
    for assignee in assignees {
        issue.user_ids.push(assignee.user_id);
    }

    Ok(Some(WsMsg::IssueUpdated(issue.into())))
}

pub async fn add_issue(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    payload: jirs_data::CreateIssuePayload,
) -> WsResult {
    current_user(user)?;
    let msg = crate::db::issues::CreateIssue {
        title: payload.title,
        issue_type: payload.issue_type,
        status: payload.status,
        priority: payload.priority,
        description: payload.description,
        description_text: payload.description_text,
        estimate: payload.estimate,
        time_spent: payload.time_spent,
        time_remaining: payload.time_remaining,
        project_id: payload.project_id,
        reporter_id: payload.reporter_id,
        user_ids: payload.user_ids,
    };
    let m = match db.send(msg).await {
        Ok(Ok(issue)) => Some(WsMsg::IssueCreated(issue.into())),
        _ => None,
    };
    Ok(m)
}

pub async fn delete_issue(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    id: i32,
) -> WsResult {
    current_user(user)?;
    let m = match db
        .send(crate::db::issues::DeleteIssue { issue_id: id })
        .await
    {
        Ok(Ok(_)) => Some(WsMsg::IssueDeleted(id)),
        _ => None,
    };
    Ok(m)
}

pub async fn load_issues(db: &Data<Addr<DbExecutor>>, user: &Option<jirs_data::User>) -> WsResult {
    let project_id = current_user(user).map(|u| u.project_id)?;

    let issues: Vec<jirs_data::Issue> = match db.send(LoadProjectIssues { project_id }).await {
        Ok(Ok(v)) => v.into_iter().map(|i| i.into()).collect(),
        _ => return Ok(None),
    };
    let mut issue_map = HashMap::new();
    let mut queue = vec![];
    for issue in issues.into_iter() {
        let f = db.send(LoadAssignees {
            issue_id: issue.id.clone(),
        });
        queue.push(f);
        issue_map.insert(issue.id.clone(), issue);
    }
    for f in queue {
        match f.await {
            Ok(Ok(assignees)) => {
                for assignee in assignees {
                    if let Some(issue) = issue_map.get_mut(&assignee.issue_id) {
                        issue.user_ids.push(assignee.user_id);
                    }
                }
            }
            _ => {}
        };
    }
    let mut issues = vec![];
    for (_, issue) in issue_map.into_iter() {
        issues.push(issue);
    }

    Ok(Some(WsMsg::ProjectIssuesLoaded(issues)))
}
