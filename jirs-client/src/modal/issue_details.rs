use seed::{prelude::*, *};

use jirs_data::{IssuePriority, IssueStatus, IssueType, ToVec, User};

use crate::model::{EditIssueModal, ModalType, Model};
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_editor::StyledEditor;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_select::{SelectOption, StyledSelect, StyledSelectChange};
use crate::shared::styled_textarea::StyledTextarea;
use crate::shared::ToNode;
use crate::{FieldChange, FieldId, IssueId, Msg};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let modal: &mut EditIssueModal = match model.modals.get_mut(0) {
        Some(ModalType::EditIssue(_issue_id, modal)) => modal,
        _ => return,
    };
    modal.top_type_state.update(msg, orders);
    modal.status_state.update(msg, orders);
    modal.reporter_state.update(msg, orders);
    modal.assignees_state.update(msg, orders);
    modal.priority_state.update(msg, orders);

    match msg {
        Msg::StyledSelectChanged(
            FieldId::IssueTypeEditModalTop,
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.issue_type = (*value).into();
        }
        Msg::StyledSelectChanged(
            FieldId::StatusIssueEditModal,
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.status = (*value).into();
        }
        Msg::StyledSelectChanged(
            FieldId::ReporterIssueEditModal,
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.reporter_id = *value as i32;
        }
        Msg::StyledSelectChanged(
            FieldId::AssigneesIssueEditModal,
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.user_ids.push(*value as i32);
        }
        Msg::StyledSelectChanged(
            FieldId::PriorityIssueEditModal,
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.priority = (*value).into();
        }
        Msg::InputChanged(FieldId::TitleIssueEditModal, value) => {
            modal.payload.title = value.clone();
        }
        Msg::InputChanged(FieldId::DescriptionIssueEditModal, value) => {
            modal.payload.description = Some(value.clone());
            modal.payload.description_text = Some(value.clone());
        }
        Msg::ModalChanged(FieldChange::TabChanged(FieldId::DescriptionIssueEditModal, mode)) => {
            modal.description_editor_mode = mode.clone();
        }

        _ => (),
    }
}

pub fn view(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        id,
        link_copied,
        payload,
        top_type_state,
        status_state,
        reporter_state: _,
        assignees_state: _,
        priority_state: _,
        description_editor_mode,
    } = modal;
    let issue_id = id.clone();

    let issue_type_select = StyledSelect::build(FieldId::IssueTypeEditModalTop)
        .dropdown_width(150)
        .name("type")
        .text_filter(top_type_state.text_filter.as_str())
        .opened(top_type_state.opened)
        .valid(true)
        .options(
            IssueType::ordered()
                .into_iter()
                .map(|t| IssueTypeTopOption(issue_id, t))
                .collect(),
        )
        .selected(vec![IssueTypeTopOption(
            issue_id,
            payload.issue_type.clone(),
        )])
        .build()
        .into_node();

    let click_handler = mouse_ev(Ev::Click, move |_| {
        use wasm_bindgen::JsCast;

        let link = format!("http://localhost:7000/issues/{id}", id = issue_id);
        let el = match seed::html_document().create_element("textarea") {
            Ok(el) => el
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .clone(),
            _ => return Msg::NoOp,
        };
        seed::body().append_child(&el).unwrap();
        el.set_text_content(Some(link.as_str()));
        el.select();
        el.set_selection_range(0, 9999).unwrap();
        seed::html_document().exec_command("copy").unwrap();
        seed::body().remove_child(&el).unwrap();
        Msg::ModalChanged(FieldChange::LinkCopied(FieldId::CopyButtonLabel, true))
    });
    let close_handler = mouse_ev(Ev::Click, |_| Msg::ModalDropped);
    let delete_confirmation_handler = mouse_ev(Ev::Click, move |_| {
        Msg::ModalOpened(ModalType::DeleteIssueConfirm(issue_id))
    });

    let copy_button = StyledButton::build()
        .empty()
        .icon(Icon::Link)
        .on_click(click_handler)
        .children(vec![span![if *link_copied {
            "Link Copied"
        } else {
            "Copy link"
        }]])
        .build()
        .into_node();
    let delete_button = StyledButton::build()
        .empty()
        .icon(Icon::Trash.into_styled_builder().size(19).build())
        .on_click(delete_confirmation_handler)
        .build()
        .into_node();
    let close_button = StyledButton::build()
        .empty()
        .icon(Icon::Close.into_styled_builder().size(24).build())
        .on_click(close_handler)
        .build()
        .into_node();

    // left
    let title = StyledTextarea::build()
        .value(payload.title.as_str())
        .add_class("textarea")
        .max_height(48)
        .height(0)
        .build(FieldId::TitleIssueEditModal)
        .into_node();

    let description_text = payload.description.as_ref().cloned().unwrap_or_default();
    let description = StyledEditor::build(FieldId::DescriptionIssueEditModal)
        .text(description_text)
        .mode(description_editor_mode.clone())
        .build()
        .into_node();
    let description_field = StyledField::build().input(description).build().into_node();

    // right
    let status = StyledSelect::build(FieldId::StatusIssueEditModal)
        .name("status")
        .opened(status_state.opened)
        .normal()
        .text_filter(status_state.text_filter.as_str())
        .options(
            IssueStatus::ordered()
                .into_iter()
                .map(|opt| IssueStatusOption(issue_id, opt))
                .collect(),
        )
        .selected(vec![IssueStatusOption(issue_id, payload.status.clone())])
        .valid(true)
        .build()
        .into_node();
    let status_field = StyledField::build()
        .input(status)
        .label("Status")
        .build()
        .into_node();

    let assignees = StyledSelect::build(FieldId::AssigneesIssueEditModal)
        .name("assignees")
        .opened(modal.assignees_state.opened)
        .normal()
        .multi()
        .text_filter(modal.assignees_state.text_filter.as_str())
        .options(model.users.iter().map(|user| UserOption(user)).collect())
        .selected(
            model
                .users
                .iter()
                .filter(|user| payload.user_ids.contains(&user.id))
                .map(|user| UserOption(user))
                .collect(),
        )
        .build()
        .into_node();
    let assignees_field = StyledField::build()
        .input(assignees)
        .label("Assignees")
        .build()
        .into_node();

    div![
        attrs![At::Class => "issueDetails"],
        div![
            attrs![At::Class => "topActions"],
            issue_type_select,
            div![
                attrs![At::Class => "topActionsRight"],
                copy_button,
                delete_button,
                close_button
            ],
        ],
        div![
            attrs![At::Class => "content"],
            div![
                attrs![At::Class => "left"],
                title,
                description_field,
                div![attrs![At::Class => "comments"]],
            ],
            div![attrs![At::Class => "right"], status_field, assignees_field,],
        ],
    ]
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct IssueTypeTopOption(pub IssueId, pub IssueType);

