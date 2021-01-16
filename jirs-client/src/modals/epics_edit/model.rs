use crate::FieldId;
use jirs_data::EpicFieldId;
use {
    crate::{
        components::{styled_input::*, styled_select::StyledSelectState},
        model,
    },
    jirs_data::{EpicId, IssueId},
};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Model {
    pub epic_id: EpicId,
    pub related_issues: Vec<IssueId>,
    pub name: StyledInputState,
    pub transform_into: StyledSelectState,
}

impl Model {
    pub fn new(epic_id: i32, model: &mut model::Model) -> Self {
        let name = model
            .epics_by_id
            .get(&epic_id)
            .map(|epic| epic.name.as_str())
            .unwrap_or_default();
        let related_issues = model
            .issues()
            .iter()
            .filter_map(|issue| {
                if issue.epic_id == Some(epic_id) {
                    Some(issue.id)
                } else {
                    None
                }
            })
            .collect();
        Self {
            epic_id,
            related_issues,
            name: StyledInputState::new(FieldId::EditEpic(EpicFieldId::Name), name),
            transform_into: StyledSelectState::new(
                FieldId::EditEpic(EpicFieldId::StartsAt),
                vec![],
            ),
        }
    }
}
