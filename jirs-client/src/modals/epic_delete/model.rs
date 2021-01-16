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
    pub fn new(issue_id: i32, model: &mut model::Model) -> Self {
        let related_issues = model
            .issues
            .iter()
            .filter_map(|issue| {
                if issue.epic_id == Some(issue_id) {
                    Some(issue.id)
                } else {
                    None
                }
            })
            .collect();
        Self {
            epic_id: issue_id,
            related_issues,
        }
    }
}
