use crate::{model, Msg};
use seed::{prelude::*, *};

pub fn update(_msg: Msg, _model: &mut model::Model, _orders: &mut impl Orders<Msg>) {}

pub fn view(_model: &model::Model) -> Node<Msg> {
    div![]
}
