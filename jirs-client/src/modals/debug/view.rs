use {
    crate::{components::styled_modal::StyledModal, model::Model, shared::ToNode, Msg},
    seed::{prelude::*, *},
};

pub fn view(model: &Model) -> Node<Msg> {
    let text = format!("{:#?}", model);
    let code = pre![text];
    StyledModal {
        width: Some(1200),
        class_list: "debugModal",
        children: vec![code],
        ..Default::default()
    }
    .into_node()
}
