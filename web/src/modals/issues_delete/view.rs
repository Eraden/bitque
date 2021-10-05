use seed::prelude::*;

use crate::components::styled_confirm_modal::StyledConfirmModal;
use crate::{model, Msg};

pub fn view(model: &model::Model) -> Node<Msg> {
    let issue_id = match &model.modals().delete_issue_confirm {
        Some(modal) => modal.issue_id,
        _ => return Node::Empty,
    };

    StyledConfirmModal {
        title: "Are you sure you want to delete this issue?",
        message: "Once you delete, it's gone for good.",
        confirm_text: "Delete issue",
        cancel_text: "Cancel",
        on_confirm: Some(mouse_ev(Ev::Click, move |_| Msg::DeleteIssue(issue_id))),
    }
    .render()
}
