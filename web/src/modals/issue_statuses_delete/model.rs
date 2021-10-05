use jirs_data::IssueStatusId;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Model {
    pub delete_id: IssueStatusId,
    pub receiver_id: Option<IssueStatusId>,
}

impl Model {
    pub fn new(delete_id: IssueStatusId) -> Self {
        Self {
            delete_id,
            receiver_id: None,
        }
    }
}
