use {
    crate::{
        model::{self, PageContent},
        shared::{
            outer_layout,
            styled_button::StyledButton,
            styled_field::StyledField,
            styled_form::StyledForm,
            styled_icon::{Icon, StyledIcon},
            styled_input::StyledInput,
            styled_link::StyledLink,
            ToNode,
        },
        validations::is_email,
        FieldId, Msg,
    },
    jirs_data::SignUpFieldId,
    seed::{prelude::*, *},
};

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::SignUp(page) => page,
        _ => return empty![],
    };

    let username = StyledInput::build()
        .value(page.username.as_str())
        .valid(!page.username_touched || page.username.len() > 1)
        .build(FieldId::SignUp(SignUpFieldId::Username))
        .into_node();
    let username_field = StyledField::build()
        .label("Username")
        .input(username)
        .build()
        .into_node();

    let email = StyledInput::build()
        .value(page.email.as_str())
        .valid(!page.email_touched || is_email(page.email.as_str()))
        .build(FieldId::SignUp(SignUpFieldId::Email))
        .into_node();
    let email_field = StyledField::build()
        .label("E-Mail")
        .input(email)
        .build()
        .into_node();

    let submit = if page.sign_up_success {
        StyledButton::build()
            .success()
            .text("✓ Please check your mail")
    } else {
        StyledButton::build()
            .primary()
            .text("Register")
            .on_click(mouse_ev(Ev::Click, |_| Msg::SignUpRequest))
    }
    .build()
    .into_node();

    let sign_in_link = StyledLink::build()
        .text("Sign In")
        .href("/login")
        .add_class("signInLink")
        .build()
        .into_node();

    let submit_field = StyledField::build()
        .input(div![C!["twoRow"], submit, sign_in_link,])
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
