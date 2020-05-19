use seed::{prelude::*, *};

use jirs_data::{IssueFieldId, WsMsg};
use jirs_data::{IssuePriority, IssueType, ToVec};

use crate::model::{AddIssueModal, ModalType, Model};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_modal::{StyledModal, Variant as ModalVariant};
use crate::shared::styled_select::StyledSelect;
use crate::shared::styled_select::StyledSelectChange;
use crate::shared::styled_textarea::StyledTextarea;
use crate::shared::{ToChild, ToNode};
use crate::ws::send_ws_msg;
use crate::{FieldId, Msg, WebSocketChanged};

pub fn update(msg: &Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    let modal = model.modals.iter_mut().find(|modal| match modal {
        ModalType::AddIssue(..) => true,
        _ => false,
    });
    let modal = match modal {
        Some(ModalType::AddIssue(modal)) => modal,
        _ => return,
    };

    modal.assignees_state.update(msg, orders);
    modal.reporter_state.update(msg, orders);
    modal.type_state.update(msg, orders);
    modal.priority_state.update(msg, orders);

    match msg {
        Msg::AddIssue => {
            let user_id = model.user.as_ref().map(|u| u.id).unwrap_or_default();
            let project_id = model.project.as_ref().map(|p| p.id).unwrap_or_default();

            let payload = jirs_data::CreateIssuePayload {
                title: modal.title.clone(),
                issue_type: modal.issue_type,
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
            };
            send_ws_msg(
                jirs_data::WsMsg::IssueCreateRequest(payload),
                model.ws.as_ref(),
            );
        }
        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::IssueCreated(issue))) => {
            model.issues.push(issue.clone());
            orders.skip().send_msg(Msg::ModalDropped);
        }

        Msg::StrInputChanged(FieldId::AddIssueModal(IssueFieldId::Description), value) => {
            modal.description = Some(value.clone());
        }
        Msg::StrInputChanged(FieldId::AddIssueModal(IssueFieldId::Title), value) => {
            modal.title = value.clone();
        }

        // IssueTypeAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Type),
            StyledSelectChange::Changed(id),
        ) => {
            modal.issue_type = (*id).into();
        }

        // ReporterAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Reporter),
            StyledSelectChange::Changed(id),
        ) => {
            modal.reporter_id = Some(*id as i32);
        }

        // AssigneesAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Assignees),
            StyledSelectChange::Changed(id),
        ) => {
            let id = *id as i32;
            if !modal.user_ids.contains(&id) {
                modal.user_ids.push(id);
            }
        }
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Assignees),
            StyledSelectChange::RemoveMulti(id),
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
            StyledSelectChange::Changed(id),
        ) => {
            modal.priority = (*id).into();
        }

        _ => (),
    }
}

pub fn view(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let select_type = StyledSelect::build(FieldId::AddIssueModal(IssueFieldId::Type))
        .name("type")
        .normal()
        .text_filter(modal.type_state.text_filter.as_str())
        .opened(modal.type_state.opened)
        .valid(true)
        .options(
            IssueType::ordered()
                .iter()
                .map(|t| t.to_child().name("type"))
                .collect(),
        )
        .selected(vec![modal.issue_type.to_child().name("type")])
        .build()
        .into_node();
    let issue_type_field = StyledField::build()
        .label("Issue Type")
        .tip("Start typing to get a list of possible matches.")
        .input(select_type)
        .build()
        .into_node();

    let short_summary = StyledInput::build(FieldId::AddIssueModal(IssueFieldId::Title))
        .valid(true)
        .build()
        .into_node();
    let short_summary_field = StyledField::build()
        .label("Short Summary")
        .tip("Concisely summarize the issue in one or two sentences.")
        .input(short_summary)
        .build()
        .into_node();

    let description = StyledTextarea::build(FieldId::AddIssueModal(IssueFieldId::Description))
        .height(110)
        .add_class("textarea")
        .build()
        .into_node();
    let description_field = StyledField::build()
        .label("Description")
        .tip("Describe the issue in as much detail as you'd like.")
        .input(description)
        .build()
        .into_node();

    let reporter_id = modal
        .reporter_id
        .or_else(|| model.user.as_ref().map(|u| u.id))
        .unwrap_or_default();
    let reporter = StyledSelect::build(FieldId::AddIssueModal(IssueFieldId::Reporter))
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
        .build()
        .into_node();
    let reporter_field = StyledField::build()
        .input(reporter)
        .label("Reporter")
        .tip("")
        .build()
        .into_node();

    let assignees = StyledSelect::build(FieldId::AddIssueModal(IssueFieldId::Assignees))
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
        .build()
        .into_node();
    let assignees_field = StyledField::build()
        .input(assignees)
        .label("Assignees")
        .tip("")
        .build()
        .into_node();

    let select_priority = StyledSelect::build(FieldId::AddIssueModal(IssueFieldId::Priority))
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
        .build()
        .into_node();
    let issue_priority_field = StyledField::build()
        .label("Issue Type")
        .tip("Priority in relation to other issues.")
        .input(select_priority)
        .build()
        .into_node();

    let submit = StyledButton::build()
        .primary()
        .text("Create Issue")
        .add_class("action")
        .add_class("submit")
        .add_class("ActionButton")
        .on_click(mouse_ev(Ev::Click, |ev| {
            ev.stop_propagation();
            Msg::AddIssue
        }))
        .build()
        .into_node();
    let cancel = StyledButton::build()
        .empty()
        .add_class("action")
        .add_class("cancel")
        .add_class("actionButton")
        .text("Cancel")
        .on_click(mouse_ev(Ev::Click, |ev| {
            ev.stop_propagation();
            Msg::ModalDropped
        }))
        .build()
        .into_node();
    let actions = div![attrs![At::Class => "actions"], submit, cancel];

    let form = StyledForm::build()
        .heading("Create issue")
        .add_field(issue_type_field)
        .add_field(crate::shared::divider())
        .add_field(short_summary_field)
        .add_field(description_field)
        .add_field(reporter_field)
        .add_field(assignees_field)
        .add_field(issue_priority_field)
        .add_field(actions)
        .build()
        .into_node();

    StyledModal::build()
        .width(800)
        .add_class("add-issue")
        .variant(ModalVariant::Center)
        .children(vec![form])
        .build()
        .into_node()
}
