use seed::{prelude::*, *};

use jirs_data::{ToVec, UserRole, UsersFieldId, WsMsg};

use crate::api::send_ws_msg;
use crate::model::{InvitationFormState, Model, Page, PageContent, UsersPage};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::*;
use crate::shared::styled_select_child::ToStyledSelectChild;
use crate::shared::{inner_layout, ToNode};
use crate::validations::is_email;
use crate::{FieldId, Msg};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if let Msg::ChangePage(Page::Users) = msg {
        model.page_content = PageContent::Users(Box::new(UsersPage::default()));
        // return;
    }

    let page = match &mut model.page_content {
        PageContent::Users(page) => page,
        _ => return,
    };

    page.user_role_state.update(&msg, orders);

    match msg {
        Msg::WsMsg(WsMsg::AuthorizeLoaded(Ok(_))) | Msg::ChangePage(Page::Users)
            if model.user.is_some() =>
        {
            send_ws_msg(WsMsg::InvitationListRequest);
            send_ws_msg(WsMsg::InvitedUsersRequest);
        }
        Msg::WsMsg(WsMsg::InvitedUsersLoaded(users)) => {
            page.invited_users = users;
        }
        Msg::WsMsg(WsMsg::InvitationListLoaded(invitations)) => {
            page.invitations = invitations;
        }
        Msg::StyledSelectChanged(
            FieldId::Users(UsersFieldId::UserRole),
            StyledSelectChange::Changed(role),
        ) => {
            page.user_role = role.into();
        }
        Msg::InputChanged(FieldId::Users(UsersFieldId::Username), name) => {
            page.name = name;
            page.name_touched = true;
        }
        Msg::InputChanged(FieldId::Users(UsersFieldId::Email), email) => {
            page.email = email;
            page.email_touched = true;
        }
        Msg::InviteRequest => {
            page.form_state = InvitationFormState::Sent;
            send_ws_msg(WsMsg::InvitationSendRequest {
                name: page.name.clone(),
                email: page.email.clone(),
            })
        }
        Msg::WsMsg(WsMsg::InvitationRevokeSuccess(id)) => {
            let mut old = vec![];
            std::mem::swap(&mut page.invitations, &mut old);
            for invitation in old {
                if invitation.id != id {
                    page.invitations.push(invitation);
                }
            }
        }
        Msg::InviteRevokeRequest(invitation_id) => {
            send_ws_msg(WsMsg::InvitationRevokeRequest(invitation_id));
        }
        Msg::InvitedUserRemove(email) => {
            send_ws_msg(WsMsg::InvitedUserRemoveRequest(email));
        }
        Msg::WsMsg(WsMsg::InvitedUserRemoveSuccess(email)) => {
            let mut old = vec![];
            std::mem::swap(&mut page.invited_users, &mut old);
            for user in old {
                if user.email != email {
                    page.invited_users.push(user);
                }
            }
        }
        Msg::WsMsg(WsMsg::InvitationSendSuccess) => {
            send_ws_msg(WsMsg::InvitationListRequest);
            page.form_state = InvitationFormState::Succeed;
        }
        Msg::WsMsg(WsMsg::InvitationSendFailure) => {
            page.form_state = InvitationFormState::Failed;
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    if model.user.is_none() {
        return empty![];
    }

    let page = match &model.page_content {
        PageContent::Users(page) => page,
        _ => return empty![],
    };

    let name = StyledInput::build(FieldId::Users(UsersFieldId::Username))
        .valid(!page.name_touched || page.name.len() >= 3)
        .value(page.name.as_str())
        .build()
        .into_node();
    let name_field = StyledField::build()
        .input(name)
        .label("Name")
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::Users(UsersFieldId::Email))
        .valid(!page.email_touched || is_email(page.email.as_str()))
        .value(page.email.as_str())
        .build()
        .into_node();
    let email_field = StyledField::build()
        .input(email)
        .label("E-Mail")
        .build()
        .into_node();

    let user_role = StyledSelect::build(FieldId::Users(UsersFieldId::UserRole))
        .name("user_role")
        .valid(true)
        .normal()
        .with_state(&page.user_role_state)
        .selected(vec![page.user_role.to_select_child()])
        .options(
            UserRole::ordered()
                .into_iter()
                .map(|role| role.to_select_child())
                .collect(),
        )
        .build()
        .into_node();
    let user_role_field = StyledField::build()
        .input(user_role)
        .label("Role")
        .build()
        .into_node();

    let submit = StyledButton::build()
        .add_class("submitUserInvite")
        .active(page.form_state != InvitationFormState::Sent)
        .primary()
        .text("Invite user")
        .build()
        .into_node();
    let submit_supplement = match page.form_state {
        InvitationFormState::Succeed => StyledButton::build()
            .add_class("resetUserInvite")
            .active(true)
            .empty()
            .set_type_reset()
            .text("Reset")
            .build()
            .into_node(),
        InvitationFormState::Failed => div![class!["error"], "There was an error"],
        _ => empty![],
    };
    let submit_field = StyledField::build()
        .input(div![class!["invitationActions"], submit, submit_supplement])
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
            let email = user.email.clone();
            let remove = StyledButton::build()
                .text("Remove")
                .on_click(mouse_ev(Ev::Click, move |_| Msg::InvitedUserRemove(email)))
                .build()
                .into_node();
            li![
                class!["user"],
                span![user.name],
                span![user.email],
                span![format!("{}", user.user_role)],
                remove,
            ]
        })
        .collect();

    let users_section = section![
        class!["usersSection"],
        h1![class!["heading"], "Users"],
        ul![class!["usersList"], users],
    ];

    let invitations: Vec<Node<Msg>> = page
        .invitations
        .iter()
        .map(|invitation| {
            let id = invitation.id;
            let revoke = StyledButton::build()
                .text("Revoke")
                .on_click(mouse_ev(Ev::Click, move |_| Msg::InviteRevokeRequest(id)))
                .build()
                .into_node();
            li![
                class!["invitation"],
                span![invitation.name],
                span![invitation.email],
                span![format!("{}", invitation.state)],
                revoke,
            ]
        })
        .collect();

    let invitations_section = section![
        class!["invitationsSection"],
        h1![class!["heading"], "Invitations"],
        ul![class!["invitationsList"], invitations],
    ];

    inner_layout(
        model,
        "users",
        vec![form, users_section, invitations_section],
        empty![],
    )
}
