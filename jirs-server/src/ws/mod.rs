use actix::{Actor, Addr, StreamHandler};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use jirs_data::WsMsg;

use crate::db::authorize_user::AuthorizeUser;
use crate::db::DbExecutor;

pub mod comments;
pub mod issues;
pub mod projects;
pub mod users;

pub type WsResult = std::result::Result<Option<WsMsg>, WsMsg>;

pub fn current_user(current_user: &Option<jirs_data::User>) -> Result<&jirs_data::User, WsMsg> {
    current_user
        .as_ref()
        .map(|u| u)
        .ok_or_else(|| WsMsg::AuthorizeExpired)
}

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

            // Issues
            WsMsg::IssueUpdateRequest(id, payload) => block_on(issues::update_issue(
                &self.db,
                &self.current_user,
                id,
                payload,
            ))?,
            WsMsg::IssueCreateRequest(payload) => {
                block_on(issues::add_issue(&self.db, &self.current_user, payload))?
            }
            WsMsg::IssueDeleteRequest(id) => {
                block_on(issues::delete_issue(&self.db, &self.current_user, id))?
            }
            WsMsg::ProjectIssuesRequest => {
                block_on(issues::load_issues(&self.db, &self.current_user))?
            }

            // projects
            WsMsg::ProjectRequest => {
                block_on(projects::current_project(&self.db, &self.current_user))?
            }

            WsMsg::ProjectUpdateRequest(payload) => block_on(projects::update_project(
                &self.db,
                &self.current_user,
                payload,
            ))?,

            // auth
            WsMsg::AuthorizeRequest(uuid) => block_on(self.authorize(uuid))?,
            WsMsg::AuthenticateRequest(email, name) => {
                block_on(users::authenticate(&self.db, name, email))?
            }

            // users
            WsMsg::ProjectUsersRequest => {
                block_on(users::load_project_users(&self.db, &self.current_user))?
            }

            // comments
            WsMsg::IssueCommentsRequest(issue_id) => block_on(comments::load_issues(
                &self.db,
                &self.current_user,
                issue_id,
            ))?,

            WsMsg::CreateComment(payload) => block_on(comments::create_comment(
                &self.db,
                &self.current_user,
                payload,
            ))?,

            WsMsg::UpdateComment(payload) => block_on(comments::update_comment(
                &self.db,
                &self.current_user,
                payload,
            ))?,

            WsMsg::CommentDeleteRequest(comment_id) => block_on(comments::delete_comment(
                &self.db,
                &self.current_user,
                comment_id,
            ))?,

            // else fail
            _ => {
                error!("No handle for {:?} specified", msg);
                None
            }
        };
        if msg.is_some() && msg != Some(WsMsg::Pong) {
            info!("sending message {:?}", msg);
        }
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
