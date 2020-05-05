use seed::{prelude::*, *};
use web_sys::FormData;

use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{Model, Page, PageContent, ProfilePage};
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_image_input::StyledImageInput;
use crate::shared::styled_input::StyledInput;
use crate::shared::{inner_layout, ToNode};
use crate::{FieldId, Msg, HOST_URL};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
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

    let profile_page = match &mut model.page_content {
        PageContent::Profile(profile_page) => profile_page,
        _ => return,
    };

    profile_page.name.update(&msg);
    profile_page.email.update(&msg);
    profile_page.avatar.update(&msg);

    match msg {
        Msg::FileInputChanged(FieldId::Profile(UsersFieldId::Avatar), ..) => {
            let file = match profile_page.avatar.file.as_ref() {
                Some(f) => f,
                _ => return,
            };
            let token = match crate::shared::read_auth_token() {
                Ok(uuid) => uuid,
                _ => return,
            };
            let fd = FormData::new().unwrap();
            fd.set_with_str("token", format!("{}", token).as_str())
                .unwrap();
            fd.set_with_blob("avatar", file).unwrap();
            orders.perform_cmd(update_avatar(fd));
            orders.skip();
        }
        Msg::WsMsg(WsMsg::AvatarUrlChanged(user_id, avatar_url)) => {
            if let Some(me) = model.user.as_mut() {
                if me.id == user_id {
                    profile_page.avatar.url = Some(avatar_url.clone());
                }
            }
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Profile(profile_page) => profile_page,
        _ => return empty![],
    };

    let avatar = StyledImageInput::build(FieldId::Profile(UsersFieldId::Avatar))
        .add_class("avatar")
        .state(&page.avatar)
        .build()
        .into_node();

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
        .add_field(avatar)
        .add_field(username_field)
        .add_field(email_field)
        .build()
        .into_node();
    inner_layout(model, "profile", vec![content], empty![])
}

async fn update_avatar(data: FormData) -> Result<Msg, Msg> {
    let host_url = unsafe { HOST_URL.clone() };
    let path = format!("{}/avatar/", host_url);
    Request::new(path)
        .method(Method::Post)
        .body(data.into())
        .fetch_string(Msg::AvatarUpdateFetched)
        .await
}
