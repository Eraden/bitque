use seed::{prelude::*, *};

use crate::api::send_ws_msg;
use crate::model::{LoginPage, Page, PageContent};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_input::StyledInput;
use crate::shared::{outer_layout, ToNode};
use crate::{model, FieldId, LoginFieldId, Msg};
use jirs_data::WsMsg;

pub fn update(msg: Msg, model: &mut model::Model, _orders: &mut impl Orders<Msg>) {
    if model.page != Page::Login {
        return;
    }

    if msg == Msg::ChangePage(Page::Login) {
        model.page_content = PageContent::Login(LoginPage::default());
        return;
    }

    let page = match &mut model.page_content {
        PageContent::Login(page) => page,
        _ => return,
    };

    match msg {
        Msg::InputChanged(FieldId::Login(LoginFieldId::Username), value) => {
            page.username = value;
        }
        Msg::InputChanged(FieldId::Login(LoginFieldId::Email), value) => {
            page.email = value;
        }
        Msg::SignInRequest => {
            send_ws_msg(WsMsg::AuthenticateRequest(
                page.email.clone(),
                page.username.clone(),
            ));
        }
        Msg::WsMsg(WsMsg::AuthenticateSuccess) => {
            page.login_success = true;
        }
        _ => (),
    };
}

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Login(page) => page,
        _ => return empty![],
    };

    let username = StyledInput::build(FieldId::Login(LoginFieldId::Username))
        .valid(true)
        .value(page.username.as_str())
        .build()
        .into_node();
    let username_field = StyledField::build()
        .label("Username")
        .input(username)
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::Login(LoginFieldId::Email))
        .valid(true)
        .value(page.email.as_str())
        .input_type("email")
        .build()
        .into_node();
    let email_field = StyledField::build()
        .label("E-Mail")
        .input(email)
        .build()
        .into_node();

    let submit = if page.login_success {
        StyledButton::build().success().text("✓")
    } else {
        StyledButton::build()
            .primary()
            .text("Sign In")
            .on_click(mouse_ev(Ev::Click, |_| Msg::SignInRequest))
    }
    .build()
    .into_node();
    let submit_field = StyledField::build().input(submit).build().into_node();

    let help_icon = StyledIcon::build(Icon::Help)
        .add_class("noPasswordHelp")
        .size(22)
        .build()
        .into_node();

    let no_pass_section = div![
        class!["noPasswordSection"],
        attrs![At::Title => "We don't believe password is helping anyone. Instead after user provide correct login and e-mail he'll receive mail with 1-use token."],
        help_icon,
        span!["Why I don't see password?"]
    ];

    let token = StyledInput::build(FieldId::Login(LoginFieldId::Token))
        .valid(true)
        .value(page.token.as_str())
        .build()
        .into_node();
    let token_field = StyledField::build()
        .label("Single use token")
        .input(token)
        .build()
        .into_node();
    let submit_token = StyledButton::build()
        .primary()
        .text("Authorize")
        .on_click(mouse_ev(Ev::Click, |_| Msg::SignInRequest))
        .build()
        .into_node();
    let submit_token_field = StyledField::build().input(submit_token).build().into_node();

    let form = StyledForm::build()
        .heading("Sign In to your account")
        .add_field(username_field)
        .add_field(email_field)
        .add_field(submit_field)
        .add_field(no_pass_section)
        .add_field(token_field)
        .add_field(submit_token_field)
        .build()
        .into_node();

    let children = vec![form];
    outer_layout(model, "login", children)
}
