use {
    crate::{
        components::styled_select::StyledSelectChanged,
        model::{InvitationFormState, Model, Page, PageContent},
        pages::users_page::model::UsersPage,
        ws::{invitation_load, send_ws_msg},
        FieldId, Msg, PageChanged, UsersPageChange, WebSocketChanged,
    },
    jirs_data::{InvitationState, UserRole, UsersFieldId, WsMsg},
    seed::{log, prelude::Orders},
};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    log!(model);
    if model.user.is_none() {
        return;
    }

    if let Msg::ChangePage(Page::Users) = msg {
        build_page_content(model);
    }

    let page = match &mut model.page_content {
        PageContent::Users(page) => page,
        _ => return,
    };

    page.user_role_state.update(&msg, orders);

    match msg {
        Msg::ChangePage(Page::Users) if model.user.is_some() => {
            invitation_load(model, orders);
        }
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(Ok(_))) if model.user.is_some() => {
                invitation_load(model, orders);
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
                send_ws_msg(WsMsg::InvitationListLoad, model.ws.as_ref(), orders);
            }
            WebSocketChanged::WsMsg(WsMsg::InvitedUserRemoveSuccess(removed_id)) => {
                let mut old = vec![];
                std::mem::swap(&mut page.invited_users, &mut old);
                for user in old {
                    if user.id != removed_id {
                        page.invited_users.push(user);
                    }
                }
            }
            WebSocketChanged::WsMsg(WsMsg::InvitationSendSuccess) => {
                send_ws_msg(WsMsg::InvitationListLoad, model.ws.as_ref(), orders);
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
            StyledSelectChanged::Changed(Some(role)),
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
            let role: UserRole = match page.user_role_state.values.first() {
                Some(i) => (*i).into(),
                _ => return,
            };

            page.form_state = InvitationFormState::Sent;
            send_ws_msg(
                WsMsg::InvitationSendRequest {
                    name: page.name.clone(),
                    email: page.email.clone(),
                    role,
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
        Msg::InvitedUserRemove(user_id) => {
            send_ws_msg(
                WsMsg::InvitedUserRemoveRequest(user_id),
                model.ws.as_ref(),
                orders,
            );
        }
        _ => (),
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::Users(Box::new(UsersPage::default()));
}
