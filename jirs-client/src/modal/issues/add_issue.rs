use seed::{prelude::*, *};

use jirs_data::{IssueFieldId, IssuePriority, ToVec, UserId, WsMsg};

use crate::shared::styled_date_time_input::StyledDateTimeInput;
use crate::{
    modal::issues::epic_field,
    model::{AddIssueModal, IssueModal, ModalType, Model},
    shared::{
        styled_button::StyledButton,
        styled_field::StyledField,
        styled_form::StyledForm,
        styled_input::StyledInput,
        styled_modal::{StyledModal, Variant as ModalVariant},
        styled_select::StyledSelect,
        styled_select::StyledSelectChanged,
        styled_select_child::{StyledSelectChild, StyledSelectChildBuilder},
        styled_textarea::StyledTextarea,
        ToChild, ToNode,
    },
    ws::send_ws_msg,
    FieldId, Msg, WebSocketChanged,
};

#[derive(Copy, Clone)]
enum Type {
    Task,
    Bug,
    Story,
    Epic,
}

impl From<u32> for Type {
    fn from(n: u32) -> Self {
        match n {
            0 => Type::Task,
            1 => Type::Bug,
            2 => Type::Story,
            3 => Type::Epic,
            _ => Type::Task,
        }
    }
}

impl Type {
    fn ordered<'l>() -> &'l [Type] {
        use Type::*;
        &[Task, Bug, Story, Epic]
    }

    fn submit_label(&self) -> &str {
        use Type::*;
        match self {
            Epic => "Create epic",
            Bug | Task | Story => "Create issue",
        }
    }

    fn form_label(&self) -> &str {
        use Type::*;
        match self {
            Epic => "Create epic",
            Bug | Task | Story => "Create issue",
        }
    }

    fn submit_action(&self) -> Msg {
        use Type::*;
        match self {
            Epic => Msg::AddEpic,
            Bug | Task | Story => Msg::AddIssue,
        }
    }
}

impl ToChild for Type {
    type Builder = StyledSelectChildBuilder;

    fn to_child(&self) -> Self::Builder {
        let name = match self {
            Type::Task => "Task",
            Type::Bug => "Bug",
            Type::Story => "Story",
            Type::Epic => "Epic",
        };
        let value = match self {
            Type::Task => 0,
            Type::Bug => 1,
            Type::Story => 2,
            Type::Epic => 3,
        };
        let icon = match self {
            Type::Task => crate::shared::styled_icon::Icon::Task,
            Type::Bug => crate::shared::styled_icon::Icon::Bug,
            Type::Story => crate::shared::styled_icon::Icon::Story,
            Type::Epic => crate::shared::styled_icon::Icon::Epic,
        };

        let type_icon = crate::shared::styled_icon::StyledIcon::build(icon)
            .add_class(name)
            .build()
            .into_node();

        StyledSelectChild::build()
            .add_class(name)
            .text(name)
            .icon(type_icon)
            .value(value)
    }
}

pub fn update(msg: &Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    let modal = model.modals.iter_mut().find(|modal| match modal {
        ModalType::AddIssue(..) => true,
        _ => false,
    });
    let modal = match modal {
        Some(ModalType::AddIssue(modal)) => modal,
        _ => return,
    };

    modal.update_states(msg, orders);

    match msg {
        Msg::AddEpic => {
            send_ws_msg(
                WsMsg::EpicCreate(modal.title_state.value.clone()),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::AddIssue => {
            let user_id = model.user.as_ref().map(|u| u.id).unwrap_or_default();
            let project_id = model.project.as_ref().map(|p| p.id).unwrap_or_default();
            let type_value = modal.type_state.values.get(0).cloned().unwrap_or_default();
            match type_value {
                0 | 1 | 2 => {
                    let issue_type = type_value.into();
                    let payload = jirs_data::CreateIssuePayload {
                        title: modal.title_state.value.clone(),
                        issue_type,
                        issue_status_id: modal.issue_status_id,
                        priority: modal.priority,
                        description: modal.description.clone(),
                        description_text: modal.description_text.clone(),
                        estimate: modal.estimate,
                        time_spent: modal.time_spent,
                        time_remaining: modal.time_remaining,
                        project_id: modal.project_id.unwrap_or(project_id),
                        user_ids: modal.user_ids.clone(),
                        reporter_id: modal.reporter_id.unwrap_or_else(|| user_id),
                        epic_id: modal.epic_id,
                    };

                    send_ws_msg(
                        jirs_data::WsMsg::IssueCreate(payload),
                        model.ws.as_ref(),
                        orders,
                    );
                }
                _ => {
                    //
                }
            };
        }

        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::IssueCreated(issue))) => {
            model.issues.push(issue.clone());
            orders.skip().send_msg(Msg::ModalDropped);
        }

        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::EpicCreated(_))) => {
            orders.skip().send_msg(Msg::ModalDropped);
        }

        Msg::StrInputChanged(FieldId::AddIssueModal(IssueFieldId::Description), value) => {
            modal.description = Some(value.clone());
        }

        // ReporterAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Reporter),
            StyledSelectChanged::Changed(id),
        ) => {
            modal.reporter_id = id.map(|n| n as UserId);
        }

        // AssigneesAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Assignees),
            StyledSelectChanged::Changed(Some(id)),
        ) => {
            let id = *id as UserId;
            if !modal.user_ids.contains(&id) {
                modal.user_ids.push(id);
            }
        }
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Assignees),
            StyledSelectChanged::RemoveMulti(id),
        ) => {
            let id = *id as i32;
            let mut old: Vec<i32> = vec![];
            std::mem::swap(&mut old, &mut modal.user_ids);
            for user_id in old {
                if id != user_id {
                    modal.user_ids.push(user_id);
                }
            }
        }

        // IssuePriorityAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Priority),
            StyledSelectChanged::Changed(Some(id)),
        ) => {
            modal.priority = (*id).into();
        }

        _ => (),
    }
}

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
        .variant(ModalVariant::Center)
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
        .options(
            Type::ordered()
                .iter()
                .map(|t| t.to_child().name("type"))
                .collect(),
        )
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
        .options(
            model
                .users
                .iter()
                .map(|u| u.to_child().name("reporter"))
                .collect(),
        )
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
        .options(
            model
                .users
                .iter()
                .map(|u| u.to_child().name("assignees"))
                .collect(),
        )
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
    let select_priority = StyledSelect::build()
        .name("priority")
        .normal()
        .text_filter(modal.priority_state.text_filter.as_str())
        .opened(modal.priority_state.opened)
        .valid(true)
        .options(
            IssuePriority::ordered()
                .iter()
                .map(|p| p.to_child().name("priority"))
                .collect(),
        )
        .selected(vec![modal.priority.to_child().name("priority")])
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
