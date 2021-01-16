use {
    crate::{shared::go_to_board, Msg},
    seed::prelude::*,
};

pub fn update(msg: &Msg, _model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    if let Msg::ModalDropped = msg {
        go_to_board(orders);
    };
}
