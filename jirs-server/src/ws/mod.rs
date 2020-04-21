use std::collections::{HashMap, HashSet};

use actix::{
    Actor, ActorContext, Addr, AsyncContext, Context, Handler, Message, Recipient, StreamHandler,
};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::executor::block_on;
use futures::SinkExt;

use jirs_data::{ProjectId, Token, UserId, WsMsg};

use crate::db::authorize_user::AuthorizeUser;
use crate::db::tokens::FindBindToken;
use crate::db::DbExecutor;
use crate::mail::MailExecutor;

pub mod auth;
pub mod comments;
pub mod invitations;
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
    fn send_msg(&mut self, msg: &jirs_data::WsMsg);
}

struct WebSocketActor {
    db: Data<Addr<DbExecutor>>,
    mail: Data<Addr<MailExecutor>>,
    current_user: Option<jirs_data::User>,
    addr: Addr<WsServer>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<WebSocketActor>;
}

impl WsMessageSender for ws::WebsocketContext<WebSocketActor> {
    fn send_msg(&mut self, msg: &WsMsg) {
        self.binary(bincode::serialize(msg).unwrap())
    }
}

impl Handler<InnerMsg> for WebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: InnerMsg, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InnerMsg::Transfer(msg) => ctx.send_msg(&msg),
            _ => {}
        };
    }
}

impl WebSocketActor {
    fn broadcast(&self, msg: &WsMsg) {
        let user = match self.current_user.as_ref() {
            Some(u) => u,
            _ => return,
        };
        self.addr
            .do_send(InnerMsg::BroadcastToChannel(user.project_id, msg.clone()));
    }

    fn handle_ws_msg(
        &mut self,
        msg: WsMsg,
        ctx: &mut <WebSocketActor as Actor>::Context,
    ) -> WsResult {
        if msg != WsMsg::Ping && msg != WsMsg::Pong {
            debug!("incoming message: {:?}", msg);
        }

        let msg = match msg {
            WsMsg::Ping => Some(WsMsg::Pong),
            WsMsg::Pong => Some(WsMsg::Ping),

            // Issues
            WsMsg::IssueUpdateRequest(id, field_id, payload) => match block_on(
                issues::update_issue(&self.db, &self.current_user, id, field_id, payload),
            ) {
                Ok(Some(msg)) => {
                    self.broadcast(&msg);
                    None
                }
                _ => None,
            },
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
            WsMsg::AuthorizeRequest(uuid) => block_on(self.check_auth_token(uuid, ctx))?,
            WsMsg::BindTokenCheck(uuid) => block_on(self.check_bind_token(uuid))?,
            WsMsg::AuthenticateRequest(email, name) => {
                block_on(auth::authenticate(&self.db, &self.mail, name, email))?
            }

            // register
            WsMsg::SignUpRequest(email, username) => {
                block_on(users::register(&self.db, &self.mail, username, email))?
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

            WsMsg::UpdateComment(payload) => match block_on(comments::update_comment(
                &self.db,
                &self.current_user,
                payload,
            )) {
                Ok(Some(msg)) => {
                    self.broadcast(&msg);
                    None
                }
                _ => None,
            },

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

    async fn check_auth_token(
        &mut self,
        token: uuid::Uuid,
        ctx: &mut <WebSocketActor as Actor>::Context,
    ) -> WsResult {
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
                self.join_channel(ctx.address().recipient()).await;
                Some(WsMsg::AuthorizeLoaded(Ok(user)))
            }
            Ok(Err(_)) => Some(WsMsg::AuthorizeLoaded(
                Err("Invalid auth token".to_string()),
            )),
            _ => Some(WsMsg::AuthorizeExpired),
        };
        Ok(m)
    }

    async fn check_bind_token(&mut self, bind_token: uuid::Uuid) -> WsResult {
        let token: Token = match self.db.send(FindBindToken { token: bind_token }).await {
            Ok(Ok(token)) => token,
            Ok(Err(_)) => return Ok(Some(WsMsg::BindTokenBad)),
            _ => return Ok(None),
        };
        Ok(Some(WsMsg::BindTokenOk(token.access_token)))
    }

    async fn join_channel(&self, addr: Recipient<InnerMsg>) {
        info!("joining channel...");
        info!("  current user {:?}", self.current_user);
        let user = match self.current_user.as_ref() {
            None => return,
            Some(u) => u,
        };
        match self
            .addr
            .send(InnerMsg::Join(user.project_id, user.id, addr))
            .await
        {
            Err(e) => error!("{}", e),
            _ => info!("  joined channel"),
        };
    }

    fn require_user(&self) -> Result<&jirs_data::User, WsMsg> {
        current_user(&self.current_user)
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
                match self.handle_ws_msg(msg, ctx) {
                    Ok(Some(msg)) => ctx.send_msg(&msg),
                    Err(e) => ctx.send_msg(&e),
                    _ => (),
                };
            }
            _ => (),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        info!("Disconnected");
        if let Some(user) = self.current_user.as_ref() {
            self.addr.do_send(InnerMsg::Leave(user.project_id, user.id));
        }
        ctx.stop()
    }
}

