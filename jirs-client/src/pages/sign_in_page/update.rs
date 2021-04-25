use std::str::FromStr;

use jirs_data::{SignInFieldId, WsMsg};
use seed::prelude::*;
use seed::*;
use uuid::Uuid;

use crate::model::{self, Model, Page, PageContent};
use crate::pages::sign_in_page::model::SignInPage;
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
            send_ws_msg(
                WsMsg::AuthenticateRequest(page.email.clone(), page.username.clone()),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::BindClientRequest => {
            let bind_token: uuid::Uuid = match Uuid::from_str(page.token.as_str()) {
                Ok(token) => token,
                Err(error) => {
                    error!(error);
                    return;
                }
            };
            send_ws_msg(WsMsg::BindTokenCheck(bind_token), model.ws.as_ref(), orders);
        }
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WsMsg(WsMsg::AuthenticateSuccess) => {
                page.login_success = true;
            }
            WebSocketChanged::WsMsg(WsMsg::BindTokenOk(access_token)) => {
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
