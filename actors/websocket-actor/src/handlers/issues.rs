use std::collections::HashMap;

use database_actor::issue_assignees::LoadAssignees;
use database_actor::issues::{LoadProjectIssues, UpdateIssue};
use jirs_data::msg::{IssueSync, WsMsgIssue, WsMsgProject};
use jirs_data::{CreateIssuePayload, IssueAssignee, IssueFieldId, IssueId, PayloadVariant, WsMsg};

use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgIssue> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgIssue) -> WsResult {
        match msg {
            WsMsgIssue::IssueUpdate(id, field_id, payload) => {
                self.exec(UpdateIssueHandler {
                    id,
                    field_id,
                    payload,
                })
                .await
            }
            WsMsgIssue::IssueCreate(payload) => self.exec(payload).await,
            WsMsgIssue::IssueDelete(id) => self.exec(DeleteIssue { id }).await,
            WsMsgIssue::IssueSyncListPosition(sync) => self.exec(SyncIssueListPosition(sync)).await,

            WsMsgIssue::IssueUpdated(_) => Ok(None),
            WsMsgIssue::IssueDeleted(_, _) => Ok(None),
            WsMsgIssue::IssueCreated(_) => Ok(None),
            WsMsgIssue::IssueSyncedListPosition(_) => Ok(None),
        }
    }
}

pub struct UpdateIssueHandler {
    pub id: i32,
    pub field_id: IssueFieldId,
    pub payload: PayloadVariant,
}

#[async_trait::async_trait]
impl AsyncHandler<UpdateIssueHandler> for WebSocketActor {
    async fn exec(&mut self, msg: UpdateIssueHandler) -> WsResult {
        self.require_user()?;

        let UpdateIssueHandler {
            id,
            field_id,
            payload,
        } = msg;

        let mut msg = UpdateIssue {
            issue_id: id,
            ..Default::default()
        };
        match (field_id, payload) {
            (IssueFieldId::Type, PayloadVariant::IssueType(t)) => {
                msg.issue_type = Some(t);
            }
            (IssueFieldId::Title, PayloadVariant::String(s)) => {
                msg.title = Some(s);
            }
            (IssueFieldId::Description, PayloadVariant::String(s)) => {
                let opts = comrak::ComrakOptions::default();
                let hi = comrak::plugins::syntect::SyntectAdapter::new("InspiredGitHub");
                let mut plugins = comrak::ComrakPlugins::default();
                plugins.render.codefence_syntax_highlighter = Some(&hi);

                let html: String = comrak::markdown_to_html_with_plugins(&s, &opts, &plugins);
                msg.description = Some(html);
                msg.description_text = Some(s);
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
            (IssueFieldId::EpicName, PayloadVariant::OptionI32(o)) => {
                msg.epic_id = Some(o);
            }
            _ => (),
        };

        let issue = db_or_debug_and_return!(self, msg; async);
        let mut issue: jirs_data::Issue = issue.into();

        let assignees: Vec<IssueAssignee> =
            db_or_debug_and_return!(self, LoadAssignees { issue_id: issue.id }; async);

        for assignee in assignees {
            issue.user_ids.push(assignee.user_id);
        }
        self.broadcast(&WsMsg::Issue(WsMsgIssue::IssueUpdated(issue)));

        Ok(None)
    }
}

#[async_trait::async_trait]
impl AsyncHandler<CreateIssuePayload> for WebSocketActor {
    async fn exec(&mut self, msg: CreateIssuePayload) -> WsResult {
        self.require_user()?;
        let msg = database_actor::issues::CreateIssue {
            title: msg.title,
            issue_type: msg.issue_type,
            issue_status_id: msg.issue_status_id,
            priority: msg.priority,
            description: msg.description.clone(),
            description_text: msg.description_text.or(msg.description),
            estimate: msg.estimate,
            time_spent: msg.time_spent,
            time_remaining: msg.time_remaining,
            project_id: msg.project_id,
            reporter_id: msg.reporter_id,
            user_ids: msg.user_ids,
            epic_id: msg.epic_id,
        };
        let issue = db_or_debug_and_return!(self, msg; async);
        Ok(Some(WsMsg::Issue(WsMsgIssue::IssueCreated(issue.into()))))
    }
}

pub struct DeleteIssue {
    pub id: IssueId,
}

#[async_trait::async_trait]
impl AsyncHandler<DeleteIssue> for WebSocketActor {
    async fn exec(&mut self, msg: DeleteIssue) -> WsResult {
        self.require_user()?;
        let n = db_or_debug_and_return!(
            self,
            database_actor::issues::DeleteIssue { issue_id: msg.id }; async
        );
        Ok(Some(WsMsg::Issue(WsMsgIssue::IssueDeleted(msg.id, n))))
    }
}

pub struct LoadIssues;

#[async_trait::async_trait]
impl AsyncHandler<LoadIssues> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadIssues) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let v = db_or_debug_and_return!(self, LoadProjectIssues { project_id }; async);
        let issues: Vec<jirs_data::Issue> = v.into_iter().map(|i| i.into()).collect();
        let mut issue_map = HashMap::new();
        let mut queue = vec![];
        for issue in issues {
            let f = self.db.send(LoadAssignees { issue_id: issue.id });
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
        for (_, issue) in issue_map {
            issues.push(issue);
        }
        issues.sort_by(|a, b| a.list_position.cmp(&b.list_position));

        Ok(Some(WsMsg::Project(WsMsgProject::ProjectIssuesLoaded(
            issues,
        ))))
    }
}

pub struct SyncIssueListPosition(pub Vec<IssueSync>);

#[async_trait::async_trait]
impl AsyncHandler<SyncIssueListPosition> for WebSocketActor {
    async fn exec(&mut self, msg: SyncIssueListPosition) -> WsResult {
        let _project_id = self.require_user_project()?.project_id;
        let mut result = Vec::with_capacity(msg.0.len());
        for issue_sync in msg.0 {
            crate::actor_or_debug_and_ignore!(
                self,
                db,
                database_actor::issues::UpdateIssue {
                    issue_id: issue_sync.id,
                    list_position: Some(issue_sync.list_position),
                    issue_status_id: Some(issue_sync.issue_status_id),
                    epic_id: Some(issue_sync.epic_id),
                    ..Default::default()
                },
                |issue: database_actor::models::Issue| {
                    result.push(IssueSync {
                        id: issue.id,
                        list_position: issue.list_position,
                        issue_status_id: issue.issue_status_id,
                        epic_id: issue.epic_id,
                    });
                }; async
            );
        }

        Ok(Some(WsMsg::Issue(WsMsgIssue::IssueSyncedListPosition(
            result,
        ))))
    }
}
