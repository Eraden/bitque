use {
    crate::{model::*, Msg},
    seed::{prelude::*, *},
};

pub fn view(model: &Model) -> Node<Msg> {
    let nodes = model
        .modal_stack()
        .iter()
        .filter_map(|modal_type| match modal_type {
            ModalType::AddIssue(_) => model
                .modals()
                .add_issue
                .as_ref()
                .map(|modal| crate::modals::issues_create::view(model, modal)),
            ModalType::EditIssue(_) => model
                .modals()
                .edit_issue
                .as_ref()
                .map(|modal| crate::modals::issues_edit::view(model, modal)),
            ModalType::DeleteEpic(_) => model
                .modals()
                .delete_epic
                .as_ref()
                .map(|modal| crate::modals::epics_delete::view(model, modal)),
            ModalType::EditEpic(_) => model
                .modals()
                .edit_epic
                .as_ref()
                .map(|modal| crate::modals::epics_edit::view(model, modal)),
            ModalType::DeleteIssueConfirm(_) => model
                .modals()
                .delete_issue_confirm
                .as_ref()
                .map(|_id| crate::modals::issues_delete::view(model)),
            ModalType::DeleteCommentConfirm(_) => model
                .modals()
                .delete_comment_confirm
                .as_ref()
                .map(|modal| crate::modals::comments_delete::view(model, modal)),
            ModalType::TimeTracking(_) => model
                .modals()
                .time_tracking
                .as_ref()
                .map(|modal| crate::modals::time_tracking::view(model, modal)),
            ModalType::DeleteIssueStatusModal(_) => model
                .modals()
                .delete_issue_status_modal
                .as_ref()
                .map(|modal| crate::modals::issue_statuses_delete::view(model, modal.delete_id)),
            #[cfg(debug_assertions)]
            ModalType::DebugModal(_) => model
                .modals()
                .debug_modal
                .as_ref()
                .map(|_| crate::modals::debug::view(model)),
        });
    section![id!["modals"], nodes]
}
