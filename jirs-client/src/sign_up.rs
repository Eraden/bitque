use seed::{prelude::*, *};

use jirs_data::{SignUpFieldId, WsMsg};

use crate::model::{Model, Page, PageContent, SignUpPage};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_link::StyledLink;
use crate::shared::{outer_layout, ToNode};
use crate::validations::is_email;
use crate::ws::send_ws_msg;
use crate::{model, FieldId, Msg, WebSocketChanged};

pub fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.page != Page::SignUp {
        return;
    }

    if let Msg::ChangePage(Page::SignUp) = msg {
        build_page_content(model);
        return;
    };

    let page = match &mut model.page_content {
        PageContent::SignUp(page) => page,
        _ => return,
    };

    match msg {
        Msg::StrInputChanged(FieldId::SignUp(SignUpFieldId::Username), value) => {
            page.username = value;
            page.username_touched = true;
        }
        Msg::StrInputChanged(FieldId::SignUp(SignUpFieldId::Email), value) => {
            page.email = value;
            page.email_touched = true;
        }
        Msg::SignUpRequest => {
            send_ws_msg(
                WsMsg::SignUpRequest(page.email.clone(), page.username.clone()),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WsMsg(WsMsg::SignUpSuccess) => {
                page.sign_up_success = true;
            }
            WebSocketChanged::WsMsg(WsMsg::SignUpPairTaken) => {
                page.error = "Pair you give is either taken or is not matching".to_string();
            }
            _ => (),
        },
        _ => (),
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::SignUp(Box::new(SignUpPage::default()));
}

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::SignUp(page) => page,
        _ => return empty![],
    };

    let username = StyledInput::build(FieldId::SignUp(SignUpFieldId::Username))
        .value(page.username.as_str())
        .valid(!page.username_touched || page.username.len() > 1)
        .build()
        .into_node();
    let username_field = StyledField::build()
        .label("Username")
        .input(username)
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::SignUp(SignUpFieldId::Email))
        .value(page.email.as_str())
        .valid(!page.email_touched || is_email(page.email.as_str()))
        .build()
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
        .input(div![class!["twoRow"], submit, sign_in_link,])
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

    let error_row = if page.error.is_empty() {
        empty![]
    } else {
        div![class!["error"], p![page.error.as_str()]]
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
