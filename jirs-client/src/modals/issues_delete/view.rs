use {
    crate::{components::styled_confirm_modal::StyledConfirmModal, model, shared::ToNode, Msg},
    seed::prelude::*,
};

pub fn view(model: &model::Model) -> Node<Msg> {
    let issue_id = match &model.modals().delete_issue_confirm {
        Some(modal) => modal.issue_id,
        _ => return Node::Empty,
    };

    let handle_issue_delete = mouse_ev(Ev::Click, move |_| Msg::DeleteIssue(issue_id));

    StyledConfirmModal::build()
        .title("Are you sure you want to delete this issue?")
        .message("Once you delete, it's gone for good.")
        .confirm_text("Delete issue")
        .cancel_text("Cancel")
        .on_confirm(handle_issue_delete)
        .build()
        .into_node()
}
