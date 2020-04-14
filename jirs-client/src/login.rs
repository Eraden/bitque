use seed::{prelude::*, *};

use crate::model::Page;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_input::StyledInput;
use crate::shared::{outer_layout, ToNode};
use crate::{model, FieldId, LoginFieldId, Msg};

pub fn update(msg: Msg, model: &mut model::Model, _orders: &mut impl Orders<Msg>) {
    if model.page != Page::Login {
        return;
    }

    match msg {
        _ => (),
    };
}

pub fn view(model: &model::Model) -> Node<Msg> {
    let login = StyledInput::build(FieldId::Login(LoginFieldId::Email))
        .valid(true)
        .build()
        .into_node();
    let login_field = StyledField::build()
        .label("Login")
        .input(login)
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::Login(LoginFieldId::Email))
        .valid(true)
        .build()
        .into_node();
    let email_field = StyledField::build()
        .label("E-Mail")
        .input(email)
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

    let form = StyledForm::build()
        .heading("Sign In to your account")
        .add_field(login_field)
        .add_field(email_field)
        .add_field(no_pass_section)
        .build()
        .into_node();

    let children = vec![form];
    outer_layout(model, "login", children)
}
