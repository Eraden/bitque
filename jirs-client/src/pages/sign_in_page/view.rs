use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::StyledInput;
use crate::components::styled_link::StyledLink;
use crate::model::{self, PageContent};
use crate::shared::{outer_layout, ToNode};
use crate::validations::{is_email, is_token};
use crate::{FieldId, Msg, SignInFieldId};

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::SignIn(page) => page,
        _ => return empty![],
    };

    let username = StyledInput {
        value: page.username.as_str(),
        valid: is_valid_username(page.username_touched, &page.username),
        id: Some(FieldId::SignIn(SignInFieldId::Username)),
        ..Default::default()
    }
    .into_node();
    let username_field = StyledField {
        label: "Username",
        input: username,
        ..Default::default()
    }
    .into_node();

    let email = StyledInput {
        value: page.email.as_str(),
        valid: is_valid_email(page.email_touched, page.email.as_str()),
        id: Some(FieldId::SignIn(SignInFieldId::Email)),
        ..Default::default()
    }
    .into_node();
    let email_field = StyledField {
        label: "E-Mail",
        input: email,
        ..Default::default()
    }
    .into_node();

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
    .into_node();
    let register_link = StyledLink {
        children: vec![span!["Register"]],
        class_list: "signUpLink",
        href: "/register",
    }
    .into_node();
    let submit_field = StyledField {
        input: div![C!["twoRow"], submit, register_link],
        ..Default::default()
    }
    .into_node();

    let help_icon = StyledIcon {
        icon: Icon::Help,
        class_list: "noPasswordHelp",
        size: Some(22),
        ..Default::default()
    }
    .into_node();

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
    .into_node();

    let token = StyledInput::new_with_id_and_value_and_valid(
        FieldId::SignIn(SignInFieldId::Token),
        &page.token,
        is_valid_token(page.token_touched, page.token.as_str()),
    )
    .into_node();
    let token_field = StyledField {
        label: "Single use token",
        input: token,
        ..Default::default()
    }
    .into_node();
    let submit_token = StyledButton {
        variant: ButtonVariant::Primary,
        text: Some("Authorize"),
        on_click: Some(mouse_ev(Ev::Click, |_| Msg::BindClientRequest)),
        ..Default::default()
    }
    .into_node();
    let submit_token_field = StyledField {
        input: submit_token,
        ..Default::default()
    }
    .into_node();

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

fn is_valid_username(touched: bool, s: &str) -> bool {
    !touched || (s.len() > 1 && s.len() < 20)
}

fn is_valid_email(touched: bool, s: &str) -> bool {
    !touched || (is_email(s) && s.len() < 20)
}

fn is_valid_token(touched: bool, s: &str) -> bool {
    !touched || is_token(s)
}
