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
use crate::{FieldId, Msg, APP};

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
                m.top_select_filter = text;
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
        Msg::IssueDragStarted(issue_id) => {
            model.project_page.dragged_issue_id = Some(issue_id);
        }
        Msg::IssueDragStopped(_) => {
            model.project_page.dragged_issue_id = None;
        }
        Msg::ExchangePosition(issue_bellow_id) => {
            if model.project_page.drag_locked {
                return;
            }
            log!(issue_bellow_id);
            log!(model.project_page.dragged_issue_id);
            let dragged_id = match model.project_page.dragged_issue_id {
                Some(id) => id,
                _ => return,
            };

            let mut below = None;
            let mut dragged = None;
            let mut issues = vec![];
            std::mem::swap(&mut issues, &mut model.issues);

            for issue in issues.into_iter() {
                match issue.id {
                    id if id == issue_bellow_id => below = Some(issue),
                    id if id == dragged_id => dragged = Some(issue),
                    _ => model.issues.push(issue),
                };
            }

            let mut below = match below {
                Some(below) => below,
                _ => return,
            };
            let mut dragged = match dragged {
                Some(issue) => issue,
                _ => {
                    model.issues.push(below);
                    return;
                }
            };
            if dragged.status == below.status {
                std::mem::swap(&mut dragged.list_position, &mut below.list_position);
                below.status = dragged.status.clone();
            } else {
                below.list_position = model
                    .issues
                    .iter()
                    .map(|i| i.list_position)
                    .max()
                    .unwrap_or(0)
                    + 1;
                std::mem::swap(&mut dragged.list_position, &mut below.list_position);
                below.status = dragged.status.clone();
            }
            model.issues.push(below);
            model.issues.push(dragged);
            model
                .issues
                .sort_by(|a, b| a.list_position.cmp(&b.list_position));
            model.project_page.drag_locked = true;
            // log!(model.issues);
        }
        Msg::UnlockDragOver => {
            model.project_page.drag_locked = false;
        }
        Msg::IssueDropZone(status) => match model.project_page.dragged_issue_id.as_ref().cloned() {
            Some(issue_id) => {
                let mut position = 0;
                let mut found: Option<&mut Issue> = None;
                for issue in model.issues.iter_mut() {
                    if issue.status == status {
                        position += 1;
                    }
                    if issue.id == issue_id {
                        found = Some(issue);
                        break;
                    }
                }
                let issue = match found {
                    Some(i) => i,
                    _ => return,
                };

                issue.status = status.clone();
                issue.list_position = position + 1;

                let payload = UpdateIssuePayload {
                    title: Some(issue.title.clone()),
                    issue_type: Some(issue.issue_type.clone()),
                    status: Some(status),
                    priority: Some(issue.priority.clone()),
                    list_position: Some(issue.list_position),
                    description: Some(issue.description.clone()),
                    description_text: Some(issue.description_text.clone()),
                    estimate: Some(issue.estimate),
                    time_spent: Some(issue.time_spent),
                    time_remaining: Some(issue.time_remaining),
                    project_id: Some(issue.project_id),
                    user_ids: Some(issue.user_ids.clone()),
                };
                model.project_page.dragged_issue_id = None;
                send_ws_msg(WsMsg::IssueUpdateRequest(issue_id, payload));
            }
            _ => error!("Drag stopped before drop :("),
        },
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
        seed::set_timeout(
            Box::new(|| {
                let app = match unsafe { APP.as_mut().unwrap() }.write() {
                    Ok(app) => app,
                    _ => return,
                };
                app.update(Msg::UnlockDragOver);
            }),
            3000,
        );
        Msg::ExchangePosition(issue_id)
    });

    let class_list = vec!["issue"];
    if Some(issue_id) == model.project_page.dragged_issue_id {
        // class_list.push("hidden");
    }

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
