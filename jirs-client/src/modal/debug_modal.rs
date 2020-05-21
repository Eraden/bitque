use seed::{prelude::*, *};

use crate::model::Model;
use crate::shared::styled_modal::StyledModal;
use crate::shared::ToNode;
use crate::Msg;

pub fn view(model: &Model) -> Node<Msg> {
    let text = format!("{:#?}", model);
    let code = pre![text];
    StyledModal::build()
        .width(1200)
        .add_class("debugModal")
        .center()
        .children(vec![code])
        .build()
        .into_node()
}
