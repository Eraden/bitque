use seed::{prelude::*, *};

use jirs_data::*;

use crate::model::{Icon, Model, Page};
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::{StyledButton, Variant};
use crate::shared::styled_input::StyledInput;
use crate::shared::{drag_ev, inner_layout, ToNode};
use crate::Msg;

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
            model.project_page.recently_updated_filter =
                !model.project_page.recently_updated_filter;
        }
        Msg::ProjectClearFilters => {
            let pp = &mut model.project_page;
            pp.active_avatar_filters = vec![];
            pp.recently_updated_filter = false;
            pp.only_my_filter = false;
        }
        Msg::IssueDragStarted(issue_id) => {
            model.project_page.dragged_issue_id = Some(issue_id);
        }
        Msg::IssueDragStopped(_) => {
            model.project_page.dragged_issue_id = None;
        }
        Msg::IssueDragOver(x, y) => {
            model.project_page.drag_point.x = x;
            model.project_page.drag_point.y = y;
        }
        Msg::IssueDropZone(status) => {
            match (
                model.project_page.dragged_issue_id.as_ref().cloned(),
                model.project.as_mut(),
            ) {
                (Some(issue_id), Some(project)) => {
                    let mut position = 0f64;
                    let mut found: Option<&mut Issue> = None;
                    for issue in project.issues.iter_mut() {
                        if issue.status == status {
                            position += 1f64;
                        }
                        if issue.id == issue_id {
                            found = Some(issue);
                            break;
                        }
                    }
                    if let Some(issue) = found {
                        issue.status = status.clone();
                        issue.list_position = position + 1f64;

                        let payload = UpdateIssuePayload {
                            title: Some(issue.title.clone()),
                            issue_type: Some(issue.issue_type.clone()),
                            status: Some(status.to_payload().to_string()),
                            priority: Some(issue.priority.clone()),
                            list_position: Some(issue.list_position),
                            description: Some(issue.description.clone()),
                            description_text: Some(issue.description_text.clone()),
                            estimate: Some(issue.estimate),
                            time_spent: Some(issue.time_spent),
                            time_remaining: Some(issue.time_remaining),
                            project_id: Some(issue.project_id),
                            users: Some(vec![]),
                            user_ids: Some(issue.user_ids.clone()),
                        };
                        orders.skip().perform_cmd(crate::api::update_issue(
                            model.host_url.clone(),
                            issue.id,
                            payload,
                        ));
                    }
                }
                _ => error!("Drag stopped before drop :("),
            }
        }
        Msg::IssueUpdateResult(fetched) => {
            crate::api_handlers::update_issue_response(&fetched, model);
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
        active: model.project_page.recently_updated_filter,
        text: Some("Recently Updated".to_string()),
        icon: None,
        on_click: Some(mouse_ev(Ev::Click, |_| Msg::ProjectToggleRecentlyUpdated)),
    }
    .into_node();

    let clear_all = match project_page.only_my_filter
        || project_page.recently_updated_filter
        || !project_page.active_avatar_filters.is_empty()
    {
        true => seed::button![
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
        .filter(|issue| status == issue.status)
        .map(|issue| project_issue(model, project, issue))
        .collect();
    let label = status.to_label();

    let send_status = status.clone();
    let drop_handler = drag_ev(Ev::Drop, move |ev| {
        ev.prevent_default();
        Msg::IssueDropZone(send_status)
    });
    let drag_over_handler = drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        Msg::NoOp
    });

    div![
        attrs![At::Class => "list";],
        div![
            attrs![At::Class => "title"],
            label,
            div![attrs![At::Class => "issuesCount"]]
        ],
        div![
            attrs![At::Class => "issues"; At::DropZone => "link"],
            drop_handler,
            drag_over_handler,
            issues
        ]
    ]
}

fn project_issue(model: &Model, project: &FullProject, issue: &Issue) -> Node<Msg> {
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

    let issue_type_icon = {
        let mut node = crate::shared::styled_icon(issue.issue_type.clone().into());
        node.add_style(
            St::Color,
            format!(
                "var(--{issue_type})",
                issue_type = issue.issue_type.to_string()
            ),
        );
        node
    };
    let priority_icon = {
        let icon = match issue.priority {
            IssuePriority::Low | IssuePriority::Lowest => Icon::ArrowDown,
            _ => Icon::ArrowUp,
        };
        let mut node = crate::shared::styled_icon(icon);
        node.add_style(St::Color, format!("var(--{})", issue.priority));
        node
    };

    let issue_id = issue.id;
    let drag_started = drag_ev(Ev::DragStart, move |_| Msg::IssueDragStarted(issue_id));
    let drag_stopped = drag_ev(Ev::DragEnd, move |_| Msg::IssueDragStopped(issue_id));

    let mut class_list = vec!["issue"];
    if Some(issue_id) == model.project_page.dragged_issue_id {
        class_list.push("hidden");
    }

    a![
        attrs![At::Class => "issueLink"],
        div![
            attrs![At::Class => class_list.join(" "), At::Draggable => true],
            drag_started,
            drag_stopped,
            p![attrs![At::Class => "title"], issue.title,],
            div![
                attrs![At::Class => "bottom"],
                div![
                    div![attrs![At::Class => "issueTypeIcon"], issue_type_icon],
                    div![attrs![At::Class => "issuePriorityIcon"], priority_icon]
                ],
                div![attrs![At::Class => "assignees"], avatars,],
            ]
        ]
    ]
}
