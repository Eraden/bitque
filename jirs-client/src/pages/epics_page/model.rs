use std::collections::HashMap;

use jirs_data::{EpicId, IssueId};

use crate::model::Model;

#[derive(Debug, Default)]
pub struct EpicsPage {
    pub(crate) issues_per_epic: HashMap<EpicId, Vec<IssueId>>,
}

impl EpicsPage {
    pub fn new(model: &Model) -> Self {
        let issues_per_epic = Self::build_issues_per_epic(model);
        Self { issues_per_epic }
    }

    pub fn build_issues_per_epic(model: &Model) -> HashMap<EpicId, Vec<IssueId>> {
        model.issues().iter().fold(
            HashMap::with_capacity(model.issues().len()),
            |mut h, issue| {
                if let Some(epic_id) = issue.epic_id.as_ref() {
                    h.entry(*epic_id).or_default().push(issue.id);
                }
                h
            },
        )
    }

    pub fn issues(&self, epic_id: EpicId) -> Option<&Vec<IssueId>> {
        self.issues_per_epic.get(&epic_id)
    }
}
