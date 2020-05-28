use seed::prelude::{Method, Orders, Request};
use web_sys::FormData;

use jirs_data::{UsersFieldId, WsMsg};

use crate::model::{Model, Page, PageContent, ProfilePage};
use crate::shared::styled_select::StyledSelectChange;
use crate::ws::{enqueue_ws_msg, send_ws_msg};
use crate::{FieldId, Msg, PageChanged, ProfilePageChange, WebSocketChanged};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(..)))
        | Msg::ChangePage(Page::Profile) => {
            init_load(model, orders);
            build_page_content(model);
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
    profile_page.current_project.update(&msg, orders);

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
            orders.perform_cmd(update_avatar(fd, model.host_url.clone()));
            orders.skip();
        }
        Msg::WebSocketChange(WebSocketChanged::WsMsg(ws_msg)) => match ws_msg {
            WsMsg::AvatarUrlChanged(user_id, avatar_url) => {
                if let Some(me) = model.user.as_mut() {
                    if me.id == user_id {
                        profile_page.avatar.url = Some(avatar_url.clone());
                    }
                }
            }
            _ => (),
        },
        Msg::ProjectChanged(Some(project)) => {
            profile_page.current_project.values = vec![project.id as u32];
        }
        Msg::PageChanged(PageChanged::Profile(ProfilePageChange::SubmitForm)) => {
            send_ws_msg(
                WsMsg::ProfileUpdate(
                    profile_page.email.value.clone(),
                    profile_page.name.value.clone(),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::StyledSelectChanged(
            FieldId::Profile(UsersFieldId::CurrentProject),
            StyledSelectChange::Changed(id),
        ) => {
            if let Some(up) = model
                .user_projects
                .iter()
                .find(|up| up.project_id == id as i32)
            {
                send_ws_msg(
                    WsMsg::UserProjectSetCurrent(up.id),
                    model.ws.as_ref(),
                    orders,
                );
            }
        }
        _ => (),
    }
}

fn init_load(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.user.is_none() {
        return;
    }
    enqueue_ws_msg(vec![WsMsg::ProjectIssuesRequest], model.ws.as_ref(), orders);
}

fn build_page_content(model: &mut Model) {
    let user = match model.user {
        Some(ref user) => user,
        _ => return,
    };
    model.page_content = PageContent::Profile(Box::new(ProfilePage::new(
        user,
        model
            .project
            .as_ref()
            .map(|p| vec![p.id])
            .unwrap_or_default(),
    )));
}

async fn update_avatar(data: FormData, host_url: String) -> Option<Msg> {
    let path = format!("{}/avatar/", host_url);
    let result = Request::new(path)
        .method(Method::Post)
        .body(data.into())
        .fetch()
        .await;
    let response = match result {
        Ok(r) => r,
        Err(_) => return None,
    };
    let text = response.text().await.ok()?;
    Some(Msg::AvatarUpdateFetched(text))
}
