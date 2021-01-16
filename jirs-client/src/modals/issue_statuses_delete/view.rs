use {
    crate::{components::styled_confirm_modal::StyledConfirmModal, model, shared::ToNode, Msg},
    jirs_data::IssueStatusId,
    seed::prelude::*,
};

pub fn view(_model: &model::Model, issue_status_id: IssueStatusId) -> Node<Msg> {
    StyledConfirmModal::build()
        .title("Delete column")
        .cancel_text("No")
        .confirm_text("Yes")
        .on_confirm(mouse_ev(Ev::Click, move |_| {
            Msg::DeleteIssueStatus(issue_status_id)
        }))
        .message("Are you sure you want to delete column?")
        .build()
        .into_node()
}
