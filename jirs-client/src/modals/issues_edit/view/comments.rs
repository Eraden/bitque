use {
    crate::{
        components::{
            styled_avatar::StyledAvatar, styled_button::StyledButton,
            styled_textarea::StyledTextarea,
        },
        modals::issues_edit::Model as EditIssueModal,
        model::{CommentForm, ModalType, Model},
        shared::ToNode,
        EditIssueModalSection, FieldChange, FieldId, Msg,
    },
    jirs_data::{Comment, CommentFieldId},
    seed::{prelude::*, *},
};

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

    let text_area = StyledTextarea::build(FieldId::EditIssueModal(EditIssueModalSection::Comment(
        CommentFieldId::Body,
    )))
    .value(form.body.as_str())
    .placeholder("Add a comment...")
    .build()
    .into_node();

    let submit = StyledButton::build()
        .primary()
        .on_click(submit_comment_form)
        .text("Save")
        .build()
        .into_node();
    let cancel = StyledButton::build()
        .empty()
        .on_click(close_comment_form)
        .text("Cancel")
        .build()
        .into_node();

    vec![text_area, div![C!["actions"], submit, cancel]]
}

pub fn comment(model: &Model, modal: &EditIssueModal, comment: &Comment) -> Option<Node<Msg>> {
    let show_form = modal.comment_form.creating && modal.comment_form.id == Some(comment.id);

    let user = model.users_by_id.get(&comment.user_id)?;

    let avatar = StyledAvatar::build()
        .size(32)
        .avatar_url(user.avatar_url.as_deref()?)
        .add_class("userAvatar")
        .build()
        .into_node();

    let buttons = if model.user.as_ref().map(|u| u.id) == Some(comment.user_id) {
        let comment_id = comment.id;
        let delete_comment_handler = mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            Msg::ModalOpened(Box::new(ModalType::DeleteCommentConfirm(comment_id)))
        });
        let edit_button = StyledButton::build()
            .add_class("editButton")
            .on_click(mouse_ev(Ev::Click, move |_| {
                Msg::ModalChanged(FieldChange::EditComment(
                    FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
                    comment_id,
                ))
            }))
            .text("Edit")
            .empty()
            .build()
            .into_node();

        let cancel_button = StyledButton::build()
            .add_class("deleteButton")
            .on_click(delete_comment_handler)
            .text("Delete")
            .empty()
            .build()
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
