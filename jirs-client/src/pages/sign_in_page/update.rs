use std::str::FromStr;

use seed::{prelude::*, *};
use uuid::Uuid;

use jirs_data::{SignInFieldId, WsMsg};

use crate::pages::sign_in_page::model::SignInPage;
use crate::{
    model::{self, Model, Page, PageContent},
    shared::write_auth_token,
    ws::send_ws_msg,
    FieldId, Msg, WebSocketChanged,
};

pub fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.page != Page::SignIn {
        return;
    }

    if let Msg::ChangePage(Page::SignIn) = msg {
        build_page_content(model);
        return;
    };

    let page = match &mut model.page_content {
        PageContent::SignIn(page) => page,
        _ => return,
    };

    match msg {
        Msg::StrInputChanged(FieldId::SignIn(SignInFieldId::Username), value) => {
            page.username = value;
            page.username_touched = true;
        }
        Msg::StrInputChanged(FieldId::SignIn(SignInFieldId::Email), value) => {
            page.email = value;
            page.email_touched = true;
        }
        Msg::StrInputChanged(FieldId::SignIn(SignInFieldId::Token), value) => {
            page.token = value;
            page.token_touched = true;
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
