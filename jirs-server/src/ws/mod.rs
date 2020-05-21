use std::collections::HashMap;

use actix::{
    Actor, ActorContext, Addr, AsyncContext, Context, Handler, Message, Recipient, StreamHandler,
};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::executor::block_on;

use jirs_data::{Project, ProjectId, User, UserId, UserProject, WsMsg};

use crate::db::projects::LoadCurrentProject;
use crate::db::user_projects::CurrentUserProject;
use crate::db::DbExecutor;
use crate::mail::MailExecutor;
use crate::ws::auth::*;
use crate::ws::comments::*;
use crate::ws::invitations::*;
use crate::ws::issue_statuses::*;
use crate::ws::issues::*;
use crate::ws::projects::*;
use crate::ws::user_projects::LoadUserProjects;
use crate::ws::users::*;

pub mod auth;
pub mod comments;
pub mod invitations;
pub mod issue_statuses;
pub mod issues;
pub mod projects;
pub mod user_projects;
pub mod users;

pub type WsResult = std::result::Result<Option<WsMsg>, WsMsg>;

trait WsMessageSender {
    fn send_msg(&mut self, msg: &jirs_data::WsMsg);
}

struct WebSocketActor {
    db: Data<Addr<DbExecutor>>,
    mail: Data<Addr<MailExecutor>>,
    addr: Addr<WsServer>,
    current_user: Option<jirs_data::User>,
    current_user_project: Option<jirs_data::UserProject>,
    current_project: Option<jirs_data::Project>,
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
        if let InnerMsg::Transfer(msg) = msg {
            ctx.send_msg(&msg)
        };
    }
}

