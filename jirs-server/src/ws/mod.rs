use std::collections::{HashMap, HashSet};

use actix::{Actor, ActorContext, Addr, Context, Handler, Message, Recipient, StreamHandler};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use jirs_data::{ProjectId, UserId, WsMsg};

use crate::db::DbExecutor;
use crate::mail::MailExecutor;
use crate::ws::auth::{Authenticate, CheckAuthToken, CheckBindToken};
use crate::ws::invitations::*;
use crate::ws::issues::UpdateIssueHandler;

pub mod auth;
pub mod comments;
pub mod invitations;
pub mod issues;
pub mod projects;
pub mod users;

pub type WsResult = std::result::Result<Option<WsMsg>, WsMsg>;

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
            WsMsg::IssueUpdateRequest(id, field_id, payload) => self.handle_msg(
                UpdateIssueHandler {
                    id,
                    field_id,
                    payload,
                },
                ctx,
            )?,
            WsMsg::IssueCreateRequest(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::IssueDeleteRequest(id) => self.handle_msg(issues::DeleteIssue { id }, ctx)?,
            WsMsg::ProjectIssuesRequest => self.handle_msg(issues::LoadIssues, ctx)?,

            // projects
            WsMsg::ProjectRequest => self.handle_msg(projects::CurrentProject, ctx)?,
            WsMsg::ProjectUpdateRequest(payload) => self.handle_msg(payload, ctx)?,

            // auth
            WsMsg::AuthorizeRequest(uuid) => {
                self.handle_msg(CheckAuthToken { token: uuid }, ctx)?
            }
            WsMsg::BindTokenCheck(uuid) => {
                self.handle_msg(CheckBindToken { bind_token: uuid }, ctx)?
            }
            WsMsg::AuthenticateRequest(email, name) => {
                self.handle_msg(Authenticate { name, email }, ctx)?
            }

            // register
            WsMsg::SignUpRequest(email, username) => self.handle_msg(
                users::Register {
                    name: username,
                    email,
                },
                ctx,
            )?,

            // users
            WsMsg::ProjectUsersRequest => self.handle_msg(users::LoadProjectUsers, ctx)?,

            // comments
            WsMsg::IssueCommentsRequest(issue_id) => {
                self.handle_msg(comments::LoadIssueComments { issue_id }, ctx)?
            }
            WsMsg::CreateComment(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::UpdateComment(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::CommentDeleteRequest(comment_id) => {
                self.handle_msg(comments::DeleteComment { comment_id }, ctx)?
            }

            // invitations
            WsMsg::InvitationSendRequest { name, email } => self.handle_msg(
                CreateInvitation {
                    name: name.clone(),
                    email: email.clone(),
                },
                ctx,
            )?,
            WsMsg::InvitationListRequest => self.handle_msg(ListInvitation, ctx)?,
            WsMsg::InvitationAcceptRequest(id) => self.handle_msg(AcceptInvitation { id }, ctx)?,

            WsMsg::InvitationRevokeRequest(id) => self.handle_msg(RevokeInvitation { id }, ctx)?,

            WsMsg::InvitedUsersRequest => None,

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
        self.current_user
            .as_ref()
            .map(|u| u)
            .ok_or_else(|| WsMsg::AuthorizeExpired)
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

pub trait WsHandler<Message>
where
    Self: Actor,
{
    fn handle_msg(&mut self, msg: Message, _ctx: &mut <Self as Actor>::Context) -> WsResult;
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
