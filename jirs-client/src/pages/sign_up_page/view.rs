use {
    crate::{
        components::{
            styled_button::StyledButton,
            styled_field::StyledField,
            styled_form::StyledForm,
            styled_icon::{Icon, StyledIcon},
            styled_input::StyledInput,
            styled_link::StyledLink,
        },
        match_page,
        model::{self, PageContent},
        shared::{outer_layout, ToNode},
        validations::is_email,
        FieldId, Msg,
    },
    jirs_data::SignUpFieldId,
    seed::{prelude::*, *},
};

use crate::components::styled_button::ButtonVariant;

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match_page!(model, SignUp; Empty);

    let username = StyledInput {
        value: page.username.as_str(),
        valid: !page.username_touched || page.username.len() > 1,
        id: Some(FieldId::SignUp(SignUpFieldId::Username)),
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
        valid: !page.email_touched || is_email(page.email.as_str()),
        id: Some(FieldId::SignUp(SignUpFieldId::Email)),
        ..Default::default()
    }
    .into_node();
    let email_field = StyledField {
        label: "E-Mail",
        input: email,
        ..Default::default()
    }
    .into_node();

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
    .into_node();

    let sign_in_link = StyledLink {
        children: vec![span!["Sign In"]],
        class_list: "signInLink",
        href: "/login",
    }
    .into_node();

    let submit_field = StyledField {
        input: div![C!["twoRow"], submit, sign_in_link],
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

    let error_row = if page.error.is_empty() {
        empty![]
    } else {
        div![C!["error"], p![page.error.as_str()]]
    };

    let sign_up_form = StyledForm::build()
        .heading("Sign In to your account")
        .on_submit(ev(Ev::Submit, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::SignUpRequest
        }))
        .add_field(username_field)
        .add_field(email_field)
        .add_field(submit_field)
        .add_field(no_pass_section)
        .add_field(error_row)
        .build()
        .into_node();
    let children = vec![sign_up_form];
    outer_layout(model, "register", children)
}
