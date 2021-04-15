use {
    crate::{model, shared::ToNode, styled_confirm_modal::StyledConfirmModal, Msg},
    jirs_data::CommentId,
    seed::prelude::*,
};

pub fn view(_model: &model::Model, modal: &super::Model) -> Node<Msg> {
    let comment_id: CommentId = modal.comment_id;
    StyledConfirmModal {
        title: "Are you sure you want to delete this comment?",
        message: "Once you delete, it's gone for good.",
        confirm_text: "Delete comment",
        on_confirm: Some(mouse_ev(Ev::Click, move |_| Msg::DeleteComment(comment_id))),
        ..Default::default()
    }
    .into_node()
}
