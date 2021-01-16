use {
    crate::model,
    jirs_data::{EpicId, IssueId},
};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Model {
    pub epic_id: EpicId,
    pub related_issues: Vec<IssueId>,
}

impl Model {
    pub fn new(epic_id: i32, model: &mut model::Model) -> Self {
        let related_issues = model.epic_issue_ids(epic_id);
        Self {
            epic_id,
            related_issues,
        }
    }
}
