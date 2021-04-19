use jirs_data::{InvitationState, UserRole, UsersFieldId};
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_input::{InputVariant, StyledInput};
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::{InvitationFormState, Model, PageContent};
use crate::pages::users_page::UsersPage;
use crate::shared::inner_layout;
use crate::validations::is_email;
use crate::{FieldId, Msg, PageChanged, UsersPageChange};

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
    .render();
    let name_field = StyledField {
        label: "Name",
        input: name,
        ..Default::default()
    }
    .render();

    let email = StyledInput {
        id: Some(FieldId::Users(UsersFieldId::Email)),
        valid: !page.email_touched || is_email(page.email.as_str()),
        value: page.email.as_str(),
        variant: InputVariant::Normal,
        ..Default::default()
    }
    .render();
    let email_field = StyledField {
        input: email,
        label: "E-Mail",
        ..Default::default()
    }
    .render();

    let user_role_field = user_role_select(page);

    let submit = StyledButton {
        text: Some("Invite user"),
        variant: ButtonVariant::Primary,
        class_list: "submitUserInvite",
        active: page.form_state != InvitationFormState::Sent,
        ..Default::default()
    }
    .render();
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
        .render(),
        InvitationFormState::Failed => div![C!["error"], "There was an error"],
        _ => empty![],
    };
    let submit_field = StyledField {
        input: div![C!["invitationActions"], submit, submit_supplement],
        ..Default::default()
    }
    .render();

    let form = StyledForm {
        heading: "Invite new user",
        on_submit: Some(ev(Ev::Submit, |ev| {
            ev.prevent_default();
            Msg::InviteRequest
        })),
        fields: vec![name_field, email_field, user_role_field, submit_field],
    }
    .render();

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
            .render();
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
            .render();
            // let revoke = StyledButton::build()
            //     .text("Revoke")
            //     .disabled(invitation.state == InvitationState::Revoked)
            //     .on_click(mouse_ev(Ev::Click, move |_| Msg::InviteRevokeRequest(id)))
            //     .build()
            //     .render();
            li![
                C!["invitation", invitation.state.to_str()],
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

#[inline(always)]
fn user_role_select(page: &UsersPage) -> Node<Msg> {
    let user_role = StyledSelect {
        id: FieldId::Users(UsersFieldId::UserRole),
        name: "user_role",
        valid: true,
        variant: SelectVariant::Normal,
        text_filter: page.user_role_state.text_filter.as_str(),
        opened: page.user_role_state.opened,
        selected: vec![user_role_select_option(page.user_role)],
        options: Some(UserRole::default().into_iter().map(user_role_select_option)),
        ..Default::default()
    }
    .render();
    StyledField {
        input: user_role,
        label: "Role",
        ..Default::default()
    }
    .render()
}

fn user_role_select_option<'l>(ur: UserRole) -> StyledSelectOption<'l> {
    let name = ur.to_str();

    StyledSelectOption {
        text: Some(name),
        value: ur.into(),
        class_list: name,
        ..Default::default()
    }
}
