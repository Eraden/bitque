use jirs_data::*;
use seed::prelude::*;
use seed::*;

use crate::components::styled_avatar::*;
use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_icon::*;
use crate::model::PageContent;
use crate::pages::project_page::{events, StatusIssueIds};
use crate::{match_page, Model, Msg, Page};

#[inline(always)]
pub fn project_board_lists(model: &Model) -> Node<Msg> {
    let project_page = match_page!(model, Project; Empty);

    let now = chrono::Utc::now().naive_utc();
    let rows = project_page.visible_issues.iter().map(|per_epic| {
        let columns = per_epic
            .per_status_issues
            .iter()
            .map(|per_status| project_issue_list(model, per_status));

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
fn project_issue_list(model: &Model, per_status: &StatusIssueIds) -> Node<Msg> {
    let status_id = per_status.status_id;
    let status_name = per_status.status_name.as_str();

    let issues = per_status
        .issue_ids
        .iter()
        .filter_map(|id| model.issues_by_id.get(id))
        .map(|issue| ProjectIssue { model, issue }.render());

    let drop_handler = events::on_drop_issue_drop_zone(status_id);
    let drag_over_handler = events::on_drag_over_move_issue(status_id);

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
        let avatars = self
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
            });

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
        let drag_started = events::on_drag_started_drag_issue(self.issue.id);
        let drag_stopped = events::on_drag_stop_issue_drag_stop(self.issue.id);
        let drag_over_handler = events::on_drag_enter_change_position(self.issue.id);
        let drag_out = events::on_drag_leave_issue_drag_leave(self.issue.id);
        let on_click = events::on_click_edit_issue(self.issue.id);

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
                    div![C!["assignees"], avatars],
                ]
            ]
        ]
    }
}
