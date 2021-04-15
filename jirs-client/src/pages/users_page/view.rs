use {
    crate::{
        components::{
            styled_button::StyledButton, styled_field::StyledField, styled_form::StyledForm,
            styled_input::StyledInput, styled_select::StyledSelect,
        },
        model::{InvitationFormState, Model, PageContent},
        shared::{inner_layout, IntoChild, ToNode},
        validations::is_email,
        FieldId, Msg, PageChanged, UsersPageChange,
    },
    jirs_data::{InvitationState, UserRole, UsersFieldId},
    seed::{prelude::*, *},
};

use crate::components::styled_button::ButtonVariant;
use crate::components::styled_input::InputVariant;
use crate::components::styled_select::SelectVariant;

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Users(page) => page,
        _ => return empty![],
    };

    let name = StyledInput {
        valid: !page.name_touched || page.name.len() >= 3,
        value: page.name.as_str(),
        variant: InputVariant::Normal,
        id: Some(FieldId::Users(UsersFieldId::Username)),
        ..Default::default()
    }
    .into_node();
    let name_field = StyledField {
        label: "Name",
        input: name,
        ..Default::default()
    }
    .into_node();

    let email = StyledInput {
        id: Some(FieldId::Users(UsersFieldId::Email)),
        valid: !page.email_touched || is_email(page.email.as_str()),
        value: page.email.as_str(),
        variant: InputVariant::Normal,
        ..Default::default()
    }
    .into_node();
    let email_field = StyledField {
        input: email,
        label: "E-Mail",
        ..Default::default()
    }
    .into_node();

    let user_role = StyledSelect {
        id: FieldId::Users(UsersFieldId::UserRole),
        name: "user_role",
        valid: true,
        variant: SelectVariant::Normal,
        text_filter: page.user_role_state.text_filter.as_str(),
        opened: page.user_role_state.opened,
        selected: vec![page.user_role.into_child()],
        options: Some(
            UserRole::default()
                .into_iter()
                .map(|role| role.into_child()),
        ),
        ..Default::default()
    }
    .into_node();
    let user_role_field = StyledField::build()
        .input(user_role)
        .label("Role")
        .build()
        .into_node();

    let submit = StyledButton {
        text: Some("Invite user"),
        variant: ButtonVariant::Primary,
        class_list: "submitUserInvite",
        active: page.form_state != InvitationFormState::Sent,
        ..Default::default()
    }
    .into_node();
    let submit_supplement = match page.form_state {
        InvitationFormState::Succeed => StyledButton {
            variant: ButtonVariant::Empty,
            class_list: "resetUserInvite",
            active: true,
            on_click: Some(mouse_ev(Ev::Click, |_| {
                Msg::PageChanged(PageChanged::Users(UsersPageChange::ResetForm))
            })),
            text: Some("Reset"),
            button_type: "reset",
            ..Default::default()
        }
        .into_node(),
        InvitationFormState::Failed => div![C!["error"], "There was an error"],
        _ => empty![],
    };
    let submit_field = StyledField::build()
        .input(div![C!["invitationActions"], submit, submit_supplement])
        .build()
        .into_node();

    let form = StyledForm::build()
        .heading("Invite new user")
        .add_field(name_field)
        .add_field(email_field)
        .add_field(user_role_field)
        .add_field(submit_field)
        .on_submit(ev(Ev::Submit, |ev| {
            ev.prevent_default();
            Msg::InviteRequest
        }))
        .build()
        .into_node();

    let users: Vec<Node<Msg>> = page
        .invited_users
        .iter()
        .map(|user| {
            let user_id = user.id;
            let remove = StyledButton {
                text: Some("Remove"),
                on_click: Some(mouse_ev(Ev::Click, move |_| {
                    Msg::InvitedUserRemove(user_id)
                })),
                ..Default::default()
            }
            .into_node();
            let role = page
                .invitations
                .iter()
                .find(|iv| iv.email.eq(user.email.as_str()) && iv.name.eq(user.name.as_str()))
                .map(|iv| iv.role)
                .unwrap_or_default();

            li![
                C!["user"],
                span![user.name.as_str()],
                span![user.email.as_str()],
                span![format!("{}", role)],
                remove,
            ]
        })
        .collect();

    let users_section = section![
        C!["usersSection"],
        h1![C!["heading"], "Users"],
        ul![C!["usersList"], users],
    ];

    let invitations: Vec<Node<Msg>> = page
        .invitations
        .iter()
        .map(|invitation| {
            let id = invitation.id;
            let revoke = StyledButton {
                disabled: invitation.state == InvitationState::Revoked,
                text: Some("Revoke"),
                on_click: Some(mouse_ev(Ev::Click, move |_| Msg::InviteRevokeRequest(id))),
                ..Default::default()
            }
            .into_node();
            // let revoke = StyledButton::build()
            //     .text("Revoke")
            //     .disabled(invitation.state == InvitationState::Revoked)
            //     .on_click(mouse_ev(Ev::Click, move |_| Msg::InviteRevokeRequest(id)))
            //     .build()
            //     .into_node();
            li![
                C!["invitation", format!("{}", invitation.state)],
                span![invitation.name.as_str()],
                span![invitation.email.as_str()],
                span![format!("{}", invitation.state)],
                revoke,
            ]
        })
        .collect();

    let invitations_section = section![
        C!["invitationsSection"],
        h1![C!["heading"], "Invitations"],
        ul![C!["invitationsList"], invitations],
    ];

    inner_layout(model, "users", &[form, users_section, invitations_section])
}
