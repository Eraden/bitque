use jirs_data::SignUpFieldId;
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::StyledInput;
use crate::components::styled_link::StyledLink;
use crate::shared::outer_layout;
use crate::validations::is_email;
use crate::{match_page, model, FieldId, Msg};

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match_page!(model, SignUp; Empty);

    let username_field = StyledField {
        label: "Username",
        input: StyledInput {
            value: page.username.as_str(),
            valid: !page.username_touched || page.username.len() > 1,
            id: Some(FieldId::SignUp(SignUpFieldId::Username)),
            ..Default::default()
        }
        .render(),
        ..Default::default()
    }
    .render();

    let email_field = StyledField {
        label: "E-Mail",
        input: StyledInput {
            value: page.email.as_str(),
            valid: !page.email_touched || is_email(page.email.as_str()),
            id: Some(FieldId::SignUp(SignUpFieldId::Email)),
            ..Default::default()
        }
        .render(),
        ..Default::default()
    }
    .render();

    let submit = if page.sign_up_success {
        StyledButton {
            variant: ButtonVariant::Success,
            text: Some("✓ Please check your mail"),
            ..Default::default()
        }
    } else {
        StyledButton {
            variant: ButtonVariant::Primary,
            text: Some("Register"),
            on_click: Some(mouse_ev(Ev::Click, |_| Msg::SignUpRequest)),
            ..Default::default()
        }
    }
    .render();

    let submit_field = StyledField {
        input: div![
            C!["twoRow"],
            submit,
            StyledLink {
                children: vec![span!["Sign In"]],
                class_list: "signInLink",
                href: "/login",
                ..Default::default()
            }
            .render()
        ],
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

    let error_row = if page.error.is_empty() {
        empty![]
    } else {
        div![C!["error"], p![page.error.as_str()]]
    };

    let sign_up_form = StyledForm {
        heading: "Sign In to your account",
        on_submit: Some(ev(Ev::Submit, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::SignUpRequest
        })),
        fields: vec![
            username_field,
            email_field,
            submit_field,
            no_pass_section,
            error_row,
        ],
    }
    .render();
    let children = vec![sign_up_form];
    outer_layout(model, "register", children)
}
