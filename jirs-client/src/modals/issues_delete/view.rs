use {
    crate::{
        model, model::ModalType, shared::styled_confirm_modal::StyledConfirmModal, shared::ToNode,
        Msg,
    },
    seed::{prelude::*, *},
};

pub fn view(model: &model::Model) -> Node<Msg> {
    let opt_id = model
        .modals
        .iter()
        .filter_map(|modal| match modal {
            ModalType::EditIssue(issue_id, _) => Some(issue_id),
            _ => None,
        })
        .find(|id| id.eq(id));

    let issue_id = match opt_id {
        Some(id) => *id,
        _ => return empty![],
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
