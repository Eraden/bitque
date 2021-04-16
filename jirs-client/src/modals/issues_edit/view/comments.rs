use jirs_data::{Comment, CommentFieldId};
use seed::prelude::*;
use seed::*;

use crate::components::styled_avatar::StyledAvatar;
use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_textarea::StyledTextarea;
use crate::modals::issues_edit::Model as EditIssueModal;
use crate::model::{CommentForm, ModalType, Model};
use crate::shared::ToNode;
use crate::{EditIssueModalSection, FieldChange, FieldId, Msg};

pub fn build_comment_form(form: &CommentForm) -> Vec<Node<Msg>> {
    let submit_comment_form = mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::SaveComment
    });
    let close_comment_form = mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ModalChanged(FieldChange::ToggleCommentForm(
            FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
            false,
        ))
    });

    let text_area = StyledTextarea {
        id: Some(FieldId::EditIssueModal(EditIssueModalSection::Comment(
            CommentFieldId::Body,
        ))),
        value: form.body.as_str(),
        placeholder: "Add a comment...",
        ..Default::default()
    }
    .into_node();

    let submit = StyledButton {
        variant: ButtonVariant::Primary,
        on_click: Some(submit_comment_form),
        text: Some("Save"),
        ..Default::default()
    }
    .into_node();
    let cancel = StyledButton {
        variant: ButtonVariant::Empty,
        on_click: Some(close_comment_form),
        text: Some("Cancel"),
        ..Default::default()
    }
    .into_node();

    vec![text_area, div![C!["actions"], submit, cancel]]
}

pub fn comment(model: &Model, modal: &EditIssueModal, comment: &Comment) -> Option<Node<Msg>> {
    let show_form = modal.comment_form.creating && modal.comment_form.id == Some(comment.id);

    let user = model.users_by_id.get(&comment.user_id)?;

    let avatar = StyledAvatar {
        avatar_url: user.avatar_url.as_deref(),
        size: 32,
        class_list: "userAvatar",
        ..StyledAvatar::default()
    }
    .into_node();

    let buttons = if model.user.as_ref().map(|u| u.id) == Some(comment.user_id) {
        let comment_id = comment.id;
        let delete_comment_handler = mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            Msg::ModalOpened(ModalType::DeleteCommentConfirm(Some(comment_id)))
        });
        let edit_button = StyledButton {
            class_list: "editButton",
            on_click: Some(mouse_ev(Ev::Click, move |_| {
                Msg::ModalChanged(FieldChange::EditComment(
                    FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
                    comment_id,
                ))
            })),
            text: Some("Edit"),
            variant: ButtonVariant::Empty,
            ..Default::default()
        }
        .into_node();

        let cancel_button = StyledButton {
            class_list: "deleteButton",
            on_click: Some(delete_comment_handler),
            text: Some("Delete"),
            variant: ButtonVariant::Empty,
            ..Default::default()
        }
        .into_node();

        vec![edit_button, cancel_button]
    } else {
        vec![]
    };

    let content = if show_form {
        div![C!["content"], build_comment_form(&modal.comment_form)]
    } else {
        div![
            C!["content"],
            div![C!["userName"], user.name.as_str()],
            p![C!["body"], comment.body.as_str()],
            buttons,
        ]
    };

    let node = div![C!["styledComment"], avatar, content];
    Some(node)
}
