use jirs_data::{IssueFieldId, IssueId, TimeTracking};
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::StyledButton;
use crate::components::styled_field::StyledField;
use crate::components::styled_input::{StyledInput, StyledInputState};
use crate::components::styled_modal::StyledModal;
use crate::components::styled_select::{StyledSelect, StyledSelectState};
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::Model;
use crate::shared::tracking_widget::{fibonacci_value_name, fibonacci_values, tracking_widget};
use crate::{EditIssueModalSection, FieldId, Msg};

pub fn value_for_time_tracking(v: &Option<i32>, time_tracking_type: &TimeTracking) -> String {
    match (time_tracking_type, v.as_ref()) {
        (TimeTracking::Untracked, _) => "".to_string(),
        (TimeTracking::Fibonacci, Some(n)) => n.to_string(),
        (TimeTracking::Hourly, Some(n)) => format!("{:.1}", *n as f64 / 10.0f64),
        _ => "".to_string(),
    }
}

pub fn view(model: &Model, modal: &super::Model) -> Node<Msg> {
    let issue_id: IssueId = modal.issue_id;
    if model.issues_by_id.get(&issue_id).is_none() {
        return Node::Empty;
    }

    let edit_issue_modal = match &model.modals().edit_issue {
        Some(modal) => modal,
        _ => return Node::Empty,
    };

    let time_tracking_type = model
        .project
        .as_ref()
        .map(|p| p.time_tracking)
        .unwrap_or(TimeTracking::Untracked);

    let modal_title = div![C!["modalTitle"], "Time tracking"];

    let tracking = tracking_widget(model, edit_issue_modal);

    let time_spent_field = time_tracking_field(
        time_tracking_type,
        FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeSpent)),
        "Time spent",
        &edit_issue_modal.time_spent,
        &edit_issue_modal.time_spent_select,
    );
    let time_remaining_field = time_tracking_field(
        time_tracking_type,
        FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeRemaining)),
        "Time remaining",
        &edit_issue_modal.time_remaining,
        &edit_issue_modal.time_remaining_select,
    );

    let inputs = div![
        C!["inputs"],
        div![C!["inputContainer"], time_spent_field],
        div![C!["inputContainer"], time_remaining_field]
    ];

    let close = StyledButton {
        text: Some("Done"),
        on_click: Some(mouse_ev(Ev::Click, |_| Msg::ModalDropped)),
        ..Default::default()
    }
    .render();

    StyledModal {
        class_list: "timeTrackingModal",
        children: vec![modal_title, tracking, inputs, div![C!["actions"], close]],
        width: Some(400),
        ..Default::default()
    }
    .render()
}

#[inline(always)]
pub fn time_tracking_field(
    time_tracking_type: TimeTracking,
    field_id: FieldId,
    label: &str,
    input_state: &StyledInputState,
    select_state: &StyledSelectState,
) -> Node<Msg> {
    let fibonacci_values = fibonacci_values();
    let input = match time_tracking_type {
        TimeTracking::Untracked => empty![],
        TimeTracking::Fibonacci => StyledSelect {
            id: field_id,
            selected: select_state
                .values
                .iter()
                .copied()
                .map(fibonacci_value_select_option)
                .collect(),

            text_filter: select_state.text_filter.as_str(),
            opened: select_state.opened,
            options: Some(
                fibonacci_values
                    .iter()
                    .copied()
                    .map(fibonacci_value_select_option),
            ),
            ..Default::default()
        }
        .render(),
        TimeTracking::Hourly => StyledInput {
            valid: input_state.is_valid(),
            value: input_state.value.as_str(),
            id: Some(field_id),
            ..Default::default()
        }
        .render(),
    };
    StyledField {
        label,
        input,
        ..Default::default()
    }
    .render()
}

fn fibonacci_value_select_option<'l>(value: u32) -> StyledSelectOption<'l> {
    let name = fibonacci_value_name(value);

    StyledSelectOption {
        class_list: name,
        text: Some(name),
        value,
        ..Default::default()
    }
}
