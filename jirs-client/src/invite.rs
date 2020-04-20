use seed::{prelude::*, *};

use jirs_data::InviteFieldId;

use crate::model::{InvitePage, Model, Page, PageContent};
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::{outer_layout, ToNode};
use crate::validations::is_token;
use crate::{FieldId, Msg};

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(Page::Project) => {
            model.page_content = PageContent::Invite(InvitePage::default());
            return;
        }
        _ => (),
    }

    let page = match &mut model.page_content {
        PageContent::Invite(page) => page,
        _ => return,
    };

    match msg {
        Msg::InputChanged(FieldId::Invite(InviteFieldId::Token), text) => {
            page.token_touched = true;
            page.token = text.clone();
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Invite(page) => page,
        _ => return empty![],
    };

    let token = StyledInput::build(FieldId::Invite(InviteFieldId::Token))
        .valid(!page.token_touched || is_token(page.token.as_str()))
        .build()
        .into_node();
    let token_field = StyledField::build()
        .input(token)
        .label("Your invite token")
        .build()
        .into_node();

    let form = StyledForm::build()
        .heading("Welcome in JIRS")
        .add_field(token_field)
        .build()
        .into_node();

    outer_layout(model, "invite", vec![form])
}
