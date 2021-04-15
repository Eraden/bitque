use {
    crate::{
        components::{styled_field::StyledField, styled_select::StyledSelect},
        model::{IssueModal, Model},
        shared::{ToChild, ToNode},
        FieldId, Msg,
    },
    jirs_data::EpicId,
    seed::prelude::Node,
};

use crate::components::styled_select::SelectVariant;

pub fn epic_field<Modal>(model: &Model, modal: &Modal, field_id: FieldId) -> Option<Node<Msg>>
where
    Modal: IssueModal,
{
    if model.epics.is_empty() {
        return None;
    }
    let selected = modal
        .epic_id_value()
        .and_then(|id| model.epics.iter().find(|epic| epic.id == id as EpicId))
        .map(|epic| vec![epic.to_child()])
        .unwrap_or_default();
    let input = StyledSelect {
        id: field_id,
        name: "epic",
        selected,
        options: Some(model.epics.iter().map(|epic| epic.to_child())),
        variant: SelectVariant::Normal,
        clearable: true,
        text_filter: modal.epic_state().text_filter.as_str(),
        opened: modal.epic_state().opened,
        valid: true,
        ..Default::default()
    }
    .into_node();
    Some(
        StyledField {
            label: "Epic",
            tip: Some("Feature group"),
            input,
            ..Default::default()
        }
        .into_node(),
    )
}
