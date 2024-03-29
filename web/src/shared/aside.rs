use jirs_data::msg::{WsMsgMessage, WsMsgProject};
use jirs_data::{UserRole, WsMsg};
use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::model::{Model, Page};
use crate::shared::divider;
use crate::ws::enqueue_ws_msg;
use crate::{Msg, OperationKind, ResourceKind};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if let Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, _) = msg {
        enqueue_ws_msg(
            vec![
                WsMsg::UserProjectsLoad,
                WsMsg::Project(WsMsgProject::ProjectsLoad),
                WsMsg::Project(WsMsgProject::ProjectUsersLoad),
                WsMsg::Message(WsMsgMessage::MessagesLoad),
            ],
            model.ws.as_ref(),
            orders,
        );
        orders.skip();
    }
}

pub fn render(model: &Model) -> Node<Msg> {
    let project_icon = crate::images::project_avatar::render();

    let project_info = match model.project.as_ref() {
        Some(project) => li![
            id!["projectInfo"],
            project_icon,
            div![
                C!["projectTexts"],
                div![C!["projectName"], project.name.as_str()],
                div![C!["projectCategory"], project.category.to_string()]
            ],
        ],
        _ => li![
            id!["projectInfo"],
            div![
                C!["projectTexts"],
                div![C!["projectName"], ""],
                div![C!["projectCategory"], ""]
            ],
        ],
    };

    nav![
        id!["sidebar"],
        ul![
            project_info,
            sidebar_link_item(model, "Kanban Board", Icon::Board, Some(Page::Project)),
            project_settings(model),
            li![divider()],
            sidebar_link_item(
                model,
                "Issue and Filters",
                Icon::Issues,
                Some(Page::IssuesAndFilters)
            ),
            sidebar_link_item(model, "Reports", Icon::Reports, Some(Page::Reports)),
            sidebar_link_item(model, "Epics", Icon::Component, Some(Page::Epics)),
            users_link(model)
        ]
    ]
}

#[inline]
fn project_settings(model: &Model) -> Node<Msg> {
    if model.current_user_role() <= UserRole::User {
        return Node::Empty;
    }

    sidebar_link_item(
        model,
        "Project settings",
        Icon::Settings,
        Some(Page::ProjectSettings),
    )
}

#[inline]
fn users_link(model: &Model) -> Node<Msg> {
    if model.current_user_role() <= UserRole::User {
        return Node::Empty;
    }

    sidebar_link_item(model, "Users", Icon::Cop, Some(Page::Users))
}

#[inline]
fn sidebar_link_item(model: &Model, name: &str, icon: Icon, page: Option<Page>) -> Node<Msg> {
    let path = page.map(|ref p| p.to_path()).unwrap_or_default();
    let allow_flag = if page.is_none() {
        Some(C!["notAllowed"])
    } else {
        None
    };
    let active_flag = page.filter(|p| *p == model.page).map(|_| C!["active"]);
    let icon_node = StyledIcon::from(icon).render();
    let on_click = page.map(|p| {
        mouse_ev("click", move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            seed::Url::new()
                .set_path(p.to_path().split('/').filter(|s| !s.is_empty()))
                .go_and_push();
            Msg::ChangePage(p)
        })
    });

    li![
        C!["linkItem", icon.to_str()],
        active_flag,
        allow_flag,
        a![
            attrs![At::Href => path],
            on_click,
            icon_node,
            div![C!["linkText"], name],
        ]
    ]
}
