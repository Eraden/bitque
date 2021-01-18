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
    let close = StyledButton::build()
        .on_click(mouse_ev("click", |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::ModalDropped
        }))
        .empty()
        .icon(Icon::Close)
        .build()
        .into_node();
    StyledModal::build()
        .center()
        .width(600)
        .add_class("editEpic")
        .child(div![C!["header"], h1!["Edit epic"], close])
        .child(
            StyledInput::build()
                .state(&modal.name)
                .build(FieldId::EditEpic(EpicFieldId::Name))
                .into_node(),
        )
        .child(transform)
        .build()
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
    let execute = StyledButton::build().text("Transform").build().into_node();
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
