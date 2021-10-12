use jirs_data::IssueStatusId;
use seed::prelude::*;

type EvHandler = EventHandler<crate::Msg>;

pub fn on_click_submit_save_changes() -> EvHandler {
    submit_changes(Ev::Click)
}

pub fn on_submit_submit_save_changes() -> EvHandler {
    submit_changes(Ev::Submit)
}

pub fn on_focus_out_close_edit_status() -> EvHandler {
    ev("focusout", move |ev| {
        ev.stop_propagation();

        crate::Msg::PageChanged(crate::PageChanged::ProjectSettings(
            crate::ProjectPageChange::EditIssueStatusName(None),
        ))
    })
}

fn submit_changes(event: Ev) -> EvHandler {
    ev(event, |ev| {
        ev.prevent_default();
        ev.stop_propagation();

        crate::Msg::PageChanged(crate::PageChanged::ProjectSettings(
            crate::ProjectPageChange::SubmitProjectSettingsForm,
        ))
    })
}

pub fn on_submit_create_column() -> EvHandler {
    ev(Ev::Submit, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Some(crate::Msg::PageChanged(
            crate::PageChanged::ProjectSettings(crate::ProjectPageChange::SubmitIssueStatusForm),
        ))
    })
}

pub fn on_drag_start_start_drag_column(issue_status_id: IssueStatusId) -> EvHandler {
    drag_ev(Ev::DragStart, move |_| {
        crate::Msg::PageChanged(crate::PageChanged::ProjectSettings(
            crate::ProjectPageChange::ColumnDragStarted(issue_status_id),
        ))
    })
}

pub fn on_drag_end_stop_drag_column(issue_status_id: IssueStatusId) -> EvHandler {
    drag_ev(Ev::DragEnd, move |_| {
        Some(crate::Msg::PageChanged(
            crate::PageChanged::ProjectSettings(crate::ProjectPageChange::ColumnDragStopped(
                issue_status_id,
            )),
        ))
    })
}

pub fn on_drag_over_exchange_position(issue_status_id: IssueStatusId) -> EvHandler {
    drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Some(crate::Msg::PageChanged(
            crate::PageChanged::ProjectSettings(crate::ProjectPageChange::ColumnExchangePosition(
                issue_status_id,
            )),
        ))
    })
}

pub fn on_drag_leave_leave_drag_column(issue_status_id: IssueStatusId) -> EvHandler {
    drag_ev(Ev::DragLeave, move |_| {
        Some(crate::Msg::PageChanged(
            crate::PageChanged::ProjectSettings(crate::ProjectPageChange::ColumnDragLeave(
                issue_status_id,
            )),
        ))
    })
}

pub fn on_click_edit_column(issue_status_id: IssueStatusId) -> EvHandler {
    mouse_ev(Ev::Click, move |_| {
        crate::Msg::PageChanged(crate::PageChanged::ProjectSettings(
            crate::ProjectPageChange::EditIssueStatusName(Some(issue_status_id)),
        ))
    })
}

pub fn on_click_delete_column(issue_status_id: IssueStatusId) -> EvHandler {
    mouse_ev(Ev::Click, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        crate::Msg::ModalOpened(crate::ModalType::DeleteIssueStatusModal(Some(
            issue_status_id,
        )))
    })
}

pub fn on_click_add_status() -> EvHandler {
    mouse_ev(Ev::Click, move |_| {
        crate::Msg::PageChanged(crate::PageChanged::ProjectSettings(
            crate::ProjectPageChange::EditIssueStatusName(Some(0)),
        ))
    })
}
