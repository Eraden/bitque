use actix::{Actor, Addr, StreamHandler};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use jirs_data::WsMsg;

use crate::db::authorize_user::AuthorizeUser;
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
        // use futures::executor::LocalPool;
        use futures::executor::block_on;
        // let mut pool = LocalPool::new();
        // pool.run_until();

        if msg != WsMsg::Ping && msg != WsMsg::Pong {
            info!("(2)incoming message: {:?}", msg);
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
                error!("Failed to resolve message");
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
        match self.db.send(LoadProjectIssues { project_id }).await {
            Ok(Ok(v)) => Ok(Some(WsMsg::ProjectIssuesLoaded(
                v.into_iter().map(|i| i.into()).collect(),
            ))),
            _ => Ok(None),
        }
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
        let m = match self
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
        };
        Ok(m)
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
                if msg != WsMsg::Ping && msg != WsMsg::Pong {
                    info!("(1)incoming message: {:?}", msg);
                }
                let _x = 1;
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
