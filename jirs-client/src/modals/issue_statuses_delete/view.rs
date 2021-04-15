use {
    crate::{components::styled_confirm_modal::StyledConfirmModal, model, shared::ToNode, Msg},
    jirs_data::IssueStatusId,
    seed::prelude::*,
};

pub fn view(_model: &model::Model, issue_status_id: IssueStatusId) -> Node<Msg> {
    StyledConfirmModal {
        title: "Delete column",
        message: "Are you sure you want to delete column?",
        confirm_text: "Yes",
        cancel_text: "No",
        on_confirm: Some(mouse_ev(Ev::Click, move |_| {
            Msg::DeleteIssueStatus(issue_status_id)
        })),
    }
    .into_node()
}
