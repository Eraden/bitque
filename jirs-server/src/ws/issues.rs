use std::collections::HashMap;

use actix::*;
use actix_web::web::Data;

use jirs_data::{IssueFieldId, PayloadVariant, WsMsg};

use crate::db::issue_assignees::LoadAssignees;
use crate::db::issues::{LoadProjectIssues, UpdateIssue};
use crate::db::DbExecutor;
use crate::ws::{current_user, WsResult};

/*
pub struct UpdateIssueHandler {
    id: i32,
    field_id: IssueFieldId,
    payload: PayloadVariant,
}

impl Message for UpdateIssueHandler {
    type Result = WsResult;
}

impl Actor for UpdateIssueHandler {
    type Context = Context<Self>;
}

impl Handler<UpdateIssueHandler> for WebSocketActor {
    type Result = WsResult;

    fn handle(&mut self, msg: UpdateIssueHandler, ctx: &mut Self::Context) -> Self::Result {
        self.require_user()?;

        let UpdateIssueHandler {
            id,
            field_id,
            payload,
        } = msg;

        let mut msg = UpdateIssue::default();
        msg.issue_id = id;
        match (field_id, payload) {
            (IssueFieldId::Type, PayloadVariant::IssueType(t)) => {
                msg.issue_type = Some(t);
            }
            (IssueFieldId::Title, PayloadVariant::String(s)) => {
                msg.title = Some(s);
            }
            (IssueFieldId::Description, PayloadVariant::String(s)) => {
                msg.description = Some(s);
            }
            (IssueFieldId::Status, PayloadVariant::IssueStatus(s)) => {
                msg.status = Some(s);
            }
            (IssueFieldId::ListPosition, PayloadVariant::I32(i)) => {
                msg.list_position = Some(i);
            }
            (IssueFieldId::Assignees, PayloadVariant::VecI32(v)) => {
                msg.user_ids = Some(v);
            }
            (IssueFieldId::Reporter, PayloadVariant::I32(i)) => {
                msg.reporter_id = Some(i);
            }
            (IssueFieldId::Priority, PayloadVariant::IssuePriority(p)) => {
                msg.priority = Some(p);
            }
            (IssueFieldId::Estimate, PayloadVariant::OptionI32(o)) => {
                msg.estimate = o;
            }
            (IssueFieldId::TimeSpend, PayloadVariant::OptionI32(o)) => {
                msg.time_spent = o;
            }
            (IssueFieldId::TimeRemaining, PayloadVariant::OptionI32(o)) => {
                msg.time_remaining = o;
            }
            _ => (),
        };

        let mut updated: Option<jirs_data::Issue> = None;

        self.db
            .send(msg)
            .into_actor(self)
            .then(move |res, _act, _ctx| {
                updated = res.ok().and_then(|r| r.ok()).map(|i| i.into());
                fut::ready(())
            })
            .wait(ctx);

        let mut issue = match updated {
            Some(issue) => issue,
            _ => return Ok(None),
        };

        let mut assignees = vec![];

        self.db
            .send(LoadAssignees { issue_id: issue.id })
            .into_actor(self)
            .then(|res, _act, _ctx| {
                if let Ok(Ok(v)) = res {
                    assignees = v;
                }
                fut::ready(())
            })
            .wait(ctx);

        for assignee in assignees {
            issue.user_ids.push(assignee.user_id);
        }

        Ok(Some(WsMsg::IssueUpdated(issue)))
    }
}
*/

pub async fn update_issue(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    issue_id: i32,
    issue_field_id: IssueFieldId,
    payload: PayloadVariant,
) -> WsResult {
    current_user(user)?;

    let mut msg = UpdateIssue::default();
    msg.issue_id = issue_id;
    match (issue_field_id, payload) {
        (IssueFieldId::Type, PayloadVariant::IssueType(t)) => {
            msg.issue_type = Some(t);
        }
        (IssueFieldId::Title, PayloadVariant::String(s)) => {
            msg.title = Some(s);
        }
        (IssueFieldId::Description, PayloadVariant::String(s)) => {
            msg.description = Some(s);
        }
        (IssueFieldId::Status, PayloadVariant::IssueStatus(s)) => {
            msg.status = Some(s);
        }
        (IssueFieldId::ListPosition, PayloadVariant::I32(i)) => {
            msg.list_position = Some(i);
        }
        (IssueFieldId::Assignees, PayloadVariant::VecI32(v)) => {
            msg.user_ids = Some(v);
        }
        (IssueFieldId::Reporter, PayloadVariant::I32(i)) => {
            msg.reporter_id = Some(i);
        }
        (IssueFieldId::Priority, PayloadVariant::IssuePriority(p)) => {
            msg.priority = Some(p);
        }
        (IssueFieldId::Estimate, PayloadVariant::OptionI32(o)) => {
            msg.estimate = o;
        }
        (IssueFieldId::TimeSpend, PayloadVariant::OptionI32(o)) => {
            msg.time_spent = o;
        }
        (IssueFieldId::TimeRemaining, PayloadVariant::OptionI32(o)) => {
            msg.time_remaining = o;
        }
        _ => (),
    };

    let mut issue: jirs_data::Issue = match db.send(msg).await {
        Ok(Ok(issue)) => issue.into(),
        _ => return Ok(None),
    };

    let assignees = match db.send(LoadAssignees { issue_id: issue.id }).await {
        Ok(Ok(v)) => v,
        _ => vec![],
    };
    for assignee in assignees {
        issue.user_ids.push(assignee.user_id);
    }

    Ok(Some(WsMsg::IssueUpdated(issue)))
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
        let f = db.send(LoadAssignees { issue_id: issue.id });
        queue.push(f);
        issue_map.insert(issue.id, issue);
    }
    for f in queue {
        if let Ok(Ok(assignees)) = f.await {
            for assignee in assignees {
                if let Some(issue) = issue_map.get_mut(&assignee.issue_id) {
                    issue.user_ids.push(assignee.user_id);
                }
            }
        };
    }
    let mut issues = vec![];
    for (_, issue) in issue_map.into_iter() {
        issues.push(issue);
    }

    Ok(Some(WsMsg::ProjectIssuesLoaded(issues)))
}
