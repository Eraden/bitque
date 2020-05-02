use seed::{prelude::*, *};

use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{Model, Page, PageContent, ProfilePage};
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::{inner_layout, ToNode};
use crate::{FieldId, Msg};

pub fn update(msg: Msg, model: &mut crate::model::Model, _orders: &mut impl Orders<Msg>) {
    let user = match model.user {
        Some(ref user) => user,
        _ => return,
    };

    match msg {
        Msg::WsMsg(WsMsg::AuthorizeLoaded(..)) | Msg::ChangePage(Page::Profile) => {
            send_ws_msg(WsMsg::ProjectRequest);
            model.page_content = PageContent::Profile(Box::new(ProfilePage::new(user)));
        }
        _ => (),
    }

    let project_page = match &mut model.page_content {
        PageContent::Profile(profile_page) => profile_page,
        _ => return,
    };

    project_page.name.update(&msg);
    project_page.email.update(&msg);
}

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Profile(profile_page) => profile_page,
        _ => return empty![],
    };

    let username = StyledInput::build(FieldId::Profile(UsersFieldId::Username))
        .state(&page.name)
        .valid(true)
        .primary()
        .build()
        .into_node();
    let username_field = StyledField::build()
        .label("Username")
        .input(username)
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::Profile(UsersFieldId::Username))
        .state(&page.email)
        .valid(true)
        .primary()
        .build()
        .into_node();
    let email_field = StyledField::build()
        .label("E-Mail")
        .input(email)
        .build()
        .into_node();

    let content = StyledForm::build()
        .heading("Profile")
        .add_field(username_field)
        .add_field(email_field)
        .build()
        .into_node();
    inner_layout(model, "profile", vec![content], empty![])
}
