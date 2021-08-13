use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Recipient, StreamHandler};
use actix_web::web::{self, Data};
use actix_web::{get, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use common::log::*;
use common::{actix_web, actix_web_actors};
use database_actor::projects::LoadCurrentProject;
use database_actor::user_projects::CurrentUserProject;
use database_actor::DbExecutor;
use futures::executor::block_on as wait;
use jirs_data::msg::WsMsgInvitation;
use jirs_data::{Project, User, UserProject, WsMsg};
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

pub struct WebSocketActor {
    db: Data<Addr<DbExecutor>>,
    mail: Data<Addr<MailExecutor>>,
    addr: Addr<WsServer>,
    hi: Data<Addr<highlight_actor::HighlightActor>>,
    current_user: Option<jirs_data::User>,
    current_user_project: Option<jirs_data::UserProject>,
    current_project: Option<jirs_data::Project>,
}

pub type WsCtx = ws::WebsocketContext<WebSocketActor>;

impl actix::Actor for WebSocketActor {
    type Context = WsCtx;
}

impl WsMessageSender for ws::WebsocketContext<WebSocketActor> {
    fn send_msg(&mut self, msg: &WsMsg) {
        match bincode::serialize(msg) {
            Err(err) => {
                common::log::error!("{}", err);
            }
            Ok(v) => self.binary(v),
        }
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

        match msg {
            WsMsg::Ping => return Ok(Some(WsMsg::Pong)),
            WsMsg::Pong => return Ok(Some(WsMsg::Ping)),
            WsMsg::AuthorizeLoad(uuid) => {
                return Ok(self.handle_msg(CheckAuthToken { token: uuid }, ctx)?)
            }
            WsMsg::Invitation(WsMsgInvitation::InvitationAcceptRequest(invitation_token)) => {
                return Ok(self.handle_msg(AcceptInvitation { invitation_token }, ctx)?)
            }
            _ => {}
        };

        let fut = match msg {
            WsMsg::Project(m) => self.exec(m),
            WsMsg::Issue(m) => self.exec(m),
            WsMsg::IssueStatus(m) => self.exec(m),
            WsMsg::Comment(m) => self.exec(m),
            WsMsg::Invitation(m) => self.exec(m),
            WsMsg::Epic(m) => self.exec(m),

            // user projects
            WsMsg::UserProjectsLoad => self.exec(LoadUserProjects),
            WsMsg::UserProjectSetCurrent(user_project_id) => self.exec(SetCurrentUserProject {
                id: user_project_id,
            }),

            // auth
            WsMsg::BindTokenCheck(uuid) => self.exec(CheckBindToken { bind_token: uuid }),
            WsMsg::AuthenticateRequest(email, name) => self.exec(Authenticate { name, email }),

            // register
            WsMsg::SignUpRequest(email, username) => self.exec(Register {
                name: username,
                email,
            }),

            // user settings
            WsMsg::UserSettingSetEditorMode(mode) => {
                self.exec(user_settings::SetTextEditorMode { mode })
            }

            // users
            WsMsg::ProfileUpdate(email, name) => self.exec(ProfileUpdate { name, email }),

            // messages
            WsMsg::MessagesLoad => self.exec(LoadMessages),
            WsMsg::MessageMarkSeen(id) => self.exec(MarkMessageSeen { id }),

            // hi
            WsMsg::HighlightCode(lang, code) => self.exec(hi::HighlightCode(lang, code)),

            // else fail
            _ => {
                error!("No handle for {:?} specified", msg);
                return Ok(None);
            }
        };
        let msg = wait(fut)?;

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
            Err(e) => common::log::error!("{:?}", e),
            _ => common::log::info!("  joined channel"),
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
        match wait(self.db.send(CurrentUserProject { user_id })) {
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
        match wait(self.db.send(LoadCurrentProject { project_id })) {
            Ok(Ok(project)) => Ok(project),
            Ok(Err(e)) => {
                error!("{:?}", e);
                Err(WsMsg::AuthorizeExpired)
            }
            Err(e) => {
                error!("{:?}", e);
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
                ctx.address().recipient::<InnerMsg>(),
            ));
        }
        ctx.stop()
    }
}

pub trait WsHandler<Message>
where
    Self: actix::Actor<Context = WsCtx>,
{
    fn handle_msg(&mut self, msg: Message, _ctx: &mut <Self as Actor>::Context) -> WsResult;
}

#[async_trait::async_trait]
pub trait AsyncHandler<Message>
where
    Self: actix::Actor<Context = WsCtx>,
{
    async fn exec(&mut self, msg: Message) -> WsResult;
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
