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
    if let Msg::ChangePage(Page::Project) = msg {
        model.page_content = PageContent::Invite(Box::new(InvitePage::default()));
        return;
    }

    let page = match &mut model.page_content {
        PageContent::Invite(page) => page,
        _ => return,
    };

    if let Msg::InputChanged(FieldId::Invite(InviteFieldId::Token), text) = msg {
        page.token_touched = true;
        page.token = text;
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
