use seed::{prelude::*, *};

use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{ModalType, Model, Page};
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::StyledSelectChange;
use crate::shared::{drag_ev, inner_layout, ToNode};
use crate::{FieldId, Msg};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(Page::Project)
        | Msg::ChangePage(Page::AddIssue)
        | Msg::ChangePage(Page::EditIssue(..))
        | Msg::WsMsg(WsMsg::AuthorizeLoaded(Ok(_))) => {
            send_ws_msg(jirs_data::WsMsg::ProjectRequest);
            send_ws_msg(jirs_data::WsMsg::ProjectIssuesRequest);
            send_ws_msg(jirs_data::WsMsg::ProjectUsersRequest);
        }
        Msg::WsMsg(WsMsg::IssueUpdated(issue)) => {
            let mut old: Vec<Issue> = vec![];
            std::mem::swap(&mut old, &mut model.issues);
            for is in old {
                if is.id == issue.id {
                    model.issues.push(issue.clone())
                } else {
                    model.issues.push(is);
                }
            }
        }
        Msg::WsMsg(WsMsg::IssueDeleted(id)) => {
            let mut old: Vec<Issue> = vec![];
            std::mem::swap(&mut old, &mut model.issues);
            for is in old {
                if is.id != id {
                    model.issues.push(is);
                }
            }
            orders.skip().send_msg(Msg::ModalDropped);
        }
        Msg::ToggleAboutTooltip => {
            model.project_page.about_tooltip_visible = !model.project_page.about_tooltip_visible;
        }
        Msg::StyledSelectChanged(
            FieldId::IssueTypeEditModalTop,
            StyledSelectChange::Text(text),
        ) => {
            let modal = model
                .modals
                .iter_mut()
                .filter_map(|modal| match modal {
                    ModalType::EditIssue(_, modal) => Some(modal),
                    _ => None,
                })
                .last();
            if let Some(m) = modal {
                m.top_type_state.text_filter = text;
            }
        }
        Msg::InputChanged(FieldId::TextFilterBoard, text) => {
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
        Msg::IssueDragStarted(issue_id) => crate::ws::issue::drag_started(issue_id, model),
        Msg::IssueDragStopped(_) => {
            model.project_page.dragged_issue_id = None;
        }
        Msg::ExchangePosition(issue_bellow_id) => {
            crate::ws::issue::exchange_position(issue_bellow_id, model)
        }
        Msg::IssueDragOverStatus(status) => crate::ws::issue::change_status(status, model),
        Msg::IssueDropZone(status) => crate::ws::issue::dropped(status, model),
        Msg::DeleteIssue(issue_id) => {
            send_ws_msg(jirs_data::WsMsg::IssueDeleteRequest(issue_id));
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

    inner_layout(
        model,
        "projectPage",
        project_section,
        crate::modal::view(model),
    )
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
    let button = StyledButton::build()
        .secondary()
        .text("Github Repo".to_string())
        .icon(Icon::Github)
        .build()
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
    let search_input = StyledInput::build(FieldId::TextFilterBoard)
        .icon(Icon::Search)
        .valid(true)
        .build()
        .into_node();

    let project_page = &model.project_page;

    let only_my = StyledButton::build()
        .empty()
        .active(model.project_page.only_my_filter)
        .text("Only My Issues")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ProjectToggleOnlyMy))
        .build()
        .into_node();

    let recently_updated = StyledButton::build()
        .empty()
        .text("Recently Updated")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ProjectToggleRecentlyUpdated))
        .build()
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
    let active_avatar_filters = &model.project_page.active_avatar_filters;
    let avatars: Vec<Node<Msg>> = model
        .users
        .iter()
        .map(|user| {
            let mut class_list = vec!["avatarIsActiveBorder"];
            let user_id = user.id;
            let active = active_avatar_filters.contains(&user_id);
            if active {
                class_list.push("isActive");
            }
            let styled_avatar = StyledAvatar::build()
                .avatar_url(user.avatar_url.as_ref().cloned().unwrap_or_default())
                .on_click(mouse_ev(Ev::Click, move |_| {
                    Msg::ProjectAvatarFilterChanged(user_id, active)
                }))
                .name(user.name.as_str())
                .build()
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
    let issues: Vec<Node<Msg>> = model
        .issues
        .iter()
        .filter(|issue| {
            status == issue.status
                && (model.project_page.text_filter.is_empty()
                    || issue
                        .title
                        .contains(model.project_page.text_filter.as_str()))
        })
        .map(|issue| project_issue(model, issue))
        .collect();
    let label = status.to_label();

    let send_status = status.clone();
    let drop_handler = drag_ev(Ev::Drop, move |ev| {
        ev.prevent_default();
        Msg::IssueDropZone(send_status)
    });

    let send_status = status.clone();
    let drag_over_handler = drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        Msg::IssueDragOverStatus(send_status)
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

fn project_issue(model: &Model, issue: &Issue) -> Node<Msg> {
    let avatars: Vec<Node<Msg>> = model
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
        StyledIcon::build(issue.issue_type.clone().into())
            .add_style(format!(
                "color: var(--{issue_type})",
                issue_type = issue.issue_type.to_string()
            ))
            .build()
            .into_node()
    };
    let priority_icon = {
        let icon = match issue.priority {
            IssuePriority::Low | IssuePriority::Lowest => Icon::ArrowDown,
            _ => Icon::ArrowUp,
        };
        StyledIcon::build(icon)
            .add_style(format!("color: var(--{})", issue.priority))
            .build()
            .into_node()
    };

    let issue_id = issue.id;
    let drag_started = drag_ev(Ev::DragStart, move |_| Msg::IssueDragStarted(issue_id));
    let drag_stopped = drag_ev(Ev::DragEnd, move |_| Msg::IssueDragStopped(issue_id));
    let drag_over_handler = drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Msg::ExchangePosition(issue_id)
    });

    let class_list = vec!["issue"];

    let href = format!("/issues/{id}", id = issue_id);

    a![
        drag_started,
        attrs![At::Class => "issueLink"; At::Href => href],
        div![
            attrs![At::Class => class_list.join(" "), At::Draggable => true],
            drag_stopped,
            drag_over_handler,
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