impl SelectOption for IssueTypeTopOption {
    fn into_option(self) -> Node<Msg> {
        let name = self.1.to_label().to_owned();

        let icon = StyledIcon::build(self.1.into())
            .add_class("issueTypeIcon".to_string())
            .build()
            .into_node();

        div![
            attrs![At::Class => "optionItem"],
            icon,
            div![attrs![At::Class => "optionLabel typeLabel"], name]
        ]
    }

    fn into_value(self) -> Node<Msg> {
        let issue_id = self.0;
        let name = self.1.to_label().to_owned();

        StyledButton::build()
            .empty()
            .children(vec![span![format!("{}-{}", name, issue_id)]])
            .icon(StyledIcon::build(self.1.into()).build())
            .build()
            .into_node()
    }

    fn match_text_filter(&self, text_filter: &str) -> bool {
        self.1
            .to_string()
            .to_lowercase()
            .contains(&text_filter.to_lowercase())
    }

    fn to_value(&self) -> u32 {
        self.1.clone().into()
    }
}

/////
#[derive(PartialOrd, PartialEq, Debug)]
pub struct IssueStatusOption(pub IssueId, pub IssueStatus);

impl SelectOption for IssueStatusOption {
    fn into_option(self) -> Node<Msg> {
        let name = self.1.to_label().to_owned();

        div![
            attrs![At::Class => "type optionItem"],
            div![attrs![At::Class => "typeLabel optionLabel"], name]
        ]
    }

    fn into_value(self) -> Node<Msg> {
        let name = self.1.to_label().to_owned();

        div![
            attrs![At::Class => "selectItem"],
            div![attrs![At::Class => "selectItemLabel"], name]
        ]
    }

    fn match_text_filter(&self, text_filter: &str) -> bool {
        format!("{}", self.0)
            .to_lowercase()
            .contains(&text_filter.to_lowercase())
    }

    fn to_value(&self) -> u32 {
        self.1.clone().into()
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct IssueTypeOption(pub IssueType);

impl SelectOption for IssueTypeOption {
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

impl SelectOption for IssuePriorityOption {
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

impl<'opt> SelectOption for UserOption<'opt> {
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
