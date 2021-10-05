use jirs_data::IssueStatusId;
use seed::prelude::*;

use crate::components::styled_confirm_modal::StyledConfirmModal;
use crate::{model, Msg};

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
    .render()
}
