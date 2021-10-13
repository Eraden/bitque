use seed::prelude::{ev, Ev, EventHandler};

type EvHandler = EventHandler<crate::Msg>;

pub fn on_submit_submit_profile() -> EvHandler {
    ev(Ev::Submit, |ev| {
        ev.prevent_default();
        crate::Msg::PageChanged(crate::PageChanged::Profile(
            crate::ProfilePageChange::SubmitForm,
        ))
    })
}