impl WebSocketActor {
    fn try_handle_message(
        &mut self,
        msg: WsMsg,
        _ctx: &mut <WebSocketActor as Actor>::Context,
    ) -> WsResult
    where
        Self: Actor,
    {
        match msg {
            WsMsg::InvitationSendRequest { name, email } => {
                use invitations::*;

                let m = CreateInvitation {
                    name: name.clone(),
                    email: email.clone(),
                };
                // Handler::handle(&mut self, m, _ctx);
                Ok(None)
                // Handler::<CreateInvitation>::handle(&mut self, m, _ctx)
                // <self as Handler<CreateInvitation>>.handle(m, ctx)
            }
            //     WsMsg::InvitationListRequest => self.handle(ListInvitation, ctx),
            //     WsMsg::InvitationAcceptRequest(id) => Ok(None),
            //     WsMsg::InvitationRevokeRequest(id) => self.handle(RevokeInvitation { id: *id }, ctx),
            //     WsMsg::InvitedUsersRequest => Ok(None),
            _ => Ok(None),
        }
    }
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub enum InnerMsg {
    Join(ProjectId, UserId, Recipient<InnerMsg>),
    Leave(ProjectId, UserId),
    BroadcastToChannel(ProjectId, WsMsg),
    Transfer(WsMsg),
}

pub struct WsServer {
    sessions: HashMap<i32, Recipient<InnerMsg>>,
    rooms: HashMap<i32, HashSet<i32>>,
}

impl Default for WsServer {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Message for WsServer {
    type Result = ();
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl Handler<InnerMsg> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: InnerMsg, _ctx: &mut Self::Context) -> Self::Result {
        debug!("receive {:?}", msg);
        match msg {
            InnerMsg::Join(project_id, user_id, recipient) => {
                self.sessions.insert(user_id, recipient);
                self.ensure_room(project_id);
                if let Some(room) = self.rooms.get_mut(&project_id) {
                    room.insert(user_id);
                }
            }
            InnerMsg::Leave(project_id, user_id) => {
                self.ensure_room(project_id);
                if let Some(room) = self.rooms.get_mut(&project_id) {
                    room.remove(&user_id);
                }
                self.sessions.remove(&user_id);
            }
            InnerMsg::BroadcastToChannel(project_id, msg) => {
                debug!("Begin broadcast to channel {} msg {:?}", project_id, msg);
                let set = match self.rooms.get(&project_id) {
                    Some(s) => s,
                    _ => return debug!("  channel not found, aborting..."),
                };
                for r in set {
                    let recipient = match self.sessions.get(r) {
                        Some(r) => r,
                        _ => {
                            debug!("recipient is dead, skipping...");
                            continue;
                        }
                    };
                    match recipient.do_send(InnerMsg::Transfer(msg.clone())) {
                        Ok(_) => debug!("msg sent"),
                        Err(e) => error!("{}", e),
                    }
                }
            }
            _ => (),
        }
    }
}

impl WsServer {
    pub fn ensure_room(&mut self, room: i32) {
        if !self.rooms.contains_key(&room) {
            self.rooms.insert(room, HashSet::new());
        }
    }
}

#[get("/ws/")]
pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<Addr<DbExecutor>>,
    mail: Data<Addr<MailExecutor>>,
    ws_server: Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebSocketActor {
            db,
            mail,
            current_user: None,
            addr: ws_server.get_ref().clone(),
        },
        &req,
        stream,
    )
}
