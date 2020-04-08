use seed::{prelude::*, *};

use jirs_data::{IssueType, User};

use crate::model::Page;
use crate::model::{AddIssueModal, ModalType, Model};
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

pub fn update(msg: &Msg, model: &mut crate::model::Model, _orders: &mut impl Orders<Msg>) {
    let modal = model.modals.iter_mut().find(|modal| match modal {
        ModalType::AddIssue(..) => true,
        _ => false,
    });
    let modal = match modal {
        Some(ModalType::AddIssue(modal)) => modal,
        _ => return,
    };

    match msg {
        Msg::InputChanged(FieldId::DescriptionAddIssueModal, value) => {
            modal.description = Some(value.clone());
        }
        Msg::InputChanged(FieldId::SummaryAddIssueModal, value) => {
            modal.title = value.clone();
        }
        Msg::StyledSelectChanged(
            FieldId::IssueTypeAddIssueModal,
            StyledSelectChange::DropDownVisibility(b),
        ) => {
            modal.type_select_opened = *b;
        }
        Msg::StyledSelectChanged(
            FieldId::IssueTypeAddIssueModal,
            StyledSelectChange::Text(text),
        ) => {
            modal.type_select_filter = text.clone();
        }
        Msg::StyledSelectChanged(
            FieldId::IssueTypeAddIssueModal,
            StyledSelectChange::Changed(id),
        ) => {
            modal.issue_type = (*id).into();
        }
        _ => (),
    }
    log!(modal);
}

pub fn view(model: &Model, modal: &AddIssueModal) -> Node<Msg> {
    let select_type = StyledSelect::build(FieldId::IssueTypeAddIssueModal)
        .name("type")
        .normal()
        .text_filter(modal.type_select_filter.as_str())
        .opened(modal.type_select_opened)
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

    let submit = StyledButton::build()
        .primary()
        .text("Create Issue")
        .add_class("action")
        .add_class("submit")
        .add_class("ActionButton")
        .build()
        .into_node();
    let cancel = StyledButton::build()
        .empty()
        .add_class("action")
        .add_class("cancel")
        .add_class("actionButton")
        .text("Cancel")
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
            attrs![At::Class => "type"],
            icon,
            div![attrs![At::Class => "typeLabel"], name]
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
pub struct UserOption<'opt>(pub &'opt User);

impl<'opt> crate::shared::styled_select::SelectOption for UserOption<'opt> {
    fn into_option(self) -> Node<Msg> {
        let user = self.0;

        div![
            attrs![At::Class => "type"],
            div![attrs![At::Class => "typeLabel"], user.name]
        ]
    }

    fn into_value(self) -> Node<Msg> {
        let user = self.0;

        div![
            attrs![At::Class => "selectItem"],
            div![attrs![At::Class => "selectItemLabel"], user.name]
        ]
    }

    fn match_text_filter(&self, _text_filter: &str) -> bool {
        false
    }

    fn to_value(&self) -> u32 {
        self.0.id as u32
    }
}