impl WebSocketActor {
    fn broadcast(&self, msg: &WsMsg) {
        let project_id = match self.require_user_project() {
            Ok(up) => up.project_id,
            _ => return,
        };
        self.addr
            .do_send(InnerMsg::BroadcastToChannel(project_id, msg.clone()));
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

            // issues
            WsMsg::IssueUpdateRequest(id, field_id, payload) => self.handle_msg(
                UpdateIssueHandler {
                    id,
                    field_id,
                    payload,
                },
                ctx,
            )?,
            WsMsg::IssueCreateRequest(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::IssueDeleteRequest(id) => self.handle_msg(DeleteIssue { id }, ctx)?,
            WsMsg::ProjectIssuesRequest => self.handle_msg(LoadIssues, ctx)?,

            // issue statuses
            WsMsg::IssueStatusesRequest => self.handle_msg(LoadIssueStatuses, ctx)?,
            WsMsg::IssueStatusDelete(issue_status_id) => {
                self.handle_msg(DeleteIssueStatus { issue_status_id }, ctx)?
            }
            WsMsg::IssueStatusUpdate(issue_status_id, name, position) => self.handle_msg(
                UpdateIssueStatus {
                    issue_status_id,
                    name,
                    position,
                },
                ctx,
            )?,
            WsMsg::IssueStatusCreate(name, position) => {
                self.handle_msg(CreateIssueStatus { name, position }, ctx)?
            }

            // projects
            WsMsg::ProjectRequest => self.handle_msg(CurrentProject, ctx)?,
            WsMsg::ProjectsLoad => self.handle_msg(LoadProjects, ctx)?,
            WsMsg::ProjectUpdateRequest(payload) => self.handle_msg(payload, ctx)?,

            // user projects
            WsMsg::UserProjectLoad => self.handle_msg(LoadUserProjects, ctx)?,

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
                Register {
                    name: username,
                    email,
                },
                ctx,
            )?,

            // users
            WsMsg::ProjectUsersRequest => self.handle_msg(LoadProjectUsers, ctx)?,

            // comments
            WsMsg::IssueCommentsRequest(issue_id) => {
                self.handle_msg(LoadIssueComments { issue_id }, ctx)?
            }
            WsMsg::CreateComment(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::UpdateComment(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::CommentDeleteRequest(comment_id) => {
                self.handle_msg(DeleteComment { comment_id }, ctx)?
            }

            // invitations
            WsMsg::InvitationSendRequest { name, email, role } => {
                self.handle_msg(CreateInvitation { name, email, role }, ctx)?
            }
            WsMsg::InvitationListRequest => self.handle_msg(ListInvitation, ctx)?,
            WsMsg::InvitationAcceptRequest(id) => self.handle_msg(AcceptInvitation { id }, ctx)?,
            WsMsg::InvitationRevokeRequest(id) => self.handle_msg(RevokeInvitation { id }, ctx)?,
            WsMsg::InvitedUsersRequest => self.handle_msg(LoadInvitedUsers, ctx)?,

            // users
            WsMsg::ProfileUpdate(email, name) => {
                self.handle_msg(ProfileUpdate { email, name }, ctx)?
            }

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
        let project_id = match self.require_user_project() {
            Ok(user_project) => user_project.project_id,
            _ => return,
        };
        match self
            .addr
            .send(InnerMsg::Join(project_id, user.id, addr))
            .await
        {
            Err(e) => error!("{}", e),
            _ => info!("  joined channel"),
        };
    }

    fn require_user(&self) -> Result<&User, WsMsg> {
        self.current_user
            .as_ref()
            .map(|u| u)
            .ok_or_else(|| WsMsg::AuthorizeExpired)
    }

    fn require_user_project(&self) -> Result<&UserProject, WsMsg> {
        self.current_user_project
            .as_ref()
            .map(|u| u)
            .ok_or_else(|| WsMsg::AuthorizeExpired)
    }

    // fn require_project(&self) -> Result<&Project, WsMsg> {
    //     self.current_project
    //         .as_ref()
    //         .map(|u| u)
    //         .ok_or_else(|| WsMsg::AuthorizeExpired)
    // }

    fn load_user_project(&self) -> Result<UserProject, WsMsg> {
        let user_id = self.require_user().map_err(|_| WsMsg::AuthorizeExpired)?.id;
        match block_on(self.db.send(CurrentUserProject { user_id })) {
            Ok(Ok(user_project)) => Ok(user_project),
            _ => Err(WsMsg::AuthorizeExpired),
        }
    }

    fn load_project(&self) -> Result<Project, WsMsg> {
        let project_id = self.require_user_project()?.project_id;
        match block_on(self.db.send(LoadCurrentProject { project_id })) {
            Ok(Ok(project)) => Ok(project),
            _ => Err(WsMsg::AuthorizeExpired),
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
        if let (Some(user), Some(up)) = (
            self.current_user.as_ref(),
            self.current_user_project.as_ref(),
        ) {
            self.addr.do_send(InnerMsg::Leave(
                up.project_id,
                user.id,
                ctx.address().recipient(),
            ));
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
    Leave(ProjectId, UserId, Recipient<InnerMsg>),
    BroadcastToChannel(ProjectId, WsMsg),
    Transfer(WsMsg),
}

pub struct WsServer {
    sessions: HashMap<UserId, Vec<Recipient<InnerMsg>>>,
    rooms: HashMap<ProjectId, HashMap<UserId, i32>>,
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
                let v = self.sessions.entry(user_id).or_insert(vec![]);
                v.push(recipient);
                self.ensure_room(project_id);

                if let Some(room) = self.rooms.get_mut(&project_id) {
                    let n = *room.entry(user_id).or_insert(0);
                    room.insert(user_id, n + 1);
                }
            }
            InnerMsg::Leave(project_id, user_id, recipient) => {
                self.ensure_room(project_id);
                let room = match self.rooms.get_mut(&project_id) {
                    Some(room) => room,
                    None => return,
                };
                let n = *room.entry(user_id).or_insert(0);
                if n <= 1 {
                    room.remove(&user_id);
                    self.sessions.remove(&user_id);
                } else {
                    let v = self.sessions.entry(user_id).or_insert(vec![]);
                    v.remove_item(&recipient);
                }
            }
            InnerMsg::BroadcastToChannel(project_id, msg) => {
                debug!("Begin broadcast to channel {} msg {:?}", project_id, msg);
                let set = match self.rooms.get(&project_id) {
                    Some(s) => s,
                    _ => return debug!("  channel not found, aborting..."),
                };
                let _s = set.len();
                for r in set.keys() {
                    let v = match self.sessions.get(r) {
                        Some(v) => v,
                        _ => {
                            debug!("recipient is dead, skipping...");
                            continue;
                        }
                    };
                    for recipient in v.iter() {
                        match recipient.do_send(InnerMsg::Transfer(msg.clone())) {
                            Ok(_) => debug!("msg sent"),
                            Err(e) => error!("{}", e),
                        };
                    }
                }
            }
            _ => (),
        }
    }
}

impl WsServer {
    pub fn ensure_room(&mut self, room: i32) {
        self.rooms.entry(room).or_insert_with(HashMap::new);
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
            current_user_project: None,
            current_project: None,
            addr: ws_server.get_ref().clone(),
        },
        &req,
        stream,
    )
}
