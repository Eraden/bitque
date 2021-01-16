use {
    crate::{
        model::Model,
        shared::{styled_modal::StyledModal, ToNode},
        Msg,
    },
    seed::{prelude::*, *},
};

pub fn view(model: &Model) -> Node<Msg> {
    let text = format!("{:#?}", model);
    let code = pre![text];
    StyledModal::build()
        .width(1200)
        .add_class("debugModal")
        .center()
        .children(vec![code].into_iter())
        .build()
        .into_node()
}
