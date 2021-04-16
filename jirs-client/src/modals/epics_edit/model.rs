use jirs_data::*;
use seed::prelude::Orders;

use crate::components::styled_checkbox::StyledCheckboxState;
use crate::components::styled_input::*;
use crate::{model, FieldId, Msg};

#[derive(Debug)]
pub struct Model {
    pub epic_id: EpicId,
    pub related_issues: Vec<IssueId>,
    pub name: StyledInputState,
    pub transform_into: StyledCheckboxState,
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
            transform_into: StyledCheckboxState::new(
                FieldId::EditEpic(EpicFieldId::TransformInto),
                0,
            ),
        }
    }

    pub fn update(&mut self, msg: &Msg, _orders: &mut impl Orders<Msg>) {
        self.name.update(msg);
        self.transform_into.update(msg);
    }
}
