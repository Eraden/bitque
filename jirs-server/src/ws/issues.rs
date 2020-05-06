use std::collections::HashMap;

use futures::executor::block_on;

use jirs_data::{CreateIssuePayload, IssueAssignee, IssueFieldId, IssueId, PayloadVariant, WsMsg};

use crate::db::issue_assignees::LoadAssignees;
use crate::db::issues::{LoadProjectIssues, UpdateIssue};
use crate::ws::{WebSocketActor, WsHandler, WsResult};

pub struct UpdateIssueHandler {
    pub id: i32,
    pub field_id: IssueFieldId,
    pub payload: PayloadVariant,
}

impl WsHandler<UpdateIssueHandler> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateIssueHandler, _ctx: &mut Self::Context) -> WsResult {
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
            (IssueFieldId::IssueStatusId, PayloadVariant::I32(s)) => {
                msg.issue_status_id = Some(s);
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
            (IssueFieldId::TimeSpent, PayloadVariant::OptionI32(o)) => {
                msg.time_spent = o;
            }
            (IssueFieldId::TimeRemaining, PayloadVariant::OptionI32(o)) => {
                msg.time_remaining = o;
            }
            _ => (),
        };

        let mut issue: jirs_data::Issue = match block_on(self.db.send(msg)) {
            Ok(Ok(issue)) => issue.into(),
            _ => return Ok(None),
        };

        let assignees: Vec<IssueAssignee> =
            match block_on(self.db.send(LoadAssignees { issue_id: issue.id })) {
                Ok(Ok(v)) => v,
                _ => return Ok(None),
            };

        for assignee in assignees {
            issue.user_ids.push(assignee.user_id);
        }
        self.broadcast(&WsMsg::IssueUpdated(issue));

        Ok(None)
    }
}

impl WsHandler<CreateIssuePayload> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateIssuePayload, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;
        let msg = crate::db::issues::CreateIssue {
            title: msg.title,
            issue_type: msg.issue_type,
            issue_status_id: msg.issue_status_id,
            priority: msg.priority,
            description: msg.description,
            description_text: msg.description_text,
            estimate: msg.estimate,
            time_spent: msg.time_spent,
            time_remaining: msg.time_remaining,
            project_id: msg.project_id,
            reporter_id: msg.reporter_id,
            user_ids: msg.user_ids,
        };
        let m = match block_on(self.db.send(msg)) {
            Ok(Ok(issue)) => Some(WsMsg::IssueCreated(issue.into())),
            _ => None,
        };
        Ok(m)
    }
}

pub struct DeleteIssue {
    pub id: IssueId,
}

impl WsHandler<DeleteIssue> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteIssue, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;
        let m = match block_on(
            self.db
                .send(crate::db::issues::DeleteIssue { issue_id: msg.id }),
        ) {
            Ok(Ok(_)) => Some(WsMsg::IssueDeleted(msg.id)),
            _ => None,
        };
        Ok(m)
    }
}

pub struct LoadIssues;

impl WsHandler<LoadIssues> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadIssues, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user()?.project_id;

        let issues: Vec<jirs_data::Issue> =
            match block_on(self.db.send(LoadProjectIssues { project_id })) {
                Ok(Ok(v)) => v.into_iter().map(|i| i.into()).collect(),
                _ => return Ok(None),
            };
        let mut issue_map = HashMap::new();
        let mut queue = vec![];
        for issue in issues.into_iter() {
            let f = self.db.send(LoadAssignees { issue_id: issue.id });
            queue.push(f);
            issue_map.insert(issue.id, issue);
        }
        for f in queue {
            if let Ok(Ok(assignees)) = block_on(f) {
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
}
