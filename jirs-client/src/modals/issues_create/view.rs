use {
    crate::{
        modal::issues::epic_field,
        modals::issues_create::{Model as AddIssueModal, Type},
        model::Model,
        shared::{
            styled_button::StyledButton, styled_date_time_input::StyledDateTimeInput,
            styled_field::StyledField, styled_form::StyledForm, styled_input::StyledInput,
            styled_modal::StyledModal, styled_select::StyledSelect,
            styled_textarea::StyledTextarea, IntoChild, ToChild, ToNode,
        },
        FieldId, Msg,
    },
    jirs_data::{IssueFieldId, IssuePriority},
    seed::{prelude::*, *},
};

pub fn view(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let issue_type = modal
        .type_state
        .values
        .get(0)
        .cloned()
        .map(Type::from)
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
        StyledButton::build()
            .primary()
            .text(issue_type.submit_label())
            .add_class("action")
            .add_class("submit")
            .add_class("actionButton")
            .on_click(mouse_ev(Ev::Click, move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                Some(issue_type.submit_action())
            }))
            .build()
            .into_node()
    };
    let cancel = StyledButton::build()
        .empty()
        .add_class("action")
        .add_class("cancel")
        .add_class("actionButton")
        .text("Cancel")
        .on_click(mouse_ev(Ev::Click, |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Some(Msg::ModalDropped)
        }))
        .build()
        .into_node();
    let actions = div![attrs![At::Class => "actions"], submit, cancel];

    let form = form.add_field(actions).build().into_node();

    StyledModal::build()
        .add_class("addIssue")
        .width(0)
        .variant(crate::shared::styled_modal::Variant::Center)
        .children(vec![form])
        .build()
        .into_node()
}

fn issue_type_field(modal: &AddIssueModal) -> Node<Msg> {
    let select_type = StyledSelect::build()
        .name("type")
        .normal()
        .text_filter(modal.type_state.text_filter.as_str())
        .opened(modal.type_state.opened)
        .valid(true)
        .options(Type::ordered().iter().map(|t| t.to_child().name("type")))
        .selected(vec![Type::from(
            modal.type_state.values.get(0).cloned().unwrap_or_default(),
        )
        .to_child()
        .name("type")])
        .build(FieldId::AddIssueModal(IssueFieldId::Type))
        .into_node();
    StyledField::build()
        .label("Issue Type")
        .tip("Start typing to get a list of possible matches.")
        .input(select_type)
        .build()
        .into_node()
}

#[inline]
fn short_summary_field(modal: &AddIssueModal) -> Node<Msg> {
    let short_summary = StyledInput::build()
        .state(&modal.title_state)
        .build(FieldId::AddIssueModal(IssueFieldId::Title))
        .into_node();
    StyledField::build()
        .label("Short Summary")
        .tip("Concisely summarize the issue in one or two sentences.")
        .input(short_summary)
        .build()
        .into_node()
}

fn description_field() -> Node<Msg> {
    let description = StyledTextarea::build(FieldId::AddIssueModal(IssueFieldId::Description))
        .height(110)
        .add_class("textarea")
        .build()
        .into_node();
    StyledField::build()
        .label("Description")
        .tip("Describe the issue in as much detail as you'd like.")
        .input(description)
        .build()
        .into_node()
}

fn reporter_field(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let reporter_id = modal
        .reporter_id
        .or_else(|| model.user.as_ref().map(|u| u.id))
        .unwrap_or_default();
    let reporter = StyledSelect::build()
        .normal()
        .text_filter(modal.reporter_state.text_filter.as_str())
        .opened(modal.reporter_state.opened)
        .options(model.users.iter().map(|u| u.to_child().name("reporter")))
        .selected(
            model
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
        )
        .valid(true)
        .build(FieldId::AddIssueModal(IssueFieldId::Reporter))
        .into_node();
    StyledField::build()
        .input(reporter)
        .label("Reporter")
        .tip("")
        .build()
        .into_node()
}

fn assignees_field(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let assignees = StyledSelect::build()
        .normal()
        .multi()
        .text_filter(modal.assignees_state.text_filter.as_str())
        .opened(modal.assignees_state.opened)
        .options(model.users.iter().map(|u| u.to_child().name("assignees")))
        .selected(
            model
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
        )
        .valid(true)
        .build(FieldId::AddIssueModal(IssueFieldId::Assignees))
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
    let select_priority = StyledSelect::build()
        .name("priority")
        .normal()
        .text_filter(modal.priority_state.text_filter.as_str())
        .opened(modal.priority_state.opened)
        .valid(true)
        .options(priorities.map(|p| p.into_child().name("priority")))
        .selected(vec![modal.priority.into_child().name("priority")])
        .build(FieldId::AddIssueModal(IssueFieldId::Priority))
        .into_node();
    StyledField::build()
        .label("Issue Type")
        .tip("Priority in relation to other issues.")
        .input(select_priority)
        .build()
        .into_node()
}

fn name_field(modal: &AddIssueModal) -> Node<Msg> {
    let name = StyledInput::build()
        .state(&modal.title_state)
        .build(FieldId::AddIssueModal(IssueFieldId::Title))
        .into_node();
    StyledField::build()
        .label("Epic name")
        .tip("Describe upcoming feature.")
        .input(name)
        .build()
        .into_node()
}
