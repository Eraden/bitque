use crate::model::{Icon, Model};
use crate::Msg;
use jirs_data::FullProjectResponse;
use seed::fetch::{FetchObject, FetchResult, ResponseWithDataResult};
use seed::{prelude::*, *};
use serde::Deserialize;

pub fn sidebar(model: &Model) -> Node<Msg> {
    let project_info = match model.project.as_ref() {
        Some(project) => li![
            id!["projectInfo"],
            div![
                attrs![At::Class => ".projectTexts";],
                div![attrs![At::Class => ".projectName";], project.name],
                div![attrs![At::Class => ".projectCategory";], project.category]
            ],
        ],
        _ => li![
            id!["projectInfo"],
            div![
                attrs![At::Class => ".projectTexts";],
                div![attrs![At::Class => ".projectName";], ""],
                div![attrs![At::Class => ".projectCategory";], ""]
            ],
        ],
    };
    nav![
        id!["sidebar"],
        ul![
            project_info,
            sidebar_link_item(model, "Kanban Board", Icon::Board, "/board"),
            sidebar_link_item(
                model,
                "Project settings",
                Icon::Settings,
                "/project-settings"
            ),
            li![divider()]
        ]
    ]
}

fn sidebar_link_item(_model: &Model, name: &str, icon: Icon, path: &str) -> Node<Msg> {
    let item_class = match path {
        "" => format!("linkItem notAllowed {}", icon),
        _ => format!("linkItem {}", icon),
    };
    li![
        attrs![At::Class => item_class],
        a![
            attrs![At::Href => path],
            i![attrs![At::Class => format!("styledIcon {}", icon)], ""],
            div![attrs![At::Class => "linkText"], name],
        ]
    ]
}

pub fn divider() -> Node<Msg> {
    div![attrs![At::Class => "divider"], ""]
}

pub fn inner_layout(model: &Model, children: Node<Msg>) -> Node<Msg> {
    article![id!["inner-layout"], sidebar(model), children,]
}

pub fn host_client(host_url: String, path: &str) -> Result<Request, String> {
    let url = format!("{}{}", host_url, path);
    let w = window();
    let store = match w.local_storage() {
        Ok(Some(store)) => store,
        _ => return Err("Local storage is not available".to_string()),
    };
    let token = match store.get_item("authToken") {
        Ok(Some(s)) => s,
        _ => "".to_string(),
    };
    Ok(Request::new(url).header("Authorization", format!("Bearer {}", token).as_str()))
}

pub fn update(msg: &Msg, model: &mut crate::model::Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CurrentProjectResult(FetchObject {
            result:
                Ok(ResponseWithDataResult {
                    data: Ok(body),
                    status,
                    ..
                }),
            ..
        }) if status.is_ok() => match serde_json::from_str::<'_, FullProjectResponse>(body) {
            Ok(project_response) => {
                model.project = Some(project_response.project);
            }
            _ => (),
        },
        Msg::CurrentUserResult(FetchObject {
            result:
                Ok(ResponseWithDataResult {
                    data: Ok(body),
                    status,
                    ..
                }),
            ..
        }) if status.is_ok() => match serde_json::from_str::<'_, jirs_data::User>(body) {
            Ok(user) => {
                model.user = Some(user);
            }
            _ => (),
        },
        _ => (),
    }
}
