use actix::{Actor, Addr, StreamHandler};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use jirs_data::{Project, WsMsg};

use crate::db::authorize_user::AuthorizeUser;
use crate::db::issues::{LoadProjectIssues, UpdateIssue};
use crate::db::projects::LoadCurrentProject;
use crate::db::users::LoadProjectUsers;
use crate::db::DbExecutor;

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
    fn handle_msg(&mut self, msg: WsMsg) -> Option<WsMsg> {
        use futures::executor::block_on;

        match msg {
            WsMsg::Ping => Some(WsMsg::Pong),
            WsMsg::Pong => Some(WsMsg::Ping),
            WsMsg::ProjectRequest => match block_on(self.load_project()) {
                Some(p) => Some(WsMsg::ProjectLoaded(p)),
                _ => {
                    error!("Failed to load project");
                    None
                }
            },
            WsMsg::AuthorizeRequest(uuid) => match block_on(self.authorize(uuid)) {
                Some(user) => {
                    self.current_user = Some(user.clone());
                    Some(WsMsg::AuthorizeLoaded(Ok(user)))
                }
                _ => Some(WsMsg::AuthorizeLoaded(
                    Err("Invalid auth token".to_string()),
                )),
            },
            WsMsg::ProjectIssuesRequest => block_on(self.load_issues()),
            WsMsg::ProjectUsersRequest => block_on(self.load_project_users()),
            WsMsg::IssueUpdateRequest(id, payload) => block_on(self.update_issue(id, payload)),
            _ => {
                error!("Failed to resolve message");
                None
            }
        }
    }

    async fn authorize(&mut self, token: uuid::Uuid) -> Option<jirs_data::User> {
        match self
            .db
            .send(AuthorizeUser {
                access_token: token,
            })
            .await
        {
            Ok(Ok(u)) => Some(u.into()),
            _ => None,
        }
    }

    async fn load_project(&mut self) -> Option<Project> {
        let project_id = self.current_user.as_ref().map(|u| u.project_id)?;
        match self.db.send(LoadCurrentProject { project_id }).await {
            Ok(Ok(p)) => Some(p.into()),
            Ok(e) => {
                error!("{:?}", e);
                None
            }
            Err(e) => {
                error!("{:?}", e);
                None
            }
        }
    }

    async fn load_issues(&mut self) -> Option<WsMsg> {
        let project_id = self.current_user.as_ref().map(|u| u.project_id)?;
        match self.db.send(LoadProjectIssues { project_id }).await {
            Ok(Ok(v)) => Some(WsMsg::ProjectIssuesLoaded(
                v.into_iter().map(|i| i.into()).collect(),
            )),
            _ => None,
        }
    }

    async fn load_project_users(&mut self) -> Option<WsMsg> {
        let project_id = self.current_user.as_ref().map(|u| u.project_id)?;
        match self.db.send(LoadProjectUsers { project_id }).await {
            Ok(Ok(v)) => Some(WsMsg::ProjectUsersLoaded(
                v.into_iter().map(|i| i.into()).collect(),
            )),
            _ => None,
        }
    }

    async fn update_issue(
        &mut self,
        issue_id: i32,
        payload: jirs_data::UpdateIssuePayload,
    ) -> Option<WsMsg> {
        let jirs_data::UpdateIssuePayload {
            title,
            issue_type,
            status,
            priority,
            list_position,
            description,
            description_text,
            estimate,
            time_spent,
            time_remaining,
            project_id,
            user_ids,
        } = payload;
        match self
            .db
            .send(UpdateIssue {
                issue_id,
                title,
                issue_type,
                status,
                priority,
                list_position,
                description,
                description_text,
                estimate,
                time_spent,
                time_remaining,
                project_id,
                user_ids,
            })
            .await
        {
            Ok(Ok(issue)) => Some(WsMsg::IssueUpdated(issue.into())),
            _ => None,
        }
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
                    Ok(msg) => msg,
                    _ => return,
                };
                match self.handle_msg(msg) {
                    Some(msg) => ctx.send_msg(msg),
                    _ => return,
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
