use std::str::FromStr;

use seed::{prelude::*, *};

use jirs_data::{InviteFieldId, WsMsg};

use crate::model::{InvitePage, Model, Page, PageContent};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::{outer_layout, write_auth_token, ToNode};
use crate::validations::is_token;
use crate::ws::send_ws_msg;
use crate::{
    authorize_or_redirect, FieldId, InvitationPageChange, Msg, PageChanged, WebSocketChanged,
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

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Invite(page) => page,
        _ => return empty![],
    };

    let token_field = token_field(page);
    let submit_field = submit(page);
    let error = match page.error.as_ref() {
        Some(s) => div![class!["error"], s.as_str()],
        _ => empty![],
    };

    let form = StyledForm::build()
        .heading("Welcome in JIRS")
        .on_submit(ev(Ev::Submit, move |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Invitation(InvitationPageChange::SubmitForm))
        }))
        .add_field(token_field)
        .add_field(submit_field)
        .add_field(error)
        .build()
        .into_node();

    outer_layout(model, "invite", vec![form])
}

fn submit(_page: &InvitePage) -> Node<Msg> {
    let submit = StyledButton::build()
        .text("Accept")
        .primary()
        .build()
        .into_node();
    StyledField::build().input(submit).build().into_node()
}

fn token_field(page: &InvitePage) -> Node<Msg> {
    let token = StyledInput::build()
        .valid(!page.token_touched || is_token(page.token.as_str()) && page.error.is_none())
        .value(page.token.as_str())
        .build(FieldId::Invite(InviteFieldId::Token))
        .into_node();

    StyledField::build()
        .input(token)
        .label("Your invite token")
        .build()
        .into_node()
}
