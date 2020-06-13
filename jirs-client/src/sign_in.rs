use std::str::FromStr;

use seed::{prelude::*, *};
use uuid::Uuid;

use jirs_data::WsMsg;

use crate::model::{Model, Page, PageContent, SignInPage};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_link::StyledLink;
use crate::shared::{outer_layout, write_auth_token, ToNode};
use crate::validations::{is_email, is_token};
use crate::ws::send_ws_msg;
use crate::{model, FieldId, Msg, SignInFieldId, WebSocketChanged};

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

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::SignIn(page) => page,
        _ => return empty![],
    };

    let username = StyledInput::build(FieldId::SignIn(SignInFieldId::Username))
        .value(page.username.as_str())
        .valid(!page.username_touched || page.username.len() > 1)
        .build()
        .into_node();
    let username_field = StyledField::build()
        .label("Username")
        .input(username)
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::SignIn(SignInFieldId::Email))
        .value(page.email.as_str())
        .valid(!page.email_touched || is_email(page.email.as_str()))
        .build()
        .into_node();
    let email_field = StyledField::build()
        .label("E-Mail")
        .input(email)
        .build()
        .into_node();

    let submit = if page.login_success {
        StyledButton::build()
            .success()
            .text("✓ Please check your mail")
    } else {
        StyledButton::build()
            .primary()
            .text("Sign In")
            .on_click(mouse_ev(Ev::Click, |_| Msg::SignInRequest))
    }
    .build()
    .into_node();
    let register_link = StyledLink::build()
        .text("Register")
        .href("/register")
        .add_class("signUpLink")
        .build()
        .into_node();
    let submit_field = StyledField::build()
        .input(div![class!["twoRow"], submit, register_link,])
        .build()
        .into_node();

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

    let sign_in_form = StyledForm::build()
        .heading("Sign In to your account")
        .on_submit(ev(Ev::Submit, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::SignInRequest
        }))
        .add_field(username_field)
        .add_field(email_field)
        .add_field(submit_field)
        .add_field(no_pass_section)
        .build()
        .into_node();

    let token = StyledInput::build(FieldId::SignIn(SignInFieldId::Token))
        .value(page.token.as_str())
        .valid(!page.token_touched || is_token(page.token.as_str()))
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
        .on_click(mouse_ev(Ev::Click, |_| Msg::BindClientRequest))
        .build()
        .into_node();
    let submit_token_field = StyledField::build().input(submit_token).build().into_node();

    let bind_token_form = StyledForm::build()
        .on_submit(ev(Ev::Submit, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::BindClientRequest
        }))
        .add_field(token_field)
        .add_field(submit_token_field)
        .build()
        .into_node();

    let children = vec![sign_in_form, bind_token_form];
    outer_layout(model, "login", children)
}
