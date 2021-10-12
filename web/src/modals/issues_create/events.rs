use seed::prelude::{ev, Ev, EventHandler};

use crate::modals::issues_create::Type;

type EvHandler = EventHandler<crate::Msg>;

pub fn on_click_submit_action(issue_type: Type) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        Some(issue_type.submit_action())
    })
}

pub fn on_click_close_modal() -> EvHandler {
    ev(Ev::Click, |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        Some(crate::Msg::ModalDropped)
    })
}
