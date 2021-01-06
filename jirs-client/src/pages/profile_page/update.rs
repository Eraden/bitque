use {
    crate::{
        model::{Model, Page, PageContent},
        pages::profile_page::model::ProfilePage,
        shared::styled_select::StyledSelectChanged,
        ws::{board_load, send_ws_msg},
        FieldId, Msg, OperationKind, PageChanged, ProfilePageChange, ResourceKind,
        WebSocketChanged,
    },
    jirs_data::{ProjectId, User, UsersFieldId, WsMsg},
    seed::prelude::{Method, Orders, Request},
    web_sys::FormData,
};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, Some(_))
        | Msg::ChangePage(Page::Profile) => {
            board_load(model, orders);
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
        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::AvatarUrlChanged(
            user_id,
            avatar_url,
        ))) => {
            if let Some(User { id, .. }) = model.user.as_mut() {
                if *id == user_id {
                    profile_page.avatar.url = Some(avatar_url);
                }
            }
        }
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
            StyledSelectChanged::Changed(Some(id)),
        ) => {
            if let Some(up) = model
                .user_projects
                .iter()
                .find(|up| up.project_id == id as ProjectId)
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