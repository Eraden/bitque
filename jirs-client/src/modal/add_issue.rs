use seed::{prelude::*, *};

use jirs_data::{IssuePriority, IssueType, User};

use crate::api::send_ws_msg;
use crate::model::{AddIssueModal, ModalType, Model};
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_icon::StyledIcon;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_modal::{StyledModal, Variant as ModalVariant};
use crate::shared::styled_select::StyledSelect;
use crate::shared::styled_select::StyledSelectChange;
use crate::shared::styled_textarea::StyledTextarea;
use crate::shared::ToNode;
use crate::{FieldId, Msg};

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
                issue_type: modal.issue_type.clone(),
                status: modal.status.clone(),
                priority: modal.priority.clone(),
                description: modal.description.clone(),
                description_text: modal.description_text.clone(),
                estimate: modal.estimate.clone(),
                time_spent: modal.time_spent.clone(),
                time_remaining: modal.time_remaining.clone(),
                project_id: modal.project_id.unwrap_or(project_id),
                user_ids: modal.user_ids.clone(),
                reporter_id: modal.reporter_id.unwrap_or_else(|| user_id),
            };
            send_ws_msg(jirs_data::WsMsg::IssueCreateRequest(payload));
        }
        Msg::WsMsg(jirs_data::WsMsg::IssueCreated(issue)) => {
            model.issues.push(issue.clone());
            orders.skip().send_msg(Msg::ModalDropped);
        }

        Msg::InputChanged(FieldId::DescriptionAddIssueModal, value) => {
            modal.description = Some(value.clone());
        }
        Msg::InputChanged(FieldId::SummaryAddIssueModal, value) => {
            modal.title = value.clone();
        }

        // IssueTypeAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::IssueTypeAddIssueModal,
            StyledSelectChange::Changed(id),
        ) => {
            modal.issue_type = (*id).into();
        }

        // ReporterAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::ReporterAddIssueModal,
            StyledSelectChange::Changed(id),
        ) => {
            modal.reporter_id = Some(*id as i32);
        }

        // AssigneesAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AssigneesAddIssueModal,
            StyledSelectChange::Changed(id),
        ) => {
            let id = *id as i32;
            if !modal.user_ids.contains(&id) {
                modal.user_ids.push(id);
            }
        }
        Msg::StyledSelectChanged(
            FieldId::AssigneesAddIssueModal,
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
            FieldId::IssuePriorityAddIssueModal,
            StyledSelectChange::Changed(id),
        ) => {
            modal.priority = (*id).into();
        }

        _ => (),
    }
    log!(modal);
}

