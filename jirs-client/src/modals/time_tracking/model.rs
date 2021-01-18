use jirs_data::IssueId;

#[derive(Debug, Default)]
pub struct Model {
    pub issue_id: IssueId,
}

impl Model {
    pub fn new(issue_id: IssueId) -> Self {
        Self { issue_id }
    }
}
