use seed::{prelude::*, *};

use crate::model::{Icon, Model, Page};
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::{StyledButton, Variant};
use crate::shared::styled_input::StyledInput;
use crate::shared::{host_client, inner_layout, ToNode};
use crate::Msg;
use jirs_data::{FullProject, Issue, IssuePriority, IssueStatus, IssueType};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(Page::Project) => {
            orders
                .skip()
                .perform_cmd(crate::api::fetch_current_project(model.host_url.clone()));
            orders
                .skip()
                .perform_cmd(crate::api::fetch_current_user(model.host_url.clone()));
        }
        Msg::ToggleAboutTooltip => {
            model.project_page.about_tooltip_visible = !model.project_page.about_tooltip_visible;
        }
        Msg::ProjectTextFilterChanged(text) => {
            model.project_page.text_filter = text;
        }
        Msg::ProjectAvatarFilterChanged(user_id, active) => match active {
            true => {
                model.project_page.active_avatar_filters = model
                    .project_page
                    .active_avatar_filters
                    .iter()
                    .filter(|id| **id != user_id)
                    .map(|id| *id)
                    .collect();
            }
            false => {
                model.project_page.active_avatar_filters.push(user_id);
            }
        },
        Msg::ProjectToggleOnlyMy => {
            model.project_page.only_my_filter = !model.project_page.only_my_filter;
        }
        Msg::ProjectToggleRecentlyUpdated => {
            model.project_page.recenlty_updated_filter =
                !model.project_page.recenlty_updated_filter;
        }
        Msg::ProjectClearFilters => {
            let pp = &mut model.project_page;
            pp.active_avatar_filters = vec![];
            pp.recenlty_updated_filter = false;
            pp.only_my_filter = false;
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let project_section = vec![
        breadcrumbs(model),
        header(),
        project_board_filters(model),
        project_board_lists(model),
    ];

    inner_layout(model, "projectPage", project_section)
}

fn breadcrumbs(model: &Model) -> Node<Msg> {
    let project_name = model
        .project
        .as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_default();
    div![
        attrs![At::Class => "breadcrumbsContainer"],
        span!["Projects"],
        span![attrs![At::Class => "breadcrumbsDivider"], "/"],
        span![project_name],
        span![attrs![At::Class => "breadcrumbsDivider"], "/"],
        span!["Kanban Board"]
    ]
}

fn header() -> Node<Msg> {
    let button = StyledButton {
        variant: Variant::Secondary,
        icon_only: false,
        disabled: false,
        active: false,
        text: Some("Github Repo".to_string()),
        icon: Some(Icon::Github),
        on_click: None,
    }
    .into_node();
    div![
        id!["projectBoardHeader"],
        div![id!["boardName"], "Kanban board"],
        a![
            attrs![At::Href => "https://gitlab.com/adrian.wozniak/jirs", At::Target => "__blank", At::Rel => "noreferrer noopener"],
            button
        ]
    ]
}

fn project_board_filters(model: &Model) -> Node<Msg> {
    let search_input = StyledInput {
        icon: Some(Icon::Search),
        id: Some("searchInput".to_string()),
        valid: true,
        on_change: input_ev(Ev::Change, |value| Msg::ProjectTextFilterChanged(value)),
    }
    .into_node();

    let project_page = &model.project_page;

    let only_my = StyledButton {
        variant: Variant::Empty,
        icon_only: false,
        disabled: false,
        active: model.project_page.only_my_filter,
        text: Some("Only My Issues".to_string()),
        icon: None,
        on_click: Some(mouse_ev(Ev::Click, |_| Msg::ProjectToggleOnlyMy)),
    }
    .into_node();

    let recently_updated = StyledButton {
        variant: Variant::Empty,
        icon_only: false,
        disabled: false,
        active: model.project_page.recenlty_updated_filter,
        text: Some("Recently Updated".to_string()),
        icon: None,
        on_click: Some(mouse_ev(Ev::Click, |_| Msg::ProjectToggleRecentlyUpdated)),
    }
    .into_node();

    let clear_all = match project_page.only_my_filter
        || project_page.recenlty_updated_filter
        || !project_page.active_avatar_filters.is_empty()
    {
        true => button![
            id!["clearAllFilters"],
            "Clear all",
            mouse_ev(Ev::Click, |_| Msg::ProjectClearFilters),
        ],
        false => empty![],
    };

    div![
        id!["projectBoardFilters"],
        search_input,
        avatars_filters(model),
        only_my,
        recently_updated,
        clear_all
    ]
}