pub fn view(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let select_type = StyledSelect::build(FieldId::IssueTypeAddIssueModal)
        .name("type")
        .normal()
        .text_filter(modal.type_state.text_filter.as_str())
        .opened(modal.type_state.opened)
        .valid(true)
        .options(vec![
            IssueTypeOption(IssueType::Story),
            IssueTypeOption(IssueType::Task),
            IssueTypeOption(IssueType::Bug),
        ])
        .selected(vec![IssueTypeOption(modal.issue_type.clone())])
        .build()
        .into_node();
    let issue_type_field = StyledField::build()
        .label("Issue Type")
        .tip("Start typing to get a list of possible matches.")
        .input(select_type)
        .build()
        .into_node();

    let short_summary = StyledInput::build(FieldId::SummaryAddIssueModal)
        .valid(true)
        .build()
        .into_node();
    let short_summary_field = StyledField::build()
        .label("Short Summary")
        .tip("Concisely summarize the issue in one or two sentences.")
        .input(short_summary)
        .build()
        .into_node();

    let description = StyledTextarea::build()
        .height(110)
        .build(FieldId::DescriptionAddIssueModal)
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
    let reporter = StyledSelect::build(FieldId::ReporterAddIssueModal)
        .normal()
        .text_filter(modal.reporter_state.text_filter.as_str())
        .opened(modal.reporter_state.opened)
        .options(model.users.iter().map(|u| UserOption(u)).collect())
        .selected(
            model
                .users
                .iter()
                .filter_map(|user| {
                    if user.id == reporter_id {
                        Some(UserOption(user))
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

    let assignees = StyledSelect::build(FieldId::AssigneesAddIssueModal)
        .normal()
        .multi()
        .text_filter(modal.assignees_state.text_filter.as_str())
        .opened(modal.assignees_state.opened)
        .options(model.users.iter().map(|u| UserOption(u)).collect())
        .selected(
            model
                .users
                .iter()
                .filter_map(|user| {
                    if modal.user_ids.contains(&user.id) {
                        Some(UserOption(user))
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

    let select_priority = StyledSelect::build(FieldId::IssuePriorityAddIssueModal)
        .name("priority")
        .normal()
        .text_filter(modal.priority_state.text_filter.as_str())
        .opened(modal.priority_state.opened)
        .valid(true)
        .options(vec![
            IssuePriorityOption(IssuePriority::Highest),
            IssuePriorityOption(IssuePriority::High),
            IssuePriorityOption(IssuePriority::Medium),
            IssuePriorityOption(IssuePriority::Low),
            IssuePriorityOption(IssuePriority::Lowest),
        ])
        .selected(vec![IssuePriorityOption(modal.priority.clone())])
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

#[derive(PartialOrd, PartialEq, Debug)]
pub struct IssueTypeOption(pub IssueType);

impl crate::shared::styled_select::SelectOption for IssueTypeOption {
    fn into_option(self) -> Node<Msg> {
        let name = self.0.to_label().to_owned();

        let icon = StyledIcon::build(self.0.into())
            .add_class("issueTypeIcon")
            .build()
            .into_node();

        div![
            attrs![At::Class => "type optionItem"],
            icon,
            div![attrs![At::Class => "typeLabel optionLabel"], name]
        ]
    }

    fn into_value(self) -> Node<Msg> {
        let name = self.0.to_label().to_owned();

        let type_icon = StyledIcon::build(self.0.into()).build().into_node();

        div![
            attrs![At::Class => "selectItem"],
            type_icon,
            div![attrs![At::Class => "selectItemLabel"], name]
        ]
    }

    fn match_text_filter(&self, text_filter: &str) -> bool {
        self.0
            .to_string()
            .to_lowercase()
            .contains(&text_filter.to_lowercase())
    }

    fn to_value(&self) -> u32 {
        self.0.clone().into()
    }
}

#[derive(Debug, PartialEq)]
pub struct IssuePriorityOption(IssuePriority);

impl crate::shared::styled_select::SelectOption for IssuePriorityOption {
    fn into_option(self) -> Node<Msg> {
        let name = format!("{}", self.0);

        let icon = StyledIcon::build(self.0.into())
            .add_class("issuePriorityIcon")
            .size(18)
            .build()
            .into_node();

        div![
            attrs![At::Class => format!("priority optionItem {}", name)],
            icon,
            div![attrs![At::Class => "priorityLabel optionLabel"], name]
        ]
    }

    fn into_value(self) -> Node<Msg> {
        let name = format!("{}", self.0);

        let type_icon = StyledIcon::build(self.0.into()).build().into_node();

        div![
            attrs![At::Class => format!("selectItem priority {}", name)],
            type_icon,
            div![attrs![At::Class => "selectItemLabel"], name]
        ]
    }

    fn match_text_filter(&self, text_filter: &str) -> bool {
        self.0
            .to_string()
            .to_lowercase()
            .contains(&text_filter.to_lowercase())
    }

    fn to_value(&self) -> u32 {
        self.0.clone().into()
    }
}

#[derive(Debug, PartialEq)]
pub struct UserOption<'opt>(pub &'opt User);

impl<'opt> UserOption<'opt> {
    fn avatar_node(&self) -> Node<Msg> {
        let user = self.0;
        StyledAvatar::build()
            .avatar_url(user.avatar_url.as_ref().cloned().unwrap_or_default())
            .size(20)
            .name(user.name.as_str())
            .build()
            .into_node()
    }
}

impl<'opt> crate::shared::styled_select::SelectOption for UserOption<'opt> {
    fn into_option(self) -> Node<Msg> {
        let user = self.0;

        let styled_avatar = self.avatar_node();

        div![
            attrs![At::Class => "user optionItem"],
            styled_avatar,
            div![attrs![At::Class => "typeLabel optionLabel"], user.name]
        ]
    }

    fn into_value(self) -> Node<Msg> {
        let user = self.0;

        let styled_avatar = self.avatar_node();

        div![
            attrs![At::Class => "selectItem"],
            styled_avatar,
            div![attrs![At::Class => "selectItemLabel"], user.name]
        ]
    }

    fn match_text_filter(&self, text_filter: &str) -> bool {
        self.0.name.contains(text_filter)
    }

    fn to_value(&self) -> u32 {
        self.0.id as u32
    }
}
