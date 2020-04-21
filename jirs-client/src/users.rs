use seed::{prelude::*, *};

use jirs_data::{ToVec, UsersFieldId};
use jirs_data::{UserRole, WsMsg};

use crate::api::send_ws_msg;
use crate::model::{Model, Page, PageContent, UsersPage};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::*;
use crate::shared::styled_select_child::ToStyledSelectChild;
use crate::shared::{inner_layout, ToNode};
use crate::validations::is_email;
use crate::{FieldId, Msg};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(Page::Users) => {
            model.page_content = PageContent::Users(UsersPage::default());
            return;
        }
        _ => (),
    }

    let page = match &mut model.page_content {
        PageContent::Users(page) => page,
        _ => return,
    };

    page.user_role_state.update(&msg, orders);

    match msg {
        Msg::StyledSelectChanged(
            FieldId::Users(UsersFieldId::UserRole),
            StyledSelectChange::Changed(role),
        ) => {
            page.user_role = role.into();
        }
        Msg::InputChanged(FieldId::Users(UsersFieldId::Username), name) => {
            page.name = name;
            page.name_touched = true;
        }
        Msg::InputChanged(FieldId::Users(UsersFieldId::Email), email) => {
            page.email = email;
            page.email_touched = true;
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    if model.user.is_none() {
        return empty![];
    }

    let page = match &model.page_content {
        PageContent::Users(page) => page,
        _ => return empty![],
    };

    let name = StyledInput::build(FieldId::Users(UsersFieldId::Username))
        .valid(!page.name_touched || page.name.len() >= 3)
        .value(page.name.as_str())
        .build()
        .into_node();
    let name_field = StyledField::build()
        .input(name)
        .label("Name")
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::Users(UsersFieldId::Email))
        .valid(!page.email_touched || is_email(page.email.as_str()))
        .value(page.email.as_str())
        .build()
        .into_node();
    let email_field = StyledField::build()
        .input(email)
        .label("E-Mail")
        .build()
        .into_node();

    let user_role = StyledSelect::build(FieldId::Users(UsersFieldId::UserRole))
        .name("user_role")
        .valid(true)
        .normal()
        .with_state(&page.user_role_state)
        .selected(vec![page.user_role.to_select_child()])
        .options(
            UserRole::ordered()
                .into_iter()
                .map(|role| role.to_select_child())
                .collect(),
        )
        .build()
        .into_node();
    let user_role_field = StyledField::build()
        .input(user_role)
        .label("Role")
        .build()
        .into_node();

    let submit = StyledButton::build()
        .add_class("submitUserInvite")
        .active(true)
        .primary()
        .text("Invite user")
        .build()
        .into_node();
    let submit_field = StyledField::build().input(submit).build().into_node();

    let form = StyledForm::build()
        .heading("Invite new user")
        .add_field(name_field)
        .add_field(email_field)
        .add_field(user_role_field)
        .add_field(submit_field)
        .build()
        .into_node();

    inner_layout(model, "users", vec![form], empty![])
}
