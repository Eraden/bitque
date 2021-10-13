use seed::prelude::{ev, Ev, EventHandler};

use crate::Msg;

type EvHandler = EventHandler<Msg>;

pub fn on_submit_send_sign_in() -> EvHandler {
    send_sign_in(Ev::Submit)
}

pub fn on_click_send_sign_in() -> EvHandler {
    send_sign_in(Ev::Click)
}

fn send_sign_in(event: Ev) -> EvHandler {
    ev(event, |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        Msg::SignInRequest
    })
}

pub fn on_submit_bind_client() -> EvHandler {
    bind_token(Ev::Submit)
}

pub fn on_click_bind_client() -> EvHandler {
    bind_token(Ev::Click)
}

fn bind_token(event: Ev) -> EvHandler {
    ev(event, |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        Msg::BindClientRequest
    })
}
