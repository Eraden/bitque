use {
    crate::{ws::send_ws_msg, Msg, OperationKind, ResourceKind},
    jirs_data::WsMsg,
    seed::prelude::*,
};

pub fn update(msg: &Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    let modal = match &mut model.modals_mut().delete_epic {
        Some(modal) => modal,
        _ => return,
    };

    match msg {
        Msg::DeleteEpic => {
            send_ws_msg(WsMsg::EpicDelete(modal.epic_id), model.ws.as_ref(), orders);
        }
        Msg::ResourceChanged(ResourceKind::Epic, OperationKind::SingleRemoved, Some(_)) => {
            orders.skip().send_msg(Msg::ModalDropped);
        }
        _ => {}
    };
}
