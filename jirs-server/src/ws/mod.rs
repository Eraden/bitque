use std::collections::HashMap;

use actix::{Actor, Addr, StreamHandler};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use jirs_data::WsMsg;

use crate::db::authorize_user::AuthorizeUser;
use crate::db::issue_assignees::LoadAssignees;
use crate::db::issues::{LoadProjectIssues, UpdateIssue};
use crate::db::projects::LoadCurrentProject;
use crate::db::users::LoadProjectUsers;
use crate::db::DbExecutor;

type WsResult = std::result::Result<Option<WsMsg>, WsMsg>;

trait WsMessageSender {
    fn send_msg(&mut self, msg: jirs_data::WsMsg);
}

struct WebSocketActor {
    db: Data<Addr<DbExecutor>>,
    current_user: Option<jirs_data::User>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<WebSocketActor>;
}

impl WsMessageSender for ws::WebsocketContext<WebSocketActor> {
    fn send_msg(&mut self, msg: WsMsg) {
        self.binary(bincode::serialize(&msg).unwrap())
    }
}

impl WebSocketActor {
    fn handle_ws_msg(&mut self, msg: WsMsg) -> WsResult {
        use futures::executor::block_on;

        if msg != WsMsg::Ping && msg != WsMsg::Pong {
            info!("incoming message: {:?}", msg);
        }

        let msg = match msg {
            WsMsg::Ping => Some(WsMsg::Pong),
            WsMsg::Pong => Some(WsMsg::Ping),
            WsMsg::IssueUpdateRequest(id, payload) => block_on(self.update_issue(id, payload))?,
            WsMsg::IssueCreateRequest(payload) => block_on(self.add_issue(payload))?,
            WsMsg::IssueDeleteRequest(id) => block_on(self.delete_issue(id))?,
            WsMsg::ProjectRequest => block_on(self.load_project())?,
            WsMsg::AuthorizeRequest(uuid) => block_on(self.authorize(uuid))?,
            WsMsg::ProjectIssuesRequest => block_on(self.load_issues())?,
            WsMsg::ProjectUsersRequest => block_on(self.load_project_users())?,
            _ => {
                error!("No handle for {:?} specified", msg);
                None
            }
        };
        Ok(msg)
    }

    async fn authorize(&mut self, token: uuid::Uuid) -> WsResult {
        let m = match self
            .db
            .send(AuthorizeUser {
                access_token: token,
            })
            .await
        {
            Ok(Ok(u)) => {
                let user: jirs_data::User = u.into();
                self.current_user = Some(user.clone());
                Some(WsMsg::AuthorizeLoaded(Ok(user)))
            }
            Ok(Err(_)) => Some(WsMsg::AuthorizeLoaded(
                Err("Invalid auth token".to_string()),
            )),
            _ => Some(WsMsg::AuthorizeExpired),
        };
        Ok(m)
    }

    fn current_user(&mut self) -> Result<&jirs_data::User, WsMsg> {
        self.current_user
            .as_ref()
            .map(|u| u)
            .ok_or_else(|| WsMsg::AuthorizeExpired)
    }

    async fn load_project(&mut self) -> WsResult {
        let project_id = self.current_user().map(|u| u.project_id)?;
        match self.db.send(LoadCurrentProject { project_id }).await {
            Ok(Ok(p)) => Ok(Some(WsMsg::ProjectLoaded(p.into()))),
            _ => Ok(None),
        }
    }

    async fn load_issues(&mut self) -> WsResult {
        let project_id = self.current_user().map(|u| u.project_id)?;

        let issues: Vec<jirs_data::Issue> =
            match self.db.send(LoadProjectIssues { project_id }).await {
                Ok(Ok(v)) => v.into_iter().map(|i| i.into()).collect(),
                _ => return Ok(None),
            };
        let mut issue_map = HashMap::new();
        let mut queue = vec![];
        for issue in issues.into_iter() {
            let f = self.db.send(LoadAssignees {
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

    async fn load_project_users(&mut self) -> WsResult {
        let project_id = self.current_user().map(|u| u.project_id)?;
        let m = match self.db.send(LoadProjectUsers { project_id }).await {
            Ok(Ok(v)) => Some(WsMsg::ProjectUsersLoaded(
                v.into_iter().map(|i| i.into()).collect(),
            )),
            _ => None,
        };
        Ok(m)
    }

    async fn update_issue(
        &mut self,
        issue_id: i32,
        payload: jirs_data::UpdateIssuePayload,
    ) -> WsResult {
        self.current_user()?;
        let mut issue: jirs_data::Issue = match self
            .db
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

        let assignees = match self
            .db
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

    async fn add_issue(&mut self, payload: jirs_data::CreateIssuePayload) -> WsResult {
        self.current_user()?;
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
        let m = match self.db.send(msg).await {
            Ok(Ok(issue)) => Some(WsMsg::IssueCreated(issue.into())),
            _ => None,
        };
        Ok(m)
    }

    async fn delete_issue(&mut self, id: i32) -> WsResult {
        self.current_user()?;
        let m = match self
            .db
            .send(crate::db::issues::DeleteIssue { issue_id: id })
            .await
        {
            Ok(Ok(_)) => Some(WsMsg::IssueDeleted(id)),
            _ => None,
        };
        Ok(m)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),

            Ok(ws::Message::Binary(bin)) => {
                let ws_msg: bincode::Result<jirs_data::WsMsg> =
                    bincode::deserialize(bin.to_vec().as_slice());
                let msg = match ws_msg {
                    Ok(m) => m,
                    _ => return,
                };
                match self.handle_ws_msg(msg) {
                    Ok(Some(msg)) => ctx.send_msg(msg),
                    Err(e) => ctx.send_msg(e),
                    _ => (),
                };
            }
            _ => (),
        }
    }
}

#[get("/ws/")]
pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<Addr<DbExecutor>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebSocketActor {
            db,
            current_user: None,
        },
        &req,
        stream,
    )
}
