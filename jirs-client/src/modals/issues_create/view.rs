use {
    crate::{
        components::{
            styled_button::StyledButton, styled_date_time_input::StyledDateTimeInput,
            styled_field::StyledField, styled_form::StyledForm, styled_input::StyledInput,
            styled_modal::StyledModal, styled_select::StyledSelect,
            styled_textarea::StyledTextarea,
        },
        modals::{
            epic_field,
            issues_create::{Model as AddIssueModal, Type},
        },
        model::Model,
        shared::{IntoChild, ToChild, ToNode},
        FieldId, Msg,
    },
    jirs_data::{IssueFieldId, IssuePriority},
    seed::{prelude::*, *},
};

use crate::components::styled_button::ButtonVariant;
use crate::components::styled_select::SelectVariant;

pub fn view(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let issue_type = modal
        .type_state
        .values
        .get(0)
        .cloned()
        .map(Into::into)
        .unwrap_or_else(|| Type::Task);

    let issue_type_field = issue_type_field(modal);
    let form = StyledForm::build()
        .heading(issue_type.form_label())
        .add_field(issue_type_field)
        .add_field(crate::shared::divider());

    let form = match issue_type {
        Type::Epic => {
            let name_field = name_field(modal);

            let starts = StyledField::build()
                .input(
                    StyledDateTimeInput::build()
                        .state(&modal.epic_starts_at_state)
                        .build(FieldId::AddIssueModal(IssueFieldId::EpicStartsAt))
                        .into_node(),
                )
                .label("Starts at")
                .build()
                .into_node();

            let end = StyledField::build()
                .input(
                    StyledDateTimeInput::build()
                        .state(&modal.epic_ends_at_state)
                        .build(FieldId::AddIssueModal(IssueFieldId::EpicEndsAt))
                        .into_node(),
                )
                .label("Ends at")
                .build()
                .into_node();

            form.add_field(name_field).add_field(starts).add_field(end)
        }
        Type::Task | Type::Story | Type::Bug => {
            let short_summary_field = short_summary_field(modal);
            let description_field = description_field();
            let reporter_field = reporter_field(model, modal);
            let assignees_field = assignees_field(model, modal);
            let issue_priority_field = issue_priority_field(modal);
            let epic_field =
                epic_field(model, modal, FieldId::AddIssueModal(IssueFieldId::EpicName));

            form.add_field(short_summary_field)
                .add_field(description_field)
                .add_field(reporter_field)
                .add_field(assignees_field)
                .add_field(issue_priority_field)
                .try_field(epic_field)
        }
    };

    let submit = {
        StyledButton {
            variant: ButtonVariant::Primary,
            text: Some(issue_type.submit_label()),
            class_list: "action submit actionButton",
            on_click: Some(mouse_ev(Ev::Click, move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                Some(issue_type.submit_action())
            })),
            ..Default::default()
        }
        .into_node()
    };
    let cancel = StyledButton {
        variant: ButtonVariant::Empty,
        class_list: "action cancel actionButton",
        text: Some("Cancel"),
        on_click: Some(mouse_ev(Ev::Click, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Some(Msg::ModalDropped)
        })),
        ..Default::default()
    }
    .into_node();
    let actions = div![attrs![At::Class => "actions"], submit, cancel];

    let form = form.add_field(actions).build().into_node();

    StyledModal {
        class_list: "addIssue",
        width: Some(0),
        children: vec![form],
        ..Default::default()
    }
    .into_node()
}

fn issue_type_field(modal: &AddIssueModal) -> Node<Msg> {
    let select_type = StyledSelect {
        id: FieldId::AddIssueModal(IssueFieldId::Type),
        name: "type",
        variant: SelectVariant::Normal,
        text_filter: modal.type_state.text_filter.as_str(),
        opened: modal.type_state.opened,
        valid: true,
        options: Some(Type::Task.into_iter().map(|t| t.into_child().name("type"))),
        selected: vec![{
            let v: Type = modal
                .type_state
                .values
                .get(0)
                .cloned()
                .unwrap_or_default()
                .into();
            v
        }
        .into_child()
        .name("type")],
        ..Default::default()
    }
    .into_node();
    StyledField {
        label: "Issue Type",
        tip: Some("Start typing to get a list of possible matches."),
        input: select_type,
        ..Default::default()
    }
    .into_node()
}

