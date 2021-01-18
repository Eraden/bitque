use {
    crate::{model::*, Msg},
    seed::{prelude::*, *},
};

pub fn view(model: &Model) -> Node<Msg> {
    let mut nodes = Vec::with_capacity(model.modal_stack().len());

    for modal_type in model.modal_stack() {
        match modal_type {
            ModalType::AddIssue(_) => {
                if let Some(modal) = &model.modals().add_issue {
                    let node = crate::modals::issues_create::view(model, modal);
                    nodes.push(node);
                }
            }
            ModalType::EditIssue(_) => {
                if let Some(modal) = &model.modals().edit_issue {
                    let node = crate::modals::issues_edit::view(model, modal);
                    nodes.push(node);
                }
            }
            ModalType::DeleteEpic(_) => {
                if let Some(modal) = &model.modals().delete_epic {
                    let node = crate::modals::epics_delete::view(model, modal);
                    nodes.push(node);
                }
            }
            ModalType::EditEpic(_) => {
                if let Some(modal) = &model.modals().edit_epic {
                    let node = crate::modals::epics_edit::view(model, modal);
                    nodes.push(node);
                }
            }
            ModalType::DeleteIssueConfirm(_) => {
                if let Some(_issue_id) = &model.modals().delete_issue_confirm {
                    let node = crate::modals::issues_delete::view(model);
                    nodes.push(node);
                }
            }
            ModalType::DeleteCommentConfirm(_) => {
                if let Some(modal) = &model.modals().delete_comment_confirm {
                    let node = crate::modals::comments_delete::view(model, modal);
                    nodes.push(node);
                }
            }
            ModalType::TimeTracking(_) => {
                if let Some(modal) = &model.modals().time_tracking {
                    let node = crate::modals::time_tracking::view(model, modal);
                    nodes.push(node);
                }
            }
            ModalType::DeleteIssueStatusModal(_) => {
                if let Some(modal) = &model.modals().delete_issue_status_modal {
                    let node = crate::modals::issue_statuses_delete::view(model, modal.delete_id);
                    nodes.push(node);
                }
            }
            #[cfg(debug_assertions)]
            ModalType::DebugModal(_) => {
                if let Some(true) = &model.modals().debug_modal {
                    let node = crate::modals::debug::view(model);
                    nodes.push(node)
                }
            }
        };
    }
    section![id!["modals"], nodes]
}
