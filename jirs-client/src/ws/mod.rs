use std::sync::RwLock;

use seed::{prelude::*, *};

use jirs_data::WsMsg;

use crate::model::Model;
use crate::{model, Msg, APP, RECEIVED};

pub fn handle(msg: WsMsg) {
    let app = match unsafe { APP.as_mut().unwrap() }.write() {
        Ok(app) => app,
        _ => return,
    };

    match msg {
        WsMsg::Ping | WsMsg::Pong => {}
        _ => app.update(Msg::WsMsg(msg)),
    }
}

pub fn update(msg: &Msg, model: &mut model::Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WsMsg(WsMsg::ProjectLoaded(project)) => {
            model.current_project = Some(project.clone());
            log!(model);
        }
        _ => (),
    }
}
