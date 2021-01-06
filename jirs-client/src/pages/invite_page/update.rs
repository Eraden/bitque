use std::str::FromStr;

use seed::prelude::*;

use jirs_data::{fields::*, WsMsg};

use crate::{
    authorize_or_redirect,
    model::{Model, Page, PageContent},
    pages::invite_page::InvitePage,
    shared::write_auth_token,
    ws::send_ws_msg,
    FieldId, InvitationPageChange, Msg, PageChanged, WebSocketChanged,
};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match model.page_content {
        PageContent::Invite(..) => (),
        _ if model.page == Page::Invite => build_page_content(model),
        _ => (),
    };

    let page = match &mut model.page_content {
        PageContent::Invite(page) => page,
        _ => return,
    };

    match msg {
        Msg::WebSocketChange(WebSocketChanged::WsMsg(ws_msg)) => match ws_msg {
            WsMsg::InvitationAcceptFailure(_) => {
                page.error = Some("Invalid token".to_string());
            }
            WsMsg::InvitationAcceptSuccess(token) => {
                if let Ok(Msg::AuthTokenStored) = write_auth_token(Some(token)) {
                    authorize_or_redirect(model, orders);
                }
            }
            _ => (),
        },
        Msg::StrInputChanged(FieldId::Invite(InviteFieldId::Token), text) => {
            page.token_touched = true;
            page.token = text;
            page.error = None;
        }
        Msg::PageChanged(PageChanged::Invitation(InvitationPageChange::SubmitForm)) => {
            if let Ok(token) = uuid::Uuid::from_str(page.token.as_str()) {
                send_ws_msg(
                    WsMsg::InvitationAcceptRequest(token),
                    model.ws.as_ref(),
                    orders,
                );
                page.error = None;
            }
        }
        _ => {}
    }
}

fn build_page_content(model: &mut Model) {
    let s: String = seed::document().location().unwrap().to_string().into();
    let url = seed::Url::from_str(s.as_str()).unwrap();
    let search = url.search();
    let values = search.get("token").cloned().unwrap_or_default();
    let mut content = InvitePage::default();
    content.token = values.get(0).cloned().unwrap_or_default();
    model.page_content = PageContent::Invite(Box::new(content));
}
