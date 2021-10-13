use jirs_data::{EpicId, UserId};
use seed::prelude::*;

use crate::model::Page;
use crate::{AvatarFilterActive, BoardPageChange, Msg, PageChanged};

pub type EvHandler = seed::EventHandler<Msg>;

pub fn on_drag_over_move_issue(status_id: i32) -> EvHandler {
    drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::IssueDragOverStatus(status_id),
        )))
    })
}

pub fn on_drop_issue_drop_zone(status_id: i32) -> EvHandler {
    drag_ev(Ev::Drop, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::IssueDropZone(status_id),
        )))
    })
}

pub fn on_drag_started_drag_issue(issue_id: i32) -> EvHandler {
    drag_ev(Ev::DragStart, move |ev| {
        ev.stop_propagation();
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::IssueDragStarted(issue_id),
        )))
    })
}

pub fn on_drag_stop_issue_drag_stop(issue_id: i32) -> EvHandler {
    drag_ev(Ev::DragEnd, move |ev| {
        ev.stop_propagation();
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::IssueDragStopped(issue_id),
        )))
    })
}

pub fn on_drag_enter_change_position(issue_id: i32) -> EvHandler {
    drag_ev(Ev::DragEnter, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::ChangePosition(issue_id),
        )))
    })
}

pub fn on_drag_leave_issue_drag_leave(issue_id: i32) -> EvHandler {
    drag_ev(Ev::DragLeave, move |_| {
        Some(Msg::PageChanged(PageChanged::Board(
            BoardPageChange::DragLeave(issue_id),
        )))
    })
}

pub fn on_click_edit_issue(issue_id: i32) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        seed::Url::new()
            .add_path_part("issues")
            .add_path_part(format!("{}", issue_id))
            .go_and_push();
        Msg::ChangePage(Page::EditIssue(issue_id))
    })
}

pub fn on_click_toggle_only_my() -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ProjectToggleOnlyMy
    })
}

pub fn on_click_toggle_recent() -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ProjectToggleRecentlyUpdated
    })
}

pub fn on_click_filter_by_user(user_id: UserId, active: AvatarFilterActive) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ProjectAvatarFilterChanged(user_id, active)
    })
}

pub fn on_click_clear_filters() -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ProjectClearFilters
    })
}

pub fn on_click_goto_delete_epic(epic_id: EpicId) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        seed::Url::new()
            .add_path_part("delete-epic")
            .add_path_part(epic_id.to_string())
            .go_and_push();
        Msg::ChangePage(Page::DeleteEpic(epic_id))
    })
}

pub fn on_click_goto_edit_epic(epic_id: EpicId) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        seed::Url::new()
            .add_path_part("edit-epic")
            .add_path_part(epic_id.to_string())
            .go_and_push();
        Msg::ChangePage(Page::EditEpic(epic_id))
    })
}
