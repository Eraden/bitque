use seed::fetch::{FetchObject, ResponseWithDataResult};
use seed::{prelude::*, *};

use jirs_data::FullProjectResponse;
use styled_button::*;

use crate::model::{Icon, Model, Page};
use crate::Msg;

pub mod styled_button;
pub mod styled_tooltip;

pub fn navbar_left(model: &Model) -> Vec<Node<Msg>> {
    let logo_svg = Node::from_html(include_str!("../../static/logo.svg"));

    vec![
        about_tooltip_popup(model),
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
                about_tooltip(model, navbar_left_item(model, "About", Icon::Help)),
            ],
        ],
    ]
}

fn navbar_left_item(_model: &Model, text: &str, logo: Icon) -> Node<Msg> {
    div![
        attrs![At::Class => "item"],
        i![attrs![At::Class => format!("styledIcon {}", logo)]],
        span![attrs![At::Class => "itemText"], text]
    ]
}

pub fn about_tooltip(model: &Model, children: Node<Msg>) -> Node<Msg> {
    div![
        attrs![At::Class => "aboutTooltip"],
        ev(Ev::Click, |_| Msg::ToggleAboutTooltip),
        children
    ]
}

fn about_tooltip_popup(model: &Model) -> Node<Msg> {
    styled_tooltip::StyledTooltip {
        visible: model.about_tooltip_visible,
        class_name: "aboutTooltipPopup".to_string(),
        children: div![
        ev(Ev::Click, |_| Msg::ToggleAboutTooltip),
        attrs![At::Class => "feedbackDropdown"],
        div![
            attrs![At::Class => "feedbackImageCont"],
            img![attrs![At::Src => "/feedback.png", At::Class => "feedbackImage"]]
        ],
        div![
            attrs![At::Class => "feedbackParagraph"],
            "This simplified Jira clone is built with Seed.rs on the front-end and Actix-Web on the back-end."
        ],
        div![
            attrs![At::Class => "feedbackParagraph"],
            "Read more on my website or reach out via ",
            a![
                attrs![At::Href => "mailto:adrian.wozniak@ita-prog.pl"],
                strong!["adrian.wozniak@ita-prog.pl"]
            ]
        ],
        a![
            attrs![
                At::Href => "https://gitlab.com/adrian.wozniak/jirs",
                At::Target => "_blank",
                At::Rel => "noreferrer noopener",
            ],
            StyledButton {
                text: Some("Visit Website".to_string()),
                variant: Variant::Primary,
                disabled: false,
                active: false,
                icon_only: false,
                icon: None,
            }.into_node(),
      ],
      a![
        id!["about-github-button"],
        attrs![
            At::Href => "https://gitlab.com/adrian.wozniak/jirs",
            At::Target => "_blank",
            At::Rel => "noreferrer noopener",
        ],
        StyledButton {
            text: Some("Github Repo".to_string()),
            variant: Variant::Secondary,
            disabled: false,
            active: false,
            icon_only: false,
            icon: Some(Icon::Github),
        }.into_node()
      ]
    ],
    }.into_node()
}

pub fn sidebar(model: &Model) -> Node<Msg> {
    let project_icon = Node::from_html(include_str!("../../static/project-avatar.svg"));
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
            styled_icon(icon),
            div![attrs![At::Class => "linkText"], name],
        ]
    ]
}

pub fn styled_icon(icon: Icon) -> Node<Msg> {
    i![attrs![At::Class => format!("styledIcon {}", icon)], ""]
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
