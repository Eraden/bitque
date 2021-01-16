use {
    crate::{
        model::{IssueModal, Model},
        shared::styled_field::StyledField,
        shared::styled_select::StyledSelect,
        shared::{ToChild, ToNode},
        FieldId, Msg,
    },
    jirs_data::EpicId,
    seed::prelude::Node,
};

pub fn epic_field<Modal>(model: &Model, modal: &Modal, field_id: FieldId) -> Option<Node<Msg>>
where
    Modal: IssueModal,
{
    if model.epics.is_empty() {
        None
    } else {
        let selected = modal
            .epic_id_value()
            .and_then(|id| model.epics.iter().find(|epic| epic.id == id as EpicId))
            .map(|epic| vec![epic.to_child()])
            .unwrap_or_default();
        let input = StyledSelect::build()
            .name("epic")
            .selected(selected)
            .options(model.epics.iter().map(|epic| epic.to_child()))
            .normal()
            .clearable()
            .text_filter(modal.epic_state().text_filter.as_str())
            .opened(modal.epic_state().opened)
            .valid(true)
            .build(field_id)
            .into_node();
        Some(
            StyledField::build()
                .label("Epic")
                .tip("Feature group")
                .input(input)
                .build()
                .into_node(),
        )
    }
}
