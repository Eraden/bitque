use seed::prelude::*;

use crate::model::Page;
use crate::{BoardPageChange, Msg, PageChanged};

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
