use crate::shared::inner_layout;
use crate::{model, Msg};
use seed::{prelude::*, *};

pub fn update(_msg: Msg, _model: &mut model::Model, _orders: &mut impl Orders<Msg>) {}

pub fn view(model: &model::Model) -> Node<Msg> {
    let project_section = section![id!["project-settings-section"],];

    inner_layout(model, project_section)
}
