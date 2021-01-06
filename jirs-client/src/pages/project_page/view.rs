use {
    crate::{
        model::{Model, Page, PageContent},
        shared::{
            inner_layout,
            styled_avatar::StyledAvatar,
            styled_button::StyledButton,
            styled_icon::{Icon, StyledIcon},
            styled_input::StyledInput,
            ToNode,
        },
        BoardPageChange, FieldId, Msg, PageChanged,
    },
    jirs_data::*,
    seed::{prelude::*, *},
};

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
        C!["breadcrumbsContainer"],
        span!["Projects"],
        span![C!["breadcrumbsDivider"], "/"],
        span![project_name],
        span![C!["breadcrumbsDivider"], "/"],
        span!["Kanban Board"]
    ]
}

fn header() -> Node<Msg> {
    let button = StyledButton::build()
        .secondary()
        .text("Github Repo")
        .icon(Icon::Github)
        .build()
        .into_node();
    div![
        id!["projectBoardHeader"],
        div![id!["boardName"], C!["headerChild"], "Kanban board"],
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

    let search_input = StyledInput::build()
        .icon(Icon::Search)
        .valid(true)
        .value(project_page.text_filter.as_str())
        .build(FieldId::TextFilterBoard)
        .into_node();

    let only_my = StyledButton::build()
        .empty()
        .active(project_page.only_my_filter)
        .text("Only My Issues")
        .add_class("filterChild")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ProjectToggleOnlyMy))
        .build()
        .into_node();

    let recently_updated = StyledButton::build()
        .empty()
        .text("Recently Updated")
        .add_class("filterChild")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ProjectToggleRecentlyUpdated))
        .build()
        .into_node();

    let clear_all = if project_page.only_my_filter
        || project_page.recently_updated_filter
        || !project_page.active_avatar_filters.is_empty()
    {
        seed::button![
            id!["clearAllFilters"],
            C!["filterChild"],
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
                .avatar_url(user.avatar_url.as_deref().unwrap_or_default())
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

    div![id!["avatars"], C!["filterChild"], avatars]
}

fn project_board_lists(model: &Model) -> Node<Msg> {
    let project_page = match &model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return empty![],
    };
    let rows = project_page.visible_issues.iter().map(|per_epic| {
        let columns: Vec<Node<Msg>> = per_epic
            .per_status_issues
            .iter()
            .map(|per_status| {
                let issues: Vec<&Issue> = per_status
                    .issue_ids
                    .iter()
                    .filter_map(|id| model.issues_by_id.get(id))
                    .collect();
                project_issue_list(
                    model,
                    per_status.status_id,
                    &per_status.status_name,
                    issues.as_slice(),
                )
            })
            .collect();
        div![
            C!["row"],
            div![C!["epicName"], per_epic.epic_name.as_str()],
            div![C!["projectBoardLists"], columns]
        ]
    });
    div![C!["rows"], rows]
}

fn project_issue_list(
    model: &Model,
    status_id: IssueStatusId,
    status_name: &str,
    issues: &[&Issue],
) -> Node<Msg> {
    let issues: Vec<Node<Msg>> = issues
        .iter()
        .map(|issue| project_issue(model, issue))
        .collect();
    let drop_handler = {
        let send_status = status_id;
        drag_ev(Ev::Drop, move |ev| {
            ev.prevent_default();
            Some(Msg::PageChanged(PageChanged::Board(
                BoardPageChange::IssueDropZone(send_status),
            )))
        })
    };

    let drag_over_handler = {
        let send_status = status_id;
        drag_ev(Ev::DragOver, move |ev| {
            ev.prevent_default();
            Some(Msg::PageChanged(PageChanged::Board(
                BoardPageChange::IssueDragOverStatus(send_status),
            )))
        })
    };

    div![
        C!["list"],
        div![C!["title"], status_name, div![C!["issuesCount"]]],
        div![
            C!["issues"],
            attrs![At::DropZone => "link"],
            drop_handler,
            drag_over_handler,
            issues
        ]
    ]
}

fn project_issue(model: &Model, issue: &Issue) -> Node<Msg> {
    let avatars: Vec<Node<Msg>> = issue
        .user_ids
        .iter()
        .filter_map(|id| model.users_by_id.get(id))
        .map(|user| {
            StyledAvatar::build()
                .size(24)
                .name(user.name.as_str())
                .avatar_url(user.avatar_url.as_deref().unwrap_or_default())
                .user_index(0)
                .build()
                .into_node()
        })
        .collect();

    let issue_type_icon = StyledIcon::build(issue.issue_type.clone().into())
        .with_color(issue.issue_type.to_str())
        .build()
        .into_node();
    let priority_icon = {
        let icon = match issue.priority {
            IssuePriority::Low | IssuePriority::Lowest => Icon::ArrowDown,
            _ => Icon::ArrowUp,
        };
        StyledIcon::build(icon)
            .with_color(issue.priority.to_str())
            .build()
            .into_node()
    };

    let issue_id = issue.id;
    let drag_started = drag_ev(Ev::DragStart, move |_| {
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::IssueDragStarted(issue_id),
        )))
    });
    let drag_stopped = drag_ev(Ev::DragEnd, move |_| {
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::IssueDragStopped(issue_id),
        )))
    });
    let drag_over_handler = drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::ExchangePosition(issue_id),
        )))
    });

    let drag_out = drag_ev(Ev::DragLeave, move |_| {
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::DragLeave(issue_id),
        )))
    });
    let on_click = mouse_ev("click", move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        seed::Url::new()
            .add_path_part("issues")
            .add_path_part(format!("{}", issue_id))
            .go_and_push();
        Msg::ChangePage(Page::EditIssue(issue_id))
    });

    let href = format!("/issues/{id}", id = issue_id);

    a![
        drag_started,
        on_click,
        C!["issueLink"],
        attrs![At::Href => href],
        div![
            C!["issue"],
            attrs![At::Draggable => true],
            drag_stopped,
            drag_over_handler,
            drag_out,
            p![C!["title"], issue.title.as_str()],
            div![
                C!["bottom"],
                div![
                    div![C!["issueTypeIcon"], issue_type_icon],
                    div![C!["issuePriorityIcon"], priority_icon]
                ],
                div![C!["assignees"], avatars,],
            ]
        ]
    ]
}
