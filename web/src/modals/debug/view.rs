use seed::prelude::*;
use seed::*;

use crate::components::styled_modal::StyledModal;
use crate::model::Model;
use crate::Msg;

pub fn view(model: &Model) -> Node<Msg> {
    let text = format!("{:#?}", model);
    let code = pre![text];
    StyledModal {
        width: Some(1200),
        class_list: "debugModal",
        children: vec![code],
        ..Default::default()
    }
    .render()
}
