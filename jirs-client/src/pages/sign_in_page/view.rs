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
        model::{self, PageContent},
        shared::{outer_layout, ToNode},
        validations::{is_email, is_token},
        FieldId, Msg, SignInFieldId,
    },
    seed::{prelude::*, *},
};

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::SignIn(page) => page,
        _ => return empty![],
    };

    let username = StyledInput::build()
        .value(page.username.as_str())
        .valid(is_valid_username(page.username_touched, &page.username))
        .build(FieldId::SignIn(SignInFieldId::Username))
        .into_node();
    let username_field = StyledField::build()
        .label("Username")
        .input(username)
        .build()
        .into_node();

    let email = StyledInput::build()
        .value(page.email.as_str())
        .valid(is_valid_email(page.email_touched, page.email.as_str()))
        .build(FieldId::SignIn(SignInFieldId::Email))
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
        .input(div![C!["twoRow"], submit, register_link,])
        .build()
        .into_node();

    let help_icon = StyledIcon::build(Icon::Help)
        .add_class("noPasswordHelp")
        .size(22)
        .build()
        .into_node();

    let no_pass_section = div![
        C!["noPasswordSection"],
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

    let token = StyledInput::new_with_id_and_value_and_valid(
        FieldId::SignIn(SignInFieldId::Token),
        &page.token,
        is_valid_token(page.token_touched, page.token.as_str()),
    )
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

fn is_valid_username(touched: bool, s: &str) -> bool {
    !touched || (s.len() > 1 && s.len() < 20)
}

fn is_valid_email(touched: bool, s: &str) -> bool {
    !touched || (is_email(s) && s.len() < 20)
}

fn is_valid_token(touched: bool, s: &str) -> bool {
    !touched || is_token(s)
}
