use seed::fetch::{FetchObject, FetchResult, ResponseWithDataResult};
use seed::{prelude::*, *};
use serde::Deserialize;

use jirs_data::FullProjectResponse;

use crate::model::{Icon, Model, Page};
use crate::Msg;

pub fn navbar_left(model: &Model) -> Node<Msg> {
    let mut logo_svg = Node::from_html(include_str!("../static/logo.svg"));

    aside![
        id!["navbar-left"],
        a![
            attrs![At::Class => "logoLink", At::Href => "/"],
            div![attrs![At::Class => "styledLogo"], logo_svg]
        ],
        navbar_left_item(model, "Search issues", Icon::Search),
        navbar_left_item(model, "Create Issue", Icon::Plus),
        div![
            attrs![At::Class => "bottom"],
            about_tooltip(model, navbar_left_item(model, "About", Icon::Help))
        ]
    ]
}

fn navbar_left_item(_model: &Model, text: &str, logo: Icon) -> Node<Msg> {
    div![
        attrs![At::Class => "item"],
        i![attrs![At::Class => format!("styledIcon {}", logo)]],
        span![attrs![At::Class => "itemText"], text]
    ]
}

pub fn about_tooltip(_model: &Model, children: Node<Msg>) -> Node<Msg> {
    div![attrs![At::Class => "aboutTooltip"], children]
}

pub fn styled_tooltip() -> Node<Msg> {
    div![attrs![At::Class => "styledTooltip"]]
}

pub fn sidebar(model: &Model) -> Node<Msg> {
    let project_icon = Node::from_html(include_str!("../static/project-avatar.svg"));
    let project_info = match model.project.as_ref() {
        Some(project) => li![
            id!["projectInfo"],
            project_icon,
            div![
                attrs![At::Class => "projectTexts";],
                div![attrs![At::Class => "projectName";], project.name],
                div![attrs![At::Class => "projectCategory";], project.category]
            ],
        ],
        _ => li![
            id!["projectInfo"],
            div![
                attrs![At::Class => "projectTexts";],
                div![attrs![At::Class => "projectName";], ""],
                div![attrs![At::Class => "projectCategory";], ""]
            ],
        ],
    };
    nav![
        id!["sidebar"],
        ul![
            project_info,
            sidebar_link_item(model, "Kanban Board", Icon::Board, Some(Page::Project)),
            sidebar_link_item(
                model,
                "Project settings",
                Icon::Settings,
                Some(Page::ProjectSettings)
            ),
            li![divider()],
            sidebar_link_item(model, "Releases", Icon::Shipping, None),
            sidebar_link_item(model, "Issue and Filters", Icon::Issues, None),
            sidebar_link_item(model, "Pages", Icon::Page, None),
            sidebar_link_item(model, "Reports", Icon::Reports, None),
            sidebar_link_item(model, "Components", Icon::Component, None),
        ]
    ]
}

fn sidebar_link_item(model: &Model, name: &str, icon: Icon, page: Option<Page>) -> Node<Msg> {
    let path = page.map(|ref p| p.to_path()).unwrap_or_default();
    let mut class_list = vec!["linkItem".to_string(), icon.to_string()];
    let item_class = if let None = page {
        class_list.push("notAllowed".to_string())
    };
    if Some(model.page) == page {
        class_list.push("active".to_string());
    }
    li![
        attrs![At::Class => class_list.join(" ")],
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
    article![
        id!["inner-layout"],
        navbar_left(model),
        sidebar(model),
        children,
    ]
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
