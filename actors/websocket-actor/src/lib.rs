#[macro_use]
extern crate log;

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Recipient, StreamHandler};
use actix_web::web::{self, Data};
use actix_web::{get, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use database_actor::projects::LoadCurrentProject;
use database_actor::user_projects::CurrentUserProject;
use database_actor::DbExecutor;
use futures::executor::block_on;
use jirs_data::{Project, User, UserProject, WsMsg};
use log::*;
use mail_actor::MailExecutor;

use crate::handlers::*;
use crate::server::{InnerMsg, WsServer};

pub mod handlers;
pub mod prelude;
pub mod server;

pub type WsResult = std::result::Result<Option<WsMsg>, WsMsg>;

trait WsMessageSender {
    fn send_msg(&mut self, msg: &jirs_data::WsMsg);
}

struct WebSocketActor {
    db: Data<Addr<DbExecutor>>,
    mail: Data<Addr<MailExecutor>>,
    addr: Addr<WsServer>,
    hi: Data<Addr<highlight_actor::HighlightActor>>,
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

    fn handle(&mut self, msg: InnerMsg, ctx: &mut <Self as Actor>::Context) -> Self::Result {
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
            WsMsg::IssueUpdate(id, field_id, payload) => self.handle_msg(
                UpdateIssueHandler {
                    id,
                    field_id,
                    payload,
                },
                ctx,
            )?,
            WsMsg::IssueCreate(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::IssueDelete(id) => self.handle_msg(DeleteIssue { id }, ctx)?,
            WsMsg::IssueSyncListPosition(sync) => {
                self.handle_msg(SyncIssueListPosition(sync), ctx)?
            }
            WsMsg::ProjectIssuesLoad => self.handle_msg(LoadIssues, ctx)?,

            // issue statuses
            WsMsg::IssueStatusesLoad => self.handle_msg(LoadIssueStatuses, ctx)?,
            WsMsg::IssueStatusDelete(issue_status_id) => {
                self.handle_msg(DeleteIssueStatus { issue_status_id }, ctx)?
            }
            WsMsg::IssueStatusUpdate(issue_status_id, name, position) => self.handle_msg(
                UpdateIssueStatus {
                    issue_status_id,
                    position,
                    name,
                },
                ctx,
            )?,
            WsMsg::IssueStatusCreate(name, position) => {
                self.handle_msg(CreateIssueStatus { position, name }, ctx)?
            }

            // projects
            WsMsg::ProjectsLoad => self.handle_msg(LoadProjects, ctx)?,
            WsMsg::ProjectUpdateLoad(payload) => self.handle_msg(payload, ctx)?,

            // user projects
            WsMsg::UserProjectsLoad => self.handle_msg(LoadUserProjects, ctx)?,
            WsMsg::UserProjectSetCurrent(user_project_id) => self.handle_msg(
                SetCurrentUserProject {
                    id: user_project_id,
                },
                ctx,
            )?,

            // auth
            WsMsg::AuthorizeLoad(uuid) => self.handle_msg(CheckAuthToken { token: uuid }, ctx)?,
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
            WsMsg::ProjectUsersLoad => self.handle_msg(LoadProjectUsers, ctx)?,
            WsMsg::InvitedUserRemoveRequest(user_id) => {
                self.handle_msg(RemoveInvitedUser { user_id }, ctx)?
            }

            // user settings
            WsMsg::UserSettingSetEditorMode(mode) => {
                self.handle_msg(user_settings::SetTextEditorMode { mode }, ctx)?
            }

            // comments
            WsMsg::IssueCommentsLoad(issue_id) => {
                self.handle_msg(LoadIssueComments { issue_id }, ctx)?
            }
            WsMsg::CommentCreate(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::CommentUpdate(payload) => self.handle_msg(payload, ctx)?,
            WsMsg::CommentDelete(comment_id) => {
                self.handle_msg(DeleteComment { comment_id }, ctx)?
            }

            // invitations
            WsMsg::InvitationSendRequest { name, email, role } => {
                self.handle_msg(CreateInvitation { email, name, role }, ctx)?
            }
            WsMsg::InvitationListLoad => self.handle_msg(ListInvitation, ctx)?,
            WsMsg::InvitationAcceptRequest(invitation_token) => {
                self.handle_msg(AcceptInvitation { invitation_token }, ctx)?
            }
            WsMsg::InvitationRevokeRequest(id) => self.handle_msg(RevokeInvitation { id }, ctx)?,
            WsMsg::InvitedUsersLoad => self.handle_msg(LoadInvitedUsers, ctx)?,

            // users
            WsMsg::ProfileUpdate(email, name) => {
                self.handle_msg(ProfileUpdate { name, email }, ctx)?
            }

            // messages
            WsMsg::MessagesLoad => self.handle_msg(LoadMessages, ctx)?,
            WsMsg::MessageMarkSeen(id) => self.handle_msg(MarkMessageSeen { id }, ctx)?,

            // epics
            WsMsg::EpicsLoad => self.handle_msg(epics::LoadEpics, ctx)?,
            WsMsg::EpicCreate(name, description, description_html) => self.handle_msg(
                epics::CreateEpic {
                    name,
                    description,
                    description_html,
                },
                ctx,
            )?,
            WsMsg::EpicUpdateName(epic_id, name) => {
                self.handle_msg(epics::UpdateEpicName { epic_id, name }, ctx)?
            }
            WsMsg::EpicUpdateStartsAt(epic_id, starts_at) => {
                self.handle_msg(epics::UpdateEpicStartsAt { epic_id, starts_at }, ctx)?
            }
            WsMsg::EpicUpdateEndsAt(epic_id, ends_at) => {
                self.handle_msg(epics::UpdateEpicEndsAt { epic_id, ends_at }, ctx)?
            }
            WsMsg::EpicDelete(epic_id) => self.handle_msg(epics::DeleteEpic { epic_id }, ctx)?,
            WsMsg::EpicTransform(epic_id, issue_type) => self.handle_msg(
                epics::TransformEpic {
                    epic_id,
                    issue_type,
                },
                ctx,
            )?,

            // hi
            WsMsg::HighlightCode(lang, code) => {
                self.handle_msg(hi::HighlightCode(lang, code), ctx)?
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
        self.current_user.as_ref().ok_or(WsMsg::AuthorizeExpired)
    }

    fn require_user_project(&self) -> Result<&UserProject, WsMsg> {
        self.current_user_project
            .as_ref()
            .ok_or(WsMsg::AuthorizeExpired)
    }

    fn load_user_project(&self) -> Result<UserProject, WsMsg> {
        let user_id = self.require_user()?.id;
        match block_on(self.db.send(CurrentUserProject { user_id })) {
            Ok(Ok(user_project)) => Ok(user_project),
            Ok(Err(e)) => {
                error!("load_user_project encounter service error {:?}", e);
                Err(WsMsg::AuthorizeExpired)
            }
            Err(e) => {
                error!("load_user_project encounter mailbox error {}", e);
                Err(WsMsg::AuthorizeExpired)
            }
        }
    }

    fn load_project(&self) -> Result<Project, WsMsg> {
        let project_id = self.require_user_project()?.project_id;
        match block_on(self.db.send(LoadCurrentProject { project_id })) {
            Ok(Ok(project)) => Ok(project),
            Ok(Err(e)) => {
                error!("{:?}", e);
                Err(WsMsg::AuthorizeExpired)
            }
            Err(e) => {
                error!("{}", e);
                Err(WsMsg::AuthorizeExpired)
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut <Self as Actor>::Context,
    ) {
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

    fn finished(&mut self, ctx: &mut <Self as Actor>::Context) {
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

#[get("/ws/")]
pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<Addr<DbExecutor>>,
    mail: Data<Addr<MailExecutor>>,
    ws_server: Data<Addr<WsServer>>,
    hi: Data<Addr<highlight_actor::HighlightActor>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebSocketActor {
            db,
            mail,
            hi,
            current_user: None,
            current_user_project: None,
            current_project: None,
            addr: ws_server.get_ref().clone(),
        },
        &req,
        stream,
    )
}