#[inline]
fn short_summary_field(modal: &AddIssueModal) -> Node<Msg> {
    let short_summary = StyledInput {
        value: modal.title_state.value.as_str(),
        valid: modal.title_state.is_valid(),
        id: Some(FieldId::AddIssueModal(IssueFieldId::Title)),
        ..Default::default()
    }
    .into_node();
    StyledField {
        label: "Short Summary",
        tip: Some("Concisely summarize the issue in one or two sentences."),
        input: short_summary,
        ..Default::default()
    }
    .into_node()
}

fn description_field() -> Node<Msg> {
    let description = StyledTextarea {
        id: Some(FieldId::AddIssueModal(IssueFieldId::Description)),
        height: 110,
        class_list: "textarea",
        ..Default::default()
    }
    .into_node();
    StyledField {
        label: "Description",
        tip: Some("Describe the issue in as much detail as you'd like."),
        input: description,
        ..Default::default()
    }
    .into_node()
}

fn reporter_field(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let reporter_id = modal
        .reporter_id
        .or_else(|| model.user.as_ref().map(|u| u.id))
        .unwrap_or_default();
    let reporter = StyledSelect {
        id: FieldId::AddIssueModal(IssueFieldId::Reporter),
        variant: SelectVariant::Normal,
        text_filter: modal.reporter_state.text_filter.as_str(),
        opened: modal.reporter_state.opened,
        options: Some(model.users.iter().map(|u| u.to_child().name("reporter"))),
        selected: model
            .users
            .iter()
            .filter_map(|user| {
                if user.id == reporter_id {
                    Some(user.to_child().name("reporter"))
                } else {
                    None
                }
            })
            .collect(),

        valid: true,
        ..Default::default()
    }
    .into_node();
    StyledField {
        input: reporter,
        label: "Reporter",
        ..Default::default()
    }
    .into_node()
}

fn assignees_field(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let assignees = StyledSelect {
        id: FieldId::AddIssueModal(IssueFieldId::Assignees),
        variant: SelectVariant::Normal,
        is_multi: true,
        text_filter: modal.assignees_state.text_filter.as_str(),
        opened: modal.assignees_state.opened,
        options: Some(model.users.iter().map(|u| u.to_child().name("assignees"))),
        selected: model
            .users
            .iter()
            .filter_map(|user| {
                if modal.user_ids.contains(&user.id) {
                    Some(user.to_child().name("assignees"))
                } else {
                    None
                }
            })
            .collect(),

        valid: true,
        ..Default::default()
    }
    .into_node();
    StyledField::build()
        .input(assignees)
        .label("Assignees")
        .tip("")
        .build()
        .into_node()
}

fn issue_priority_field(modal: &AddIssueModal) -> Node<Msg> {
    let priorities = IssuePriority::default().into_iter();
    let select_priority = StyledSelect {
        id: FieldId::AddIssueModal(IssueFieldId::Priority),
        name: "priority",
        variant: SelectVariant::Normal,
        text_filter: modal.priority_state.text_filter.as_str(),
        opened: modal.priority_state.opened,
        valid: true,
        options: Some(priorities.map(|p| p.into_child().name("priority"))),
        selected: vec![modal.priority.into_child().name("priority")],
        ..Default::default()
    }
    .into_node();
    StyledField {
        label: "Issue Type",
        tip: Some("Priority in relation to other issues."),
        input: select_priority,
        ..Default::default()
    }
    .into_node()
}

fn name_field(modal: &AddIssueModal) -> Node<Msg> {
    let name = StyledInput {
        value: modal.title_state.value.as_str(),
        valid: modal.title_state.is_valid(),
        id: Some(FieldId::AddIssueModal(IssueFieldId::Title)),
        ..Default::default()
    }
    .into_node();
    StyledField {
        label: "Epic name",
        tip: Some("Describe upcoming feature."),
        input: name,
        ..Default::default()
    }
    .into_node()
}
