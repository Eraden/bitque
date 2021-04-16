use jirs_data::{IssueFieldId, IssuePriority, User};
use seed::prelude::*;
use seed::*;

use crate::components::styled_avatar::StyledAvatar;
use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_date_time_input::StyledDateTimeInput;
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::StyledInput;
use crate::components::styled_modal::StyledModal;
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectChild;
use crate::components::styled_textarea::StyledTextarea;
use crate::modals::epic_field;
use crate::modals::issues_create::{Model as AddIssueModal, Type};
use crate::model::Model;
use crate::shared::ToNode;
use crate::{FieldId, Msg};

pub fn view(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let issue_type = modal
        .type_state
        .values
        .get(0)
        .cloned()
        .map(Into::into)
        .unwrap_or_else(|| Type::Task);

    let issue_type_field = issue_type_field(modal);
    let mut form = StyledForm {
        heading: issue_type.form_label(),
        fields: vec![issue_type_field, crate::shared::divider()],
        ..Default::default()
    };

    match issue_type {
        Type::Epic => {
            let name_field = name_field(modal);

            let starts = StyledField {
                input: StyledDateTimeInput {
                    field_id: FieldId::AddIssueModal(IssueFieldId::EpicStartsAt),
                    popup_visible: modal.epic_starts_at_state.popup_visible,
                    timestamp: modal.epic_starts_at_state.timestamp.clone(),
                }
                .into_node(),
                label: "Starts at",
                ..Default::default()
            }
            .into_node();

            let end = StyledField {
                input: StyledDateTimeInput {
                    field_id: FieldId::AddIssueModal(IssueFieldId::EpicEndsAt),
                    popup_visible: modal.epic_ends_at_state.popup_visible,
                    timestamp: modal.epic_ends_at_state.timestamp.clone(),
                }
                .into_node(),
                label: "Ends at",
                ..Default::default()
            }
            .into_node();

            form.fields.push(name_field);
            form.fields.push(starts);
            form.fields.push(end)
        }
        Type::Task | Type::Story | Type::Bug => {
            let short_summary_field = short_summary_field(modal);
            let description_field = description_field();
            let reporter_field = reporter_field(model, modal);
            let assignees_field = assignees_field(model, modal);
            let issue_priority_field = issue_priority_field(modal);
            let epic_field =
                epic_field(model, modal, FieldId::AddIssueModal(IssueFieldId::EpicName));

            form.fields.push(short_summary_field);
            form.fields.push(description_field);
            form.fields.push(reporter_field);
            form.fields.push(assignees_field);
            form.fields.push(issue_priority_field);
            if let Some(field) = epic_field {
                form.fields.push(field);
            }
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

    form.fields.push(actions);

    StyledModal {
        class_list: "addIssue",
        width: Some(0),
        children: vec![form.into_node()],
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
        options: Some(Type::Task.into_iter().map(type_select_option)),
        selected: vec![type_select_option(
            modal
                .type_state
                .values
                .get(0)
                .cloned()
                .unwrap_or_default()
                .into(),
        )],
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

#[inline(always)]
fn type_select_option<'l>(t: Type) -> StyledSelectChild<'l> {
    let name = match t {
        Type::Task => "Task",
        Type::Bug => "Bug",
        Type::Story => "Story",
        Type::Epic => "Epic",
    };

    StyledSelectChild {
        class_list: name,
        text: Some(name),
        icon: Some(
            StyledIcon {
                icon: match t {
                    Type::Task => Icon::Task,
                    Type::Bug => Icon::Bug,
                    Type::Story => Icon::Story,
                    Type::Epic => Icon::Epic,
                },
                class_list: name,
                ..Default::default()
            }
            .into_node(),
        ),
        name: Some("type"),
        value: t.into(),
        ..Default::default()
    }
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
        options: Some(model.users.iter().map(reporter_select_option)),
        selected: model
            .users
            .iter()
            .filter_map(|user| {
                if user.id == reporter_id {
                    Some(reporter_select_option(user))
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

#[inline(always)]
fn reporter_select_option(user: &User) -> StyledSelectChild {
    StyledSelectChild {
        value: user.id as u32,
        icon: Some(
            StyledAvatar {
                size: 20,
                name: &user.name,
                avatar_url: user.avatar_url.as_deref(),
                ..StyledAvatar::default()
            }
            .into_node(),
        ),
        text: Some(user.name.as_str()),
        name: Some("reporter"),
        ..Default::default()
    }
}

fn assignees_field(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let assignees = StyledSelect {
        id: FieldId::AddIssueModal(IssueFieldId::Assignees),
        variant: SelectVariant::Normal,
        is_multi: true,
        text_filter: modal.assignees_state.text_filter.as_str(),
        opened: modal.assignees_state.opened,
        options: Some(model.users.iter().map(assignee_select_option)),
        selected: model
            .users
            .iter()
            .filter_map(|user| {
                if modal.user_ids.contains(&user.id) {
                    Some(assignee_select_option(user))
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
        input: assignees,
        label: "Assignees",
        tip: Some(""),
        ..Default::default()
    }
    .into_node()
}

#[inline(always)]
fn assignee_select_option(user: &User) -> StyledSelectChild {
    StyledSelectChild {
        value: user.id as u32,
        icon: Some(
            StyledAvatar {
                size: 20,
                name: &user.name,
                avatar_url: user.avatar_url.as_deref(),
                ..StyledAvatar::default()
            }
            .into_node(),
        ),
        text: Some(user.name.as_str()),
        name: Some("assignees"),
        ..Default::default()
    }
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
        options: Some(priorities.map(priority_select_option)),
        selected: vec![priority_select_option(modal.priority)],
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

fn priority_select_option<'l>(priority: IssuePriority) -> StyledSelectChild<'l> {
    StyledSelectChild {
        icon: Some(
            StyledIcon {
                icon: priority.clone().into(),
                class_list: priority.to_str(),
                ..Default::default()
            }
            .into_node(),
        ),
        text: Some(priority.to_str()),
        class_list: priority.to_str(),
        value: priority.into(),
        name: Some("priority"),
        ..Default::default()
    }
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
