use chrono::NaiveDateTime;
use seed::{prelude::*, *};

use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{ModalType, Model, Page, PageContent, ProjectPage};
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::StyledSelectChange;
use crate::shared::{drag_ev, inner_layout, ToNode};
use crate::{BoardPageChange, EditIssueModalSection, FieldId, Msg, PageChanged};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    if model.user.is_none() {
        return;
    }

    match msg {
        Msg::ChangePage(Page::Project)
        | Msg::ChangePage(Page::AddIssue)
        | Msg::ChangePage(Page::EditIssue(..)) => {
            model.page_content = PageContent::Project(Box::new(ProjectPage::default()));
        }
        _ => (),
    }

    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };

    match msg {
        Msg::WsMsg(WsMsg::AuthorizeLoaded(..))
        | Msg::ChangePage(Page::Project)
        | Msg::ChangePage(Page::AddIssue)
        | Msg::ChangePage(Page::EditIssue(..)) => {
            send_ws_msg(jirs_data::WsMsg::ProjectRequest);
            send_ws_msg(jirs_data::WsMsg::ProjectIssuesRequest);
            send_ws_msg(jirs_data::WsMsg::ProjectUsersRequest);
            send_ws_msg(jirs_data::WsMsg::IssueStatusesRequest);
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
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
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
        Msg::StrInputChanged(FieldId::TextFilterBoard, text) => {
            project_page.text_filter = text;
        }
        Msg::ProjectAvatarFilterChanged(user_id, active) => {
            if active {
                project_page.active_avatar_filters = project_page
                    .active_avatar_filters
                    .iter()
                    .filter_map(|id| if *id != user_id { Some(*id) } else { None })
                    .collect();
            } else {
                project_page.active_avatar_filters.push(user_id);
            }
        }
        Msg::ProjectToggleOnlyMy => {
            project_page.only_my_filter = !project_page.only_my_filter;
        }
        Msg::ProjectToggleRecentlyUpdated => {
            project_page.recently_updated_filter = !project_page.recently_updated_filter;
        }
        Msg::ProjectClearFilters => {
            project_page.active_avatar_filters = vec![];
            project_page.recently_updated_filter = false;
            project_page.only_my_filter = false;
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStarted(issue_id))) => {
            crate::ws::issue::drag_started(issue_id, model)
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStopped(_))) => {
            crate::ws::issue::sync(model);
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::ExchangePosition(
            issue_bellow_id,
        ))) => crate::ws::issue::exchange_position(issue_bellow_id, model),
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragOverStatus(status))) => {
            crate::ws::issue::change_status(status, model)
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDropZone(_status))) => {
            crate::ws::issue::sync(model)
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::DragLeave(_id))) => {
            project_page.issue_drag.clear_last();
        }
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
    let project_page = match &model.page_content {
        PageContent::Project(page_content) => page_content,
        _ => return empty![],
    };

    let search_input = StyledInput::build(FieldId::TextFilterBoard)
        .icon(Icon::Search)
        .valid(true)
        .value(project_page.text_filter.as_str())
        .build()
        .into_node();

    let only_my = StyledButton::build()
        .empty()
        .active(project_page.only_my_filter)
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

    let clear_all = if project_page.only_my_filter
        || project_page.recently_updated_filter
        || !project_page.active_avatar_filters.is_empty()
    {
        seed::button![
            id!["clearAllFilters"],
            "Clear all",
            mouse_ev(Ev::Click, |_| Msg::ProjectClearFilters),
        ]
    } else {
        empty![]
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
    let project_page = match &model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return empty![],
    };
    let active_avatar_filters = &project_page.active_avatar_filters;
    let avatars: Vec<Node<Msg>> = model
        .users
        .iter()
        .enumerate()
        .map(|(idx, user)| {
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
                .user_index(idx)
                .build()
                .into_node();
            div![attrs![At::Class => class_list.join(" ")], styled_avatar]
        })
        .collect();

    div![id!["avatars"], avatars]
}

fn project_board_lists(model: &Model) -> Node<Msg> {
    let columns: Vec<Node<Msg>> = model
        .issue_statuses
        .iter()
        .map(|is| project_issue_list(model, is))
        .collect();
    div![id!["projectBoardLists"], columns]
}

fn project_issue_list(model: &Model, status: &jirs_data::IssueStatus) -> Node<Msg> {
    let project_page = match &model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return empty![],
    };
    let ids: Vec<IssueId> = if project_page.recently_updated_filter {
        let mut v: Vec<(IssueId, NaiveDateTime)> = model
            .issues
            .iter()
            .map(|issue| (issue.id, issue.updated_at))
            .collect();
        v.sort_by(|(_, a_time), (_, b_time)| a_time.cmp(b_time));
        if v.len() > 10 { v[0..10].to_vec() } else { v }
            .into_iter()
            .map(|(id, _)| id)
            .collect()
    } else {
        model.issues.iter().map(|issue| issue.id).collect()
    };
    let issues: Vec<Node<Msg>> = model
        .issues
        .iter()
        .filter(|issue| {
            issue_filter_status(issue, status)
                && issue_filter_with_text(issue, project_page.text_filter.as_str())
                && issue_filter_with_only_my(issue, project_page.only_my_filter, &model.user)
                && issue_filter_with_only_recent(issue, ids.as_slice())
        })
        .map(|issue| project_issue(model, issue))
        .collect();
    let label = status.name.clone();

    let send_status = status.id;
    let drop_handler = drag_ev(Ev::Drop, move |ev| {
        ev.prevent_default();
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDropZone(
            send_status,
        )))
    });

    let send_status = status.id;
    let drag_over_handler = drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragOverStatus(
            send_status,
        )))
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

#[inline]
fn issue_filter_status(issue: &Issue, status: &IssueStatus) -> bool {
    issue.issue_status_id == status.id
}

#[inline]
fn issue_filter_with_text(issue: &Issue, text: &str) -> bool {
    text.is_empty() || issue.title.contains(text)
}

#[inline]
fn issue_filter_with_only_my(issue: &Issue, only_my: bool, user: &Option<User>) -> bool {
    let my_id = user.as_ref().map(|u| u.id).unwrap_or_default();
    !only_my || issue.user_ids.contains(&my_id)
}

#[inline]
fn issue_filter_with_only_recent(issue: &Issue, ids: &[IssueId]) -> bool {
    ids.is_empty() || ids.contains(&issue.id)
}

fn project_issue(model: &Model, issue: &Issue) -> Node<Msg> {
    let avatars: Vec<Node<Msg>> = model
        .users
        .iter()
        .enumerate()
        .filter(|(_, user)| issue.user_ids.contains(&user.id))
        .map(|(idx, user)| {
            StyledAvatar::build()
                .size(24)
                .name(user.name.as_str())
                .avatar_url(user.avatar_url.as_ref().cloned().unwrap_or_default())
                .user_index(idx)
                .build()
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
    let drag_started = drag_ev(Ev::DragStart, move |_| {
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStarted(
            issue_id,
        )))
    });
    let drag_stopped = drag_ev(Ev::DragEnd, move |_| {
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStopped(
            issue_id,
        )))
    });
    let drag_over_handler = drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Msg::PageChanged(PageChanged::Board(BoardPageChange::ExchangePosition(
            issue_id,
        )))
    });
    let issue_id = issue.id;
    let drag_out = drag_ev(Ev::DragLeave, move |_| {
        Msg::PageChanged(PageChanged::Board(BoardPageChange::DragLeave(issue_id)))
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
            drag_out,
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
