use {crate::Msg, seed::prelude::*};

pub fn update(_msg: &Msg, model: &mut crate::model::Model, _orders: &mut impl Orders<Msg>) {
    let _modal = crate::match_modal_mut!(model, DeleteEpic);
}
