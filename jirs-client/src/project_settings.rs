use seed::prelude::*;

use crate::shared::inner_layout;
use crate::{model, Msg};

pub fn update(_msg: Msg, _model: &mut model::Model, _orders: &mut impl Orders<Msg>) {}

pub fn view(model: &model::Model) -> Node<Msg> {
    let project_section = vec![];

    inner_layout(model, "projectSettings", project_section, None)
}
