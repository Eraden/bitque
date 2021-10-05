use jirs_data::CommentId;
use seed::prelude::*;

use crate::styled_confirm_modal::StyledConfirmModal;
use crate::{model, Msg};

pub fn view(_model: &model::Model, modal: &super::Model) -> Node<Msg> {
    let comment_id: CommentId = modal.comment_id;
    StyledConfirmModal {
        title: "Are you sure you want to delete this comment?",
        message: "Once you delete, it's gone for good.",
        confirm_text: "Delete comment",
        on_confirm: Some(mouse_ev(Ev::Click, move |_| Msg::DeleteComment(comment_id))),
        ..Default::default()
    }
    .render()
}
