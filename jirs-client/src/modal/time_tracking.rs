use seed::{prelude::*, *};

use jirs_data::IssueId;

use crate::model::{ModalType, Model};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_modal::StyledModal;
use crate::shared::tracking_widget::tracking_widget;
use crate::shared::{find_issue, ToNode};
use crate::{EditIssueModalFieldId, FieldId, Msg};

pub fn view(model: &Model, issue_id: IssueId) -> Node<Msg> {
    let _issue = match find_issue(model, issue_id) {
        Some(issue) => issue,
        _ => return empty![],
    };

    let edit_issue_modal = match model.modals.get(0) {
        Some(ModalType::EditIssue(_, modal)) => modal,
        _ => return empty![],
    };

    let modal_title = div![class!["modalTitle"], "Time tracking"];

    let tracking = tracking_widget(model, edit_issue_modal);

    let time_spent = StyledInput::build(FieldId::EditIssueModal(EditIssueModalFieldId::TimeSpend))
        .value(
            edit_issue_modal
                .payload
                .time_spent
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_default(),
        )
        .valid(true)
        .build()
        .into_node();
    let time_spent_field = StyledField::build()
        .input(time_spent)
        .label("Time spent")
        .build()
        .into_node();
    let time_remaining = StyledInput::build(FieldId::EditIssueModal(
        EditIssueModalFieldId::TimeRemaining,
    ))
    .value(
        edit_issue_modal
            .payload
            .time_remaining
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_default(),
    )
    .valid(true)
    .build()
    .into_node();
    let time_remaining_field = StyledField::build()
        .input(time_remaining)
        .label("Time remaining")
        .build()
        .into_node();

    let inputs = div![
        class!["inputs"],
        div![class!["inputContainer"], time_spent_field],
        div![class!["inputContainer"], time_remaining_field]
    ];

    let close = StyledButton::build()
        .text("Done")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ModalDropped))
        .build()
        .into_node();

    StyledModal::build()
        .add_class("timeTrackingModal")
        .children(vec![
            modal_title,
            tracking,
            inputs,
            div![class!["actions"], close],
        ])
        .width(400)
        .build()
        .into_node()
}
