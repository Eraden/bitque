use seed::{prelude::*, *};

use jirs_data::{UserRole, WsMsg};

use crate::model::{Model, Page};
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::{divider, ToNode};
use crate::ws::enqueue_ws_msg;
use crate::{Msg, WebSocketChanged};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if let Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(Ok(_)))) = msg {
        enqueue_ws_msg(
            vec![
                WsMsg::UserProjectsLoad,
                WsMsg::ProjectsLoad,
                WsMsg::ProjectUsersLoad,
                WsMsg::MessagesLoad,
            ],
            model.ws.as_ref(),
            orders,
        );
    }
}

pub fn render(model: &Model) -> Node<Msg> {
    let project_icon = Node::from_html(include_str!("../../static/project-avatar.svg"));
    let project_info = match model.project.as_ref() {
        Some(project) => li![
            id!["projectInfo"],
            project_icon,
            div![
                class!["projectTexts"],
                div![class!["projectName"], project.name.as_str()],
                div![class!["projectCategory"], project.category.to_string()]
            ],
        ],
        _ => li![
            id!["projectInfo"],
            div![
                class!["projectTexts"],
                div![class!["projectName"], ""],
                div![class!["projectCategory"], ""]
            ],
        ],
    };
    let mut links = vec![];

    if model.current_user_role() > UserRole::User {
        links.push(sidebar_link_item(
            model,
            "Project settings",
            Icon::Settings,
            Some(Page::ProjectSettings),
        ));
    }

    links.extend(vec![
        li![divider()],
        sidebar_link_item(model, "Releases", Icon::Shipping, None),
        sidebar_link_item(model, "Issue and Filters", Icon::Issues, None),
        sidebar_link_item(model, "Pages", Icon::Page, None),
        sidebar_link_item(model, "Reports", Icon::Reports, Some(Page::Reports)),
        sidebar_link_item(model, "Components", Icon::Component, None),
    ]);

    if model.current_user_role() > UserRole::User {
        links.push(sidebar_link_item(
            model,
            "Users",
            Icon::Cop,
            Some(Page::Users),
        ));
    }

    nav![
        id!["sidebar"],
        ul![
            project_info,
            sidebar_link_item(model, "Kanban Board", Icon::Board, Some(Page::Project)),
            links,
        ]
    ]
}

fn sidebar_link_item(model: &Model, name: &str, icon: Icon, page: Option<Page>) -> Node<Msg> {
    let path = page.map(|ref p| p.to_path()).unwrap_or_default();
    let mut class_list = vec![];
    if page.is_none() {
        class_list.push("notAllowed");
    };
    if Some(model.page) == page {
        class_list.push("active");
    }
    let icon_node = StyledIcon::build(icon).build().into_node();
    li![
        class!["linkItem"],
        class![icon.to_str()],
        a![
            attrs![At::Href => path],
            icon_node,
            div![attrs![At::Class => "linkText"], name],
        ]
    ]
}
