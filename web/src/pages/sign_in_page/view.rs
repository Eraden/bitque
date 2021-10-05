use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::StyledInput;
use crate::components::styled_link::StyledLink;
use crate::shared::outer_layout;
use crate::shared::validate::Validator;
use crate::{match_page, model, FieldId, Msg, SignInFieldId};

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match_page!(model, SignIn; Empty);

    let username = StyledInput {
        value: page.username.as_str(),
        valid: page.username_v.is_valid(),
        id: Some(FieldId::SignIn(SignInFieldId::Username)),
        err_msg: page.username_v.message(),
        ..Default::default()
    }
    .render();
    let username_field = StyledField {
        label: "Username",
        input: username,
        ..Default::default()
    }
    .render();

    let email = StyledInput {
        value: page.email.as_str(),
        valid: page.email_v.is_valid(),
        id: Some(FieldId::SignIn(SignInFieldId::Email)),
        err_msg: page.email_v.message(),
        ..Default::default()
    }
    .render();
    let email_field = StyledField {
        label: "E-Mail",
        input: email,
        ..Default::default()
    }
    .render();

    let submit = if page.login_success {
        StyledButton {
            variant: ButtonVariant::Success,
            text: Some("✓ Please check your mail"),
            ..Default::default()
        }
    } else {
        StyledButton {
            variant: ButtonVariant::Primary,
            text: Some("Sign In"),
            on_click: Some(mouse_ev(Ev::Click, |_| Msg::SignInRequest)),
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
    let submit_field = StyledField {
        input: div![C!["twoRow"], submit, register_link],
        ..Default::default()
    }
    .render();

    let help_icon = StyledIcon {
        icon: Icon::Help,
        class_list: "noPasswordHelp",
        size: Some(22),
        ..Default::default()
    }
    .render();

    let no_pass_section = div![
        C!["noPasswordSection"],
        attrs![At::Title => "We don't believe password is helping anyone. Instead after user provide correct login and e-mail he'll receive mail with 1-use token."],
        help_icon,
        span!["Why I don't see password?"]
    ];

    let sign_in_form = StyledForm {
        heading: "Sign In to your account",
        fields: vec![username_field, email_field, submit_field, no_pass_section],
        on_submit: Some(ev(Ev::Submit, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::SignInRequest
        })),
    }
    .render();

    let token = StyledInput {
        id: Some(FieldId::SignIn(SignInFieldId::Token)),
        valid: page.token_v.is_valid(),
        value: page.token.as_str(),
        err_msg: page.token_v.message(),
        ..Default::default()
    }
    .render();
    let token_field = StyledField {
        label: "Single use token",
        input: token,
        ..Default::default()
    }
    .render();
    let submit_token = StyledButton {
        variant: ButtonVariant::Primary,
        text: Some("Authorize"),
        on_click: Some(mouse_ev(Ev::Click, |_| Msg::BindClientRequest)),
        ..Default::default()
    }
    .render();
    let submit_token_field = StyledField {
        input: submit_token,
        ..Default::default()
    }
    .render();

    let bind_token_form = StyledForm {
        on_submit: Some(ev(Ev::Submit, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::BindClientRequest
        })),
        fields: vec![token_field, submit_token_field],
        ..Default::default()
    }
    .render();

    let children = vec![sign_in_form, bind_token_form];
    outer_layout(model, "login", children)
}
