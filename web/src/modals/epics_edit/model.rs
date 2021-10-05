use jirs_data::*;
use seed::prelude::Orders;

use crate::components::styled_checkbox::StyledCheckboxState;
use crate::components::styled_input::*;
use crate::styled_date_time_input::StyledDateTimeInputState;
use crate::{model, FieldId, Msg};

#[derive(Debug)]
pub struct Model {
    pub epic_id: EpicId,
    pub related_issues: Vec<IssueId>,
    pub name: StyledInputState,
    pub transform_into: StyledCheckboxState,
    pub starts_at: StyledDateTimeInputState,
    pub ends_at: StyledDateTimeInputState,
}

impl Model {
    pub fn new(epic_id: i32, model: &mut model::Model) -> Self {
        let (name, starts_at, ends_at) = {
            if let Some(epic) = model.epics_by_id.get(&epic_id) {
                (epic.name.as_str(), epic.starts_at, epic.ends_at)
            } else {
                ("", None, None)
            }
        };

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
            starts_at: StyledDateTimeInputState::new(
                FieldId::EditEpic(EpicFieldId::StartsAt),
                starts_at,
            ),
            ends_at: StyledDateTimeInputState::new(FieldId::EditEpic(EpicFieldId::EndsAt), ends_at),
        }
    }

    pub fn update(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        self.name.update(msg);
        self.starts_at.update(msg, orders);
        self.ends_at.update(msg, orders);
        self.transform_into.update(msg);
    }
}
