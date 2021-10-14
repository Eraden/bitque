use std::str::FromStr;

use jirs_data::msg::{WsError, WsMsgSession};
use jirs_data::{SignInFieldId, WsMsg};
use seed::prelude::*;
use seed::*;
use uuid::Uuid;

use crate::model::{self, Model, Page, PageContent};
use crate::pages::sign_in_page::model::SignInPage;
use crate::pages::sign_in_page::SignInState;
use crate::shared::validate::*;
use crate::shared::write_auth_token;
use crate::ws::send_ws_msg;
use crate::{match_page_mut, FieldId, Msg, WebSocketChanged};

pub fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.page != Page::SignIn {
        return;
    } else if !matches!(model.page_content, PageContent::SignIn(..)) {
        build_page_content(model);
    } else if matches!(msg, Msg::ChangePage(Page::SignIn)) {
        build_page_content(model);
        return;
    };

    let page = match_page_mut!(model, SignIn);

    match msg {
        Msg::StrInputChanged(FieldId::SignIn(SignInFieldId::Username), value) => {
            page.username_v.validate(&value);
            page.username = value;
        }
        Msg::StrInputChanged(FieldId::SignIn(SignInFieldId::Email), value) => {
            page.email_v.validate(&value);
            page.email = value;
        }
        Msg::StrInputChanged(FieldId::SignIn(SignInFieldId::Token), value) => {
            page.token_v.validate(&value);
            page.token = value;
        }
        Msg::SignInRequest => {
            if page.email.is_empty() || page.username.is_empty() {
                return;
            }

            send_ws_msg(
                WsMsgSession::AuthenticateRequest(page.email.clone(), page.username.clone()).into(),
                model.ws.as_ref(),
                orders,
            );
            page.state = SignInState::RequestSend;
        }
        Msg::BindClientRequest => {
            let bind_token: uuid::Uuid = match Uuid::from_str(page.token.as_str()) {
                Ok(token) => token,
                Err(error) => {
                    error!(error);
                    return;
                }
            };
            send_ws_msg(
                WsMsgSession::BindTokenCheck(bind_token).into(),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::InvalidPair => {
            page.state = SignInState::InvalidPair;
        }
        Msg::SignIn(crate::pages::sign_in_page::SignInMsg::AuthenticateSuccess) => {
            let page = crate::match_page_mut!(model, SignIn);
            page.state = SignInState::EmailSend;
        }
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WsMsg(WsMsg::Session(WsMsgSession::AuthenticateSuccess)) => {
                page.state = SignInState::EmailSend;
            }
            WebSocketChanged::WsMsg(WsMsg::Error(WsError::InvalidPair(_, _))) => {
                page.state = SignInState::EmailSend;
            }
            WebSocketChanged::WsMsg(WsMsg::Session(WsMsgSession::BindTokenOk(access_token))) => {
                match write_auth_token(Some(access_token)) {
                    Ok(msg) => {
                        orders.skip().send_msg(msg);
                    }
                    Err(e) => {
                        error!(e);
                    }
                }
            }
            _ => (),
        },
        _ => (),
    };
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::SignIn(Box::new(SignInPage::default()));
}
