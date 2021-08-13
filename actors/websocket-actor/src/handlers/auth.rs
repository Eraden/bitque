use actix::AsyncContext;
use database_actor::authorize_user::AuthorizeUser;
use database_actor::tokens::{CreateBindToken, FindBindToken};
use database_actor::users::LookupUser;
use futures::executor::block_on;
use jirs_data::msg::WsError;
use jirs_data::{Token, WsMsg};
use mail_actor::welcome::Welcome;

use crate::server::InnerMsg;
use crate::{db_or_debug_and_return, db_or_debug_or_fallback, mail_or_debug_and_return, *};

pub struct Authenticate {
    pub name: String,
    pub email: String,
}

#[async_trait::async_trait]
impl AsyncHandler<Authenticate> for WebSocketActor {
    async fn exec(&mut self, msg: Authenticate) -> WsResult {
        let Authenticate { name, email } = msg;
        // TODO check attempt number, allow only 5 times per day
        let user = db_or_debug_and_return!(
            self,
            LookupUser { name, email },
            Ok(Some(WsMsg::Error(WsError::InvalidLoginPair))),
            Ok(Some(WsMsg::Error(WsError::InvalidLoginPair))); async
        );

        let token = db_or_debug_and_return!(self, CreateBindToken { user_id: user.id }; async);
        if let Some(bind_token) = token.bind_token.as_ref().cloned() {
            let _ = mail_or_debug_and_return!(
                self,
                Welcome {bind_token,
                    email: user.email,
                }; async
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

        let setting: jirs_data::UserSetting = db_or_debug_or_fallback!(
            self,
            database_actor::user_settings::FindUserSetting { user_id: user.id },
            crate::user_settings::default_user_setting(user.id),
            crate::user_settings::default_user_setting(user.id)
        );

        self.current_user = Some(user.clone());
        self.current_user_project = self.load_user_project().ok();
        self.current_project = self.load_project().ok();

        block_on(self.join_channel(ctx.address().recipient::<InnerMsg>()));
        Ok(Some(WsMsg::AuthorizeLoaded(Ok((user, setting)))))
    }
}

pub struct CheckBindToken {
    pub bind_token: uuid::Uuid,
}

#[async_trait::async_trait]
impl AsyncHandler<CheckBindToken> for WebSocketActor {
    async fn exec(&mut self, msg: CheckBindToken) -> WsResult {
        let token: Token = db_or_debug_and_return!(
            self,
            FindBindToken {
                token: msg.bind_token,
            },
            Ok(Some(WsMsg::BindTokenBad)),
            Ok(None); async
        );
        Ok(Some(WsMsg::BindTokenOk(token.access_token)))
    }
}
