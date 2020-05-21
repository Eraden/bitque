use seed::{prelude::*, *};

use jirs_data::{InvitationState, ToVec, UserRole, UsersFieldId, WsMsg};

use crate::model::*;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::*;
use crate::shared::{inner_layout, ToChild, ToNode};
use crate::validations::is_email;
use crate::ws::{enqueue_ws_msg, send_ws_msg};
use crate::{FieldId, Msg, PageChanged, UsersPageChange, WebSocketChanged};

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
        Msg::ChangePage(Page::Users) if model.user.is_some() => {
            enqueue_ws_msg(
                vec![WsMsg::InvitationListRequest, WsMsg::InvitedUsersRequest],
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(Ok(_))) if model.user.is_some() => {
                enqueue_ws_msg(
                    vec![WsMsg::InvitationListRequest, WsMsg::InvitedUsersRequest],
                    model.ws.as_ref(),
                    orders,
                );
            }
            WebSocketChanged::WsMsg(WsMsg::InvitedUsersLoaded(users)) => {
                page.invited_users = users;
            }
            WebSocketChanged::WsMsg(WsMsg::InvitationListLoaded(invitations)) => {
                page.invitations = invitations;
            }
            WebSocketChanged::WsMsg(WsMsg::InvitationRevokeSuccess(id)) => {
                let mut old = vec![];
                std::mem::swap(&mut page.invitations, &mut old);
                for mut invitation in old {
                    if id == invitation.id {
                        invitation.state = InvitationState::Revoked;
                    }
                    page.invitations.push(invitation);
                }
                send_ws_msg(WsMsg::InvitationListRequest, model.ws.as_ref(), orders);
            }
            WebSocketChanged::WsMsg(WsMsg::InvitedUserRemoveSuccess(email)) => {
                let mut old = vec![];
                std::mem::swap(&mut page.invited_users, &mut old);
                for user in old {
                    if user.email != email {
                        page.invited_users.push(user);
                    }
                }
            }
            WebSocketChanged::WsMsg(WsMsg::InvitationSendSuccess) => {
                send_ws_msg(WsMsg::InvitationListRequest, model.ws.as_ref(), orders);
                page.form_state = InvitationFormState::Succeed;
            }
            WebSocketChanged::WsMsg(WsMsg::InvitationSendFailure) => {
                page.form_state = InvitationFormState::Failed;
            }
            _ => (),
        },
        Msg::PageChanged(PageChanged::Users(UsersPageChange::ResetForm)) => {
            page.name.clear();
            page.name_touched = false;
            page.email.clear();
            page.email_touched = false;
            page.user_role = UserRole::User;
            page.user_role_state.reset();
            page.form_state = InvitationFormState::Initial;
        }
        Msg::StyledSelectChanged(
            FieldId::Users(UsersFieldId::UserRole),
            StyledSelectChange::Changed(role),
        ) => {
            page.user_role = role.into();
        }
        Msg::StrInputChanged(FieldId::Users(UsersFieldId::Username), name) => {
            page.name = name;
            page.name_touched = true;
        }
        Msg::StrInputChanged(FieldId::Users(UsersFieldId::Email), email) => {
            page.email = email;
            page.email_touched = true;
        }
        Msg::InviteRequest => {
            page.form_state = InvitationFormState::Sent;
            send_ws_msg(
                WsMsg::InvitationSendRequest {
                    name: page.name.clone(),
                    email: page.email.clone(),
                },
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::InviteRevokeRequest(invitation_id) => {
            send_ws_msg(
                WsMsg::InvitationRevokeRequest(invitation_id),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::InvitedUserRemove(email) => {
            send_ws_msg(
                WsMsg::InvitedUserRemoveRequest(email),
                model.ws.as_ref(),
                orders,
            );
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
        .selected(vec![page.user_role.to_child()])
        .options(
            UserRole::ordered()
                .into_iter()
                .map(|role| role.to_child())
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
            .on_click(mouse_ev(Ev::Click, |_| {
                Msg::PageChanged(PageChanged::Users(UsersPageChange::ResetForm))
            }))
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

            // span![format!("{}", user.user_role)],
            li![
                class!["user"],
                span![user.name.as_str()],
                span![user.email.as_str()],
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
                .disabled(invitation.state == InvitationState::Revoked)
                .on_click(mouse_ev(Ev::Click, move |_| Msg::InviteRevokeRequest(id)))
                .build()
                .into_node();
            li![
                class!["invitation"],
                attrs![At::Class => format!("{}", invitation.state)],
                span![invitation.name.as_str()],
                span![invitation.email.as_str()],
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