fn avatars_filters(model: &Model) -> Node<Msg> {
    let project = match model.project.as_ref() {
        Some(p) => p,
        _ => return empty![],
    };
    let active_avatar_filters = &model.project_page.active_avatar_filters;
    let avatars: Vec<Node<Msg>> = project
        .users
        .iter()
        .map(|user| {
            let mut class_list = vec!["avatarIsActiveBorder"];
            let user_id = user.id;
            let active = active_avatar_filters.contains(&user_id);
            if active {
                class_list.push("isActive");
            }
            let styled_avatar = StyledAvatar {
                avatar_url: user.avatar_url.clone(),
                size: 32,
                name: user.name.clone(),
                on_click: Some(mouse_ev(Ev::Click, move |_| {
                    Msg::ProjectAvatarFilterChanged(user_id, active)
                })),
            }
            .into_node();
            div![attrs![At::Class => class_list.join(" ")], styled_avatar]
        })
        .collect();

    div![id!["avatars"], avatars]
}

fn project_board_lists(model: &Model) -> Node<Msg> {
    use jirs_data::IssueStatus;

    div![
        id!["projectBoardLists"],
        project_issue_list(model, IssueStatus::Backlog),
        project_issue_list(model, IssueStatus::Selected),
        project_issue_list(model, IssueStatus::InProgress),
        project_issue_list(model, IssueStatus::Done),
    ]
}

fn project_issue_list(model: &Model, status: jirs_data::IssueStatus) -> Node<Msg> {
    let project = match model.project.as_ref() {
        Some(p) => p,
        _ => return empty![],
    };
    let issues: Vec<Node<Msg>> = project
        .issues
        .iter()
        .filter(|issue| status.match_name(issue.status.as_str()))
        .map(|issue| project_issue(model, project, issue))
        .collect();
    let label = status.to_label();
    div![
        attrs![At::Class => "list"],
        div![
            attrs![At::Class => "title"],
            label,
            div![attrs![At::Class => "issuesCount"]]
        ],
        div![attrs![At::Class => "issues"], issues]
    ]
}

fn project_issue(_model: &Model, project: &FullProject, issue: &Issue) -> Node<Msg> {
    let avatars: Vec<Node<Msg>> = project
        .users
        .iter()
        .filter(|user| issue.user_ids.contains(&user.id))
        .map(|user| {
            StyledAvatar {
                avatar_url: user.avatar_url.clone(),
                size: 24,
                name: user.name.clone(),
                on_click: None,
            }
            .into_node()
        })
        .collect();
    let mut issue_type_icon = match issue.issue_type.parse::<IssueType>() {
        Ok(icon) => {
            let mut node = crate::shared::styled_icon(icon.into());
            node.add_style(
                St::Color,
                format!("var(--{issue_type})", issue_type = issue.issue_type),
            );
            node
        }
        Err(e) => span![format!("{}", e)],
    };
    let priority_icon = match issue.priority.parse::<IssuePriority>() {
        Ok(IssuePriority::Low) | Ok(IssuePriority::Lowest) => {
            crate::shared::styled_icon(Icon::ArrowDown)
        }
        Ok(_) => crate::shared::styled_icon(Icon::ArrowUp),
        Err(e) => span![e.clone()],
    };
    a![
        attrs![At::Class => "issueLink"],
        div![
            attrs![At::Class => "issue"],
            p![attrs![At::Class => "title"], issue.title,],
            div![
                attrs![At::Class => "bottom"],
                div![
                    // <IssueTypeIcon type={issue.type} />
                    div![attrs![At::Class => "issueTypeIcon"], issue_type_icon],
                    // <IssuePriorityIcon priority={issue.priority} top={-1} left={4} />
                    div![attrs![At::Class => "issuePriorityIcon"], priority_icon]
                ],
                div![attrs![At::Class => "assignees"], avatars,],
            ]
        ]
    ]
}
