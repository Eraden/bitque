use jirs_data::IssueId;
use seed::prelude::*;

pub type EvHandler = seed::EventHandler<crate::Msg>;

pub fn on_click_close_modal() -> EvHandler {
    mouse_ev(Ev::Click, |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        crate::Msg::ModalDropped
    })
}

pub fn on_click_open_delete_confirm(issue_id: IssueId) -> EvHandler {
    mouse_ev(Ev::Click, move |_| {
        crate::Msg::ModalOpened(crate::ModalType::DeleteIssueConfirm(Some(issue_id)))
    })
}
