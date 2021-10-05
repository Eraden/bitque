use jirs_data::{EpicId, IssueId};

use crate::model;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Model {
    pub epic_id: EpicId,
    pub related_issues: Vec<IssueId>,
}

impl Model {
    pub fn new(epic_id: i32, model: &model::Model) -> Self {
        let related_issues = model.epic_issue_ids(epic_id);
        Self {
            epic_id,
            related_issues,
        }
    }
}
