use jirs_data::CommentId;

#[derive(Debug, Default)]
pub struct Model {
    pub comment_id: CommentId,
}

impl Model {
    pub fn new(comment_id: CommentId) -> Self {
        Self { comment_id }
    }
}
