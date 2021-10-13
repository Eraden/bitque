use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::StyledInput;
use crate::components::styled_link::StyledLink;
use crate::pages::sign_in_page::{events, SignInPage, SignInState};
use crate::shared::outer_layout;
use crate::shared::validate::Validator;
use crate::{match_page, model, FieldId, Msg, SignInFieldId};

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match_page!(model, SignIn; Empty);

    let sign_in_form = if page.state == SignInState::EmailSend {
        StyledForm {
            fields: vec![StyledField {
                input: div![
                    C!["emailSend"],
                    div![crate::images::email_send::render()],
                    div!["E-Mail send"]
                ],
                ..Default::default()
            }
            .render()],
            ..Default::default()
        }
        .render()
    } else {
        StyledForm {
            heading: "Sign In to your account",
            fields: vec![
                username_field(page),
                email_field(page),
                submit_field(page),
                if page.state == SignInState::InvalidPair {
                    div![C!["error"], p!["Invalid credentials"]]
                } else {
                    Node::Empty
                },
                no_pass_section(),
            ],
            on_submit: Some(events::on_submit_send_sign_in()),
        }
        .render()
    };

    let bind_token_form = StyledForm {
        on_submit: Some(events::on_submit_bind_client()),
        fields: vec![token_field(page), submit_token_field()],
        ..Default::default()
    }
    .render();

    outer_layout(model, "login", vec![sign_in_form, bind_token_form])
}

fn submit_token_field() -> Node<Msg> {
    StyledField {
        input: StyledButton {
            variant: ButtonVariant::Primary,
            text: Some("Authorize"),
            on_click: Some(events::on_click_bind_client()),
            ..Default::default()
        }
        .render(),
        ..Default::default()
    }
    .render()
}

fn token_field(page: &SignInPage) -> Node<Msg> {
    StyledField {
        label: "Single use token",
        input: StyledInput {
            id: Some(FieldId::SignIn(SignInFieldId::Token)),
            valid: page.token_v.is_valid(),
            value: page.token.as_str(),
            err_msg: page.token_v.message(),
            ..Default::default()
        }
        .render(),
        ..Default::default()
    }
    .render()
}

fn no_pass_section() -> Node<Msg> {
    let help_icon = StyledIcon {
        icon: Icon::Help,
        class_list: "noPasswordHelp",
        size: Some(22),
        ..Default::default()
    }
    .render();
    div![
        C!["noPasswordSection"],
        attrs![At::Title => "We don't believe password is helping anyone. Instead after user provide correct login and e-mail he'll receive mail with 1-use token."],
        help_icon,
        span!["Why I don't see password?"]
    ]
}

fn submit_field(page: &SignInPage) -> Node<Msg> {
    let submit = if page.state == SignInState::RequestSend {
        StyledButton {
            variant: ButtonVariant::Success,
            text: Some("Checking..."),
            ..Default::default()
        }
    } else {
        StyledButton {
            variant: ButtonVariant::Primary,
            text: Some("Sign In"),
            on_click: Some(events::on_click_send_sign_in()),
            ..Default::default()
        }
    }
    .render();
    let register_link = StyledLink {
        children: vec![span!["Register"]],
        class_list: "signUpLink",
        href: "/register",
        ..Default::default()
    }
    .render();
    StyledField {
        input: div![C!["twoRow"], submit, register_link],
        ..Default::default()
    }
    .render()
}

fn username_field(page: &SignInPage) -> Node<Msg> {
    StyledField {
        label: "Username",
        input: StyledInput {
            value: page.username.as_str(),
            valid: page.username_v.is_valid(),
            id: Some(FieldId::SignIn(SignInFieldId::Username)),
            err_msg: page.username_v.message(),
            ..Default::default()
        }
        .render(),
        ..Default::default()
    }
    .render()
}

fn email_field(page: &SignInPage) -> Node<Msg> {
    StyledField {
        label: "E-Mail",
        input: StyledInput {
            value: page.email.as_str(),
            valid: page.email_v.is_valid(),
            id: Some(FieldId::SignIn(SignInFieldId::Email)),
            err_msg: page.email_v.message(),
            ..Default::default()
        }
        .render(),
        ..Default::default()
    }
    .render()
}
