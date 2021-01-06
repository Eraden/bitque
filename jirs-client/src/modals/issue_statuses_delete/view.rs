use seed::prelude::*;

use jirs_data::IssueStatusId;

use crate::{
    model,
    shared::{styled_confirm_modal::StyledConfirmModal, ToNode},
    Msg,
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
