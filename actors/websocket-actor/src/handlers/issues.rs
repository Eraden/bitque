use jirs_data::{EpicId, IssueStatusId, ListPosition};
use {
    crate::{WebSocketActor, WsHandler, WsResult},
    database_actor::{
        issue_assignees::LoadAssignees,
        issues::{LoadProjectIssues, UpdateIssue},
    },
    futures::executor::block_on,
    jirs_data::{CreateIssuePayload, IssueAssignee, IssueFieldId, IssueId, PayloadVariant, WsMsg},
    std::collections::HashMap,
};

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

        let mut msg = UpdateIssue {
            issue_id: id,
            title: None,
            issue_type: None,
            priority: None,
            list_position: None,
            description: None,
            description_text: None,
            estimate: None,
            time_spent: None,
            time_remaining: None,
            project_id: None,
            user_ids: None,
            reporter_id: None,
            issue_status_id: None,
            epic_id: None,
        };
        match (field_id, payload) {
            (IssueFieldId::Type, PayloadVariant::IssueType(t)) => {
                msg.issue_type = Some(t);
            }
            (IssueFieldId::Title, PayloadVariant::String(s)) => {
                msg.title = Some(s);
            }
            (IssueFieldId::Description, PayloadVariant::String(s)) => {
                let html: String = {
                    use pulldown_cmark::*;
                    let parser = pulldown_cmark::Parser::new(s.as_str());
                    enum ParseState {
                        Code(highlight_actor::TextHighlightCode),
                        Other,
                    }
                    let mut state = ParseState::Other;

                    let parser = parser.flat_map(|event| match event {
                        Event::Text(s) => {
                            if let ParseState::Code(h) = &mut state {
                                h.code.push_str(s.as_ref());
                                return vec![];
                            }
                            vec![Event::Text(s)]
                        }
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(name))) => {
                            state = ParseState::Code(highlight_actor::TextHighlightCode {
                                lang: name.to_string(),
                                code: String::new(),
                            });
                            vec![Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(name)))]
                        }
                        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                            let ev = if let ParseState::Code(h) = &mut state {
                                let mut msg = highlight_actor::TextHighlightCode {
                                    code: String::new(),
                                    lang: String::new(),
                                };
                                std::mem::swap(h, &mut msg);
                                let highlighted =
                                    match futures::executor::block_on(self.hi.send(msg)) {
                                        Ok(Ok(res)) => res,
                                        _ => s.to_string(),
                                    };
                                vec![
                                    Event::Html(highlighted.into()),
                                    Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))),
                                ]
                            } else {
                                vec![]
                            };
                            state = ParseState::Other;
                            ev
                        }
                        _ => vec![event],
                    });
                    let mut buff = String::new();
                    let _ = html::push_html(&mut buff, parser);
                    buff
                };
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

        let mut issue: jirs_data::Issue = match block_on(self.db.send(msg)) {
            Ok(Ok(issue)) => issue.into(),
            _ => return Ok(None),
        };

        let assignees: Vec<IssueAssignee> =
            match block_on(self.db.send(LoadAssignees { issue_id: issue.id })) {
                Ok(Ok(v)) => v,
                Ok(Err(e)) => {
                    error!("{:?}", e);
                    return Ok(None);
                }
                Err(e) => {
                    error!("{:?}", e);
                    return Ok(None);
                }
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
        let msg = database_actor::issues::CreateIssue {
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
            epic_id: msg.epic_id,
        };
        let m = match block_on(self.db.send(msg)) {
            Ok(Ok(issue)) => Some(WsMsg::IssueCreated(issue.into())),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{:?}", e);
                return Ok(None);
            }
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
                .send(database_actor::issues::DeleteIssue { issue_id: msg.id }),
        ) {
            Ok(Ok(n)) => Some(WsMsg::IssueDeleted(msg.id, n)),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{:?}", e);
                return Ok(None);
            }
        };
        Ok(m)
    }
}

pub struct LoadIssues;

impl WsHandler<LoadIssues> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadIssues, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

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

pub struct SyncIssueListPosition(pub Vec<(IssueId, ListPosition, IssueStatusId, Option<EpicId>)>);

impl WsHandler<SyncIssueListPosition> for WebSocketActor {
    fn handle_msg(&mut self, msg: SyncIssueListPosition, ctx: &mut Self::Context) -> WsResult {
        let _project_id = self.require_user_project()?.project_id;
        for (issue_id, list_position, status_id, epic_id) in msg.0 {
            match block_on(self.db.send(database_actor::issues::UpdateIssue {
                issue_id,
                list_position: Some(list_position),
                issue_status_id: Some(status_id),
                epic_id: Some(epic_id),
                ..Default::default()
            })) {
                Ok(Ok(_)) => (),
                _ => return Ok(None),
            };
        }

        self.handle_msg(LoadIssues, ctx)
    }
}
