use jirs_data::*;
use seed::prelude::*;
use seed::*;

use crate::components::styled_avatar::*;
use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_icon::*;
use crate::model::PageContent;
use crate::{match_page, BoardPageChange, Model, Msg, Page, PageChanged};

#[inline(always)]
pub fn project_board_lists(model: &Model) -> Node<Msg> {
    let project_page = match_page!(model, Project; Empty);

    let now = chrono::Utc::now().naive_utc();
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
        let epic_name = match per_epic.epic_ref.as_ref() {
            Some((id, name, starts_at, ends_at)) => {
                let id = *id;
                let edit_button = StyledButton {
                    variant: ButtonVariant::Empty,
                    icon: Some(StyledIcon::from(Icon::EditAlt).render()),
                    on_click: Some(mouse_ev("click", move |ev| {
                        ev.stop_propagation();
                        ev.prevent_default();
                        seed::Url::new()
                            .add_path_part("edit-epic")
                            .add_path_part(id.to_string())
                            .go_and_push();
                        Msg::ChangePage(Page::EditEpic(id))
                    })),
                    ..Default::default()
                }
                .render();
                let delete_button = StyledButton {
                    variant: ButtonVariant::Empty,
                    icon: Some(StyledIcon::from(Icon::DeleteAlt).render()),
                    on_click: Some(mouse_ev("click", move |ev| {
                        ev.stop_propagation();
                        ev.prevent_default();
                        seed::Url::new()
                            .add_path_part("delete-epic")
                            .add_path_part(id.to_string())
                            .go_and_push();
                        Msg::ChangePage(Page::DeleteEpic(id))
                    })),
                    ..Default::default()
                }
                .render();

                let range = match (starts_at, ends_at) {
                    (Some(s), Some(e)) => div![
                        C!["timeRange"],
                        div![C!["startsAt"], format!("{}", s.format("%e %B %Y"))],
                        div![C!["separator"], "-"],
                        div![
                            IF![now.date() > e.date() => C!["error"]],
                            IF![now.date() == e.date() => C!["warning"]],
                            C!["endsAt"],
                            format!("{}", e.format("%e %B %Y"))
                        ]
                    ],
                    _ => Node::Empty,
                };

                div![
                    C!["epicHeader"],
                    div![C!["epicName"], name],
                    range,
                    div![C!["epicActions"], edit_button, delete_button],
                ]
            }
            _ => Node::Empty,
        };
        div![C!["row"], epic_name, div![C!["projectBoardLists"], columns]]
    });
    div![C!["rows"], rows]
}

#[inline(always)]
fn project_issue_list(
    model: &Model,
    status_id: IssueStatusId,
    status_name: &str,
    issues: &[&Issue],
) -> Node<Msg> {
    let issues: Vec<Node<Msg>> = issues
        .iter()
        .map(|issue| ProjectIssue { model, issue }.render())
        .collect();
    let drop_handler = {
        let send_status = status_id;
        drag_ev(Ev::Drop, move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
            Some(Msg::PageChanged(PageChanged::Board(
                BoardPageChange::IssueDropZone(send_status),
            )))
        })
    };

    let drag_over_handler = {
        let send_status = status_id;
        drag_ev(Ev::DragOver, move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
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

pub struct ProjectIssue<'l> {
    pub model: &'l Model,
    pub issue: &'l Issue,
}

impl<'l> ProjectIssue<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let is_dragging = match &self.model.page_content {
            PageContent::Project(project_page) => project_page.issue_drag.is_dragging(),
            _ => false,
        };
        let avatars: Vec<Node<Msg>> = self
            .issue
            .user_ids
            .iter()
            .filter_map(|id| self.model.users_by_id.get(id))
            .map(|user| {
                StyledAvatar {
                    avatar_url: user.avatar_url.as_deref(),
                    size: 24,
                    name: &user.name,
                    ..StyledAvatar::default()
                }
                .render()
            })
            .collect();

        let issue_type_icon = StyledIcon {
            icon: self.issue.issue_type.into(),
            class_list: self.issue.issue_type.to_str(),
            color: Some(self.issue.issue_type.to_str()),
            ..Default::default()
        }
        .render();

        let priority_icon = StyledIcon {
            icon: self.issue.priority.into(),
            class_list: self.issue.priority.to_str(),
            color: Some(self.issue.priority.to_str()),
            ..Default::default()
        }
        .render();

        let issue_id = self.issue.id;
        let drag_started = drag_ev(Ev::DragStart, move |ev| {
            ev.stop_propagation();
            Some(Msg::PageChanged(PageChanged::Board(
                BoardPageChange::IssueDragStarted(issue_id),
            )))
        });
        let drag_stopped = drag_ev(Ev::DragEnd, move |ev| {
            ev.stop_propagation();
            Some(Msg::PageChanged(PageChanged::Board(
                BoardPageChange::IssueDragStopped(issue_id),
            )))
        });
        let drag_over_handler = drag_ev(Ev::DragEnter, move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
            Some(Msg::PageChanged(PageChanged::Board(
                BoardPageChange::ChangePosition(issue_id),
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

        a![
            drag_started,
            on_click,
            C!["issueLink"],
            attrs![At::Href => format!("/issues/{id}", id = issue_id)],
            IF![is_dragging => div![C!["dragCover"], drag_over_handler]],
            div![
                C!["issue"],
                attrs![At::Draggable => true],
                drag_stopped,
                drag_out,
                p![C!["title"], self.issue.title.as_str()],
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
}
