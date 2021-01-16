use {
    crate::{
        components::{styled_input::*, styled_modal::*},
        modals::epics_edit::Model,
        model,
        shared::ToNode,
        FieldId, Msg,
    },
    jirs_data::EpicFieldId,
    seed::{prelude::*, *},
};

pub fn view(_model: &model::Model, modal: &Model) -> Node<Msg> {
    let transform = if modal.related_issues.is_empty() {
        Node::Empty
    } else {
        div![]
    };
    StyledModal::build()
        .center()
        .child(h1!["Edit epic"])
        .child(
            StyledInput::build()
                .build(FieldId::EditEpic(EpicFieldId::Name))
                .into_node(),
        )
        .child(transform)
        .build()
        .into_node()
}
