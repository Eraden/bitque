use seed::{prelude::*, *};

use jirs_data::UserRole;

use crate::model::{Model, Page};
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::{divider, ToNode};
use crate::Msg;

pub fn render(model: &Model) -> Node<Msg> {
    let project_icon = Node::from_html(include_str!("../../static/project-avatar.svg"));
    let project_info = match model.project.as_ref() {
        Some(project) => li![
            id!["projectInfo"],
            project_icon,
            div![
                attrs![At::Class => "projectTexts";],
                div![attrs![At::Class => "projectName";], project.name],
                div![
                    attrs![At::Class => "projectCategory";],
                    project.category.to_string()
                ]
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
    let mut links = vec![
        sidebar_link_item(model, "Releases", Icon::Shipping, None),
        sidebar_link_item(model, "Issue and Filters", Icon::Issues, None),
        sidebar_link_item(model, "Pages", Icon::Page, None),
        sidebar_link_item(model, "Reports", Icon::Reports, None),
        sidebar_link_item(model, "Components", Icon::Component, None),
    ];

    if model.user.as_ref().map(|u| u.user_role).unwrap_or_default() > UserRole::User {
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
            sidebar_link_item(
                model,
                "Project settings",
                Icon::Settings,
                Some(Page::ProjectSettings)
            ),
            li![divider()],
            links,
        ]
    ]
}

fn sidebar_link_item(model: &Model, name: &str, icon: Icon, page: Option<Page>) -> Node<Msg> {
    let path = page.map(|ref p| p.to_path()).unwrap_or_default();
    let mut class_list = vec!["linkItem".to_string(), icon.to_string()];
    if page.is_none() {
        class_list.push("notAllowed".to_string());
    };
    if Some(model.page) == page {
        class_list.push("active".to_string());
    }
    let icon_node = StyledIcon::build(icon).build().into_node();
    li![
        attrs![At::Class => class_list.join(" ")],
        a![
            attrs![At::Href => path],
            icon_node,
            div![attrs![At::Class => "linkText"], name],
        ]
    ]
}
