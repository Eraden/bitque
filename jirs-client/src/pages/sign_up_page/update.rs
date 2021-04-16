use jirs_data::{SignUpFieldId, WsMsg};
use seed::prelude::*;

use crate::model::{self, Model, Page, PageContent};
use crate::pages::sign_up_page::model::SignUpPage;
use crate::ws::send_ws_msg;
use crate::{FieldId, Msg, WebSocketChanged};

pub fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.page != Page::SignUp {
        return;
    }

    if let Msg::ChangePage(Page::SignUp) = msg {
        build_page_content(model);
        return;
    };

    let page = match &mut model.page_content {
        PageContent::SignUp(page) => page,
        _ => return,
    };

    match msg {
        Msg::StrInputChanged(FieldId::SignUp(SignUpFieldId::Username), value) => {
            page.username = value;
            page.username_touched = true;
        }
        Msg::StrInputChanged(FieldId::SignUp(SignUpFieldId::Email), value) => {
            page.email = value;
            page.email_touched = true;
        }
        Msg::SignUpRequest => {
            send_ws_msg(
                WsMsg::SignUpRequest(page.email.clone(), page.username.clone()),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WsMsg(WsMsg::SignUpSuccess) => {
                page.sign_up_success = true;
            }
            WebSocketChanged::WsMsg(WsMsg::SignUpPairTaken) => {
                page.error = "Pair you give is either taken or is not matching".to_string();
            }
            _ => (),
        },
        _ => (),
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::SignUp(Box::new(SignUpPage::default()));
}
