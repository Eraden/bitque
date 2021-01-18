use {
    crate::{
        components::{
            styled_button::StyledButton,
            styled_field::StyledField,
            styled_input::{StyledInput, StyledInputState},
            styled_modal::StyledModal,
            styled_select::{StyledSelect, StyledSelectState},
        },
        model::Model,
        shared::{
            tracking_widget::{fibonacci_values, tracking_widget},
            ToChild, ToNode,
        },
        EditIssueModalSection, FieldId, Msg,
    },
    jirs_data::{IssueFieldId, IssueId, TimeTracking},
    seed::{prelude::*, *},
};

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

    let close = StyledButton::build()
        .text("Done")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ModalDropped))
        .build()
        .into_node();

    StyledModal::build()
        .add_class("timeTrackingModal")
        .children(vec![modal_title, tracking, inputs, div![C!["actions"], close]].into_iter())
        .width(400)
        .build()
        .into_node()
}

#[inline]
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
        TimeTracking::Fibonacci => StyledSelect::build()
            .selected(
                select_state
                    .values
                    .iter()
                    .map(|n| (*n).to_child())
                    .collect(),
            )
            .state(select_state)
            .options(fibonacci_values.iter().map(|v| v.to_child()))
            .build(field_id)
            .into_node(),
        TimeTracking::Hourly => StyledInput::build()
            .state(input_state)
            .valid(true)
            .build(field_id)
            .into_node(),
    };
    StyledField::build()
        .input(input)
        .label(label)
        .build()
        .into_node()
}
