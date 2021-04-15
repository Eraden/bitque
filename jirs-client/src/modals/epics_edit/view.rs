use {
    crate::{
        components::{
            styled_button::*, styled_checkbox::*, styled_icon::Icon, styled_input::*,
            styled_modal::*,
        },
        modals::epics_edit::Model,
        model,
        shared::{IntoChild, ToNode},
        FieldId, Msg,
    },
    jirs_data::{EpicFieldId, IssueType},
    seed::{prelude::*, *},
};

pub struct IssueTypeWrapper(IssueType);

impl<'l> IntoChild<'l> for IssueTypeWrapper {
    type Builder = ChildBuilder<'l>;

    fn into_child(self) -> Self::Builder {
        Self::Builder::default()
            .label(self.0.to_label())
            .name(self.0.to_str())
            .value(self.0.into())
            .add_class(self.0.to_str())
    }
}

pub fn view(_model: &model::Model, modal: &Model) -> Node<Msg> {
    let transform = if modal.related_issues.is_empty() {
        transform_into_available(modal)
    } else {
        transform_into_unavailable(modal)
    };
    let close = StyledButton {
        on_click: Some(mouse_ev("click", |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::ModalDropped
        })),
        variant: ButtonVariant::Empty,
        icon: Some(Icon::Close.into_node()),
        ..Default::default()
    }
    .into_node();
    StyledModal {
        width: Some(600),
        class_list: "editEpic",
        children: vec![
            div![C!["header"], h1!["Edit epic"], close],
            StyledInput {
                value: modal.name.value.as_str(),
                valid: modal.name.is_valid(),
                id: Some(FieldId::EditEpic(EpicFieldId::Name)),
                ..Default::default()
            }
            .into_node(),
            transform,
        ],
        ..Default::default()
    }
    .into_node()
}

fn transform_into_available(modal: &super::Model) -> Node<Msg> {
    let types = StyledCheckbox::build()
        .options(
            IssueType::default()
                .into_iter()
                .map(|ty| IssueTypeWrapper(ty).into_child()),
        )
        .state(&modal.transform_into)
        .build(FieldId::EditEpic(EpicFieldId::TransformInto))
        .into_node();
    let execute = StyledButton {
        on_click: Some(mouse_ev("click", |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::TransformEpic
        })),
        text: Some("Transform"),
        ..Default::default()
    }
    .into_node();
    div![C!["transform available"], div![types], div![execute]]
}

fn transform_into_unavailable(modal: &super::Model) -> Node<Msg> {
    let (n, s) = match modal.related_issues.len() {
        1 => (1.to_string(), "issue"),
        n => (n.to_string(), "issues"),
    };
    div![
        C!["transform unavailable"],
        span![
            C!["info"],
            "This epic have related issues so you can't change it type."
        ],
        span![C!["count"], format!("Epic have {} {}", n, s)]
    ]
}
