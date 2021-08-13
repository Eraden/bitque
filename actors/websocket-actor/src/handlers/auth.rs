use actix::AsyncContext;
use database_actor::authorize_user::AuthorizeUser;
use database_actor::tokens::{CreateBindToken, FindBindToken};
use database_actor::users::LookupUser;
use futures::executor::block_on;
use jirs_data::msg::{WsError, WsMsgSession};
use jirs_data::{Token, WsMsg};
use mail_actor::welcome::Welcome;

use crate::server::InnerMsg;
use crate::{db_or_debug_and_return, db_or_debug_or_fallback, mail_or_debug_and_return, *};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgSession> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgSession) -> WsResult {
        match msg {
            // auth
            WsMsgSession::BindTokenCheck(uuid) => {
                self.exec(CheckBindToken { bind_token: uuid }).await
            }
            WsMsgSession::AuthenticateRequest(email, name) => {
                self.exec(Authenticate { name, email }).await
            }

            // register
            WsMsgSession::SignUpRequest(email, username) => {
                self.exec(Register {
                    name: username,
                    email,
                })
                .await
            }

            WsMsgSession::AuthorizeLoad(_) => Ok(None),
            WsMsgSession::AuthorizeLoaded(_) => Ok(None),
            WsMsgSession::AuthorizeExpired => Ok(None),
            WsMsgSession::AuthenticateSuccess => Ok(None),
            WsMsgSession::BindTokenBad => Ok(None),
            WsMsgSession::BindTokenOk(_) => Ok(None),
            WsMsgSession::SignUpSuccess => Ok(None),
            WsMsgSession::SignUpPairTaken => Ok(None),
        }
    }
}

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
        Ok(Some(WsMsgSession::AuthenticateSuccess.into()))
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
            Ok(Some(
                WsMsgSession::AuthorizeLoaded(Err("Invalid auth token".to_string())).into()
            )),
            Ok(Some(WsMsgSession::AuthorizeExpired.into()))
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
        Ok(Some(
            WsMsgSession::AuthorizeLoaded(Ok((user, setting))).into(),
        ))
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
            Ok(Some(WsMsgSession::BindTokenBad.into())),
            Ok(None); async
        );
        Ok(Some(WsMsgSession::BindTokenOk(token.access_token).into()))
    }
}
