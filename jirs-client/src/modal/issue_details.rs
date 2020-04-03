use seed::{prelude::*, *};

use jirs_data::{Issue, IssueType};

use crate::model::{EditIssueModal, Model};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_select::{StyledSelect, Variant as SelectVariant};
use crate::shared::ToNode;
use crate::{FieldId, IssueId, Msg};

#[derive(PartialOrd, PartialEq, Debug)]
struct IssueTypeOption(IssueId, IssueType);

impl crate::shared::styled_select::SelectOption for IssueTypeOption {
    fn into_option(self) -> Node<Msg> {
        let name = self.1.to_label().to_owned();

        let icon = StyledIcon::build(self.1.into())
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

pub fn view(_model: &Model, issue: &Issue, modal: &EditIssueModal) -> Node<Msg> {
    let issue_id = issue.id;

    let issue_type_select = StyledSelect {
        id: FieldId::IssueTypeEditModalTop,
        variant: SelectVariant::Empty,
        dropdown_width: Some(150),
        name: Some("type".to_string()),
        placeholder: None,
        text_filter: modal.top_select_filter.clone(),
        opened: modal.top_select_opened,
        valid: true,
        is_multi: false,
        allow_clear: false,
        options: vec![
            IssueTypeOption(issue_id, IssueType::Story),
            IssueTypeOption(issue_id, IssueType::Task),
            IssueTypeOption(issue_id, IssueType::Bug),
        ],
        selected: vec![IssueTypeOption(issue_id, modal.value.clone())],
    }
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
        Msg::NoOp
    });

    let copy_button = StyledButton::build()
        .empty()
        .icon(Icon::Link)
        .on_click(click_handler)
        .children(vec![span![if modal.link_copied {
            "Link Copied"
        } else {
            "Copy link"
        }]])
        .build()
        .into_node();

    let delete_button = StyledButton::build()
        .empty()
        .icon(Icon::Trash.into_styled_builder().size(19).build())
        .build()
        .into_node();
    let close_button = StyledButton::build()
        .empty()
        .icon(Icon::Close.into_styled_builder().size(24).build())
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
                div![attrs![At::Class => "title"]],
                div![attrs![At::Class => "description"]],
                div![attrs![At::Class => "comments"]],
            ],
            div![attrs![At::Class => "right"]],
        ],
    ]
}
