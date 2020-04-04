use crate::model::{AddIssueModal, Model};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_modal::{StyledModal, Variant as ModalVariant};
use crate::shared::styled_select::StyledSelect;
use crate::shared::ToNode;
use crate::{FieldId, Msg};
use jirs_data::IssueType;
use seed::{prelude::*, *};

pub fn view(_model: &Model, modal: &AddIssueModal) -> Node<Msg> {
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
    let form = StyledForm::build()
        .heading("Create issue")
        .add_field(select_type)
        .build()
        .into_node();

    StyledModal::build()
        .width(800)
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
            .add_class("issueTypeIcon".to_string())
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
        let chevron_icon = StyledIcon::build(Icon::ChevronDown).build().into_node();
        div![attrs![At::Class => "option"], type_icon, name, chevron_icon]
        // StyledButton::build()
        //     .secondary()
        //     .children(vec![span![format!("{}", name)]])
        //     .icon(StyledIcon::build(self.0.into()).build())
        //     .build()
        //     .into_node()
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
