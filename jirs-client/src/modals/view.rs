use {
    crate::{
        components::{styled_confirm_modal::StyledConfirmModal, styled_modal::StyledModal},
        model::*,
        shared::ToNode,
        Msg,
    },
    seed::{prelude::*, *},
};

pub fn view(model: &Model) -> Node<Msg> {
    use crate::modals::{issue_statuses_delete, issues_create, issues_edit};
    let modals: Vec<Node<Msg>> = model
        .modals
        .iter()
        .map(|modal| match modal {
            ModalType::EditIssue(issue_id, modal) => {
                if let Some(_issue) = model.issues_by_id.get(issue_id) {
                    let details = issues_edit::view(model, modal.as_ref());
                    StyledModal::build()
                        .variant(crate::components::styled_modal::Variant::Center)
                        .width(1040)
                        .child(details)
                        .build()
                        .into_node()
                } else {
                    empty![]
                }
            }
            ModalType::DeleteIssueConfirm(_id) => crate::modals::issues_delete::view(model),
            ModalType::AddIssue(modal) => issues_create::view(model, modal),
            ModalType::DeleteCommentConfirm(comment_id) => {
                let comment_id = *comment_id;
                StyledConfirmModal::build()
                    .title("Are you sure you want to delete this comment?")
                    .message("Once you delete, it's gone for good.")
                    .confirm_text("Delete comment")
                    .on_confirm(mouse_ev(Ev::Click, move |_| Msg::DeleteComment(comment_id)))
                    .build()
                    .into_node()
            }
            ModalType::TimeTracking(issue_id) => {
                crate::modals::time_tracking::view(model, *issue_id)
            }
            ModalType::DeleteIssueStatusModal(delete_issue_modal) => {
                issue_statuses_delete::view(model, delete_issue_modal.delete_id)
            }
            #[cfg(debug_assertions)]
            ModalType::DebugModal => crate::modals::debug::view(model),
            ModalType::DeleteEpic(modal) => crate::modals::epic_delete::view(model, modal),
        })
        .collect();
    section![id!["modals"], modals]
}
