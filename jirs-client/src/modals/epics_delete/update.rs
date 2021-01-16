use {
    crate::{shared::go_to_board, ws::send_ws_msg, ModalType, Msg, OperationKind, ResourceKind},
    jirs_data::WsMsg,
    seed::prelude::*,
};

pub fn update(msg: &Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    let modal = match model.modals.iter_mut().find_map(|modal| {
        if let ModalType::DeleteEpic(modal) = modal {
            Some(modal)
        } else {
            None
        }
    }) {
        Some(modal) => modal,
        _ => return,
    };

    match msg {
        Msg::ModalDropped => {
            go_to_board(orders);
        }
        Msg::DeleteEpic => {
            send_ws_msg(WsMsg::EpicDelete(modal.epic_id), model.ws.as_ref(), orders);
        }
        Msg::ResourceChanged(ResourceKind::Epic, OperationKind::SingleRemoved, Some(_)) => {
            go_to_board(orders);
            orders.skip().send_msg(Msg::ModalDropped);
        }
        _ => {}
    };
}
