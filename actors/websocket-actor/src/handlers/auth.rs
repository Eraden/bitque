use {
    crate::{
        db_or_debug_and_return, mail_or_debug_and_return, WebSocketActor, WsHandler, WsResult,
    },
    actix::AsyncContext,
    database_actor::{
        authorize_user::AuthorizeUser,
        tokens::{CreateBindToken, FindBindToken},
        users::LookupUser,
    },
    futures::executor::block_on,
    jirs_data::{Token, WsMsg},
    mail_actor::welcome::Welcome,
};

pub struct Authenticate {
    pub name: String,
    pub email: String,
}

impl WsHandler<Authenticate> for WebSocketActor {
    fn handle_msg(&mut self, msg: Authenticate, _ctx: &mut Self::Context) -> WsResult {
        let Authenticate { name, email } = msg;
        // TODO check attempt number, allow only 5 times per day
        let user = db_or_debug_and_return!(self, LookupUser { name, email });
        let token = db_or_debug_and_return!(self, CreateBindToken { user_id: user.id });
        if let Some(bind_token) = token.bind_token.as_ref().cloned() {
            let _ = mail_or_debug_and_return!(
                self,
                Welcome {
                    bind_token,
                    email: user.email,
                }
            );
        }
        Ok(Some(WsMsg::AuthenticateSuccess))
    }
}

pub struct CheckAuthToken {
    pub token: uuid::Uuid,
}

impl WsHandler<CheckAuthToken> for WebSocketActor {
    fn handle_msg(&mut self, msg: CheckAuthToken, ctx: &mut Self::Context) -> WsResult {
        let user: jirs_data::User = db_or_debug_and_return!(
            self,
            AuthorizeUser {
                access_token: msg.token,
            },
            Ok(Some(WsMsg::AuthorizeLoaded(Err(
                "Invalid auth token".to_string()
            )))),
            Ok(Some(WsMsg::AuthorizeExpired))
        );
        self.current_user = Some(user.clone());
        self.current_user_project = self.load_user_project().ok();
        self.current_project = self.load_project().ok();

        block_on(self.join_channel(ctx.address().recipient()));
        Ok(Some(WsMsg::AuthorizeLoaded(Ok(user))))
    }
}

pub struct CheckBindToken {
    pub bind_token: uuid::Uuid,
}

impl WsHandler<CheckBindToken> for WebSocketActor {
    fn handle_msg(&mut self, msg: CheckBindToken, _ctx: &mut Self::Context) -> WsResult {
        let token: Token = db_or_debug_and_return!(
            self,
            FindBindToken {
                token: msg.bind_token,
            },
            Ok(Some(WsMsg::BindTokenBad)),
            Ok(None)
        );
        Ok(Some(WsMsg::BindTokenOk(token.access_token)))
    }
}
