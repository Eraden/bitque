use jirs_data::msg::WsMsgInvitation;
use jirs_data::{InvitationState, UserRole, UsersFieldId, WsMsg};
use seed::prelude::Orders;

use crate::components::styled_select::StyledSelectChanged;
use crate::model::{InvitationFormState, Model, Page, PageContent};
use crate::pages::users_page::model::UsersPage;
use crate::ws::{invitation_load, send_ws_msg};
use crate::{match_page_mut, FieldId, Msg, PageChanged, UsersPageChange, WebSocketChanged};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.user.is_none() {
        return;
    }

    if let Msg::ChangePage(Page::Users) = msg {
        build_page_content(model);
    }

    let page = match_page_mut!(model, Users);

    page.user_role_state.update(&msg, orders);

    match msg {
        Msg::ChangePage(Page::Users) if model.user.is_some() => {
            invitation_load(model, orders);
        }
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(Ok(_))) if model.user.is_some() => {
                invitation_load(model, orders);
            }
            WebSocketChanged::WsMsg(WsMsg::Invitation(WsMsgInvitation::InvitedUsersLoaded(
                users,
            ))) => {
                page.invited_users = users;
            }
            WebSocketChanged::WsMsg(WsMsg::Invitation(WsMsgInvitation::InvitationListLoaded(
                invitations,
            ))) => {
                page.invitations = invitations;
            }
            WebSocketChanged::WsMsg(WsMsg::Invitation(
                WsMsgInvitation::InvitationRevokeSuccess(id),
            )) => {
                let mut old = vec![];
                std::mem::swap(&mut page.invitations, &mut old);
                for mut invitation in old {
                    if id == invitation.id {
                        invitation.state = InvitationState::Revoked;
                    }
                    page.invitations.push(invitation);
                }
                send_ws_msg(
                    WsMsgInvitation::InvitationListLoad.into(),
                    model.ws.as_ref(),
                    orders,
                );
            }
            WebSocketChanged::WsMsg(WsMsg::Invitation(
                WsMsgInvitation::InvitedUserRemoveSuccess(removed_id),
            )) => {
                let mut old = vec![];
                std::mem::swap(&mut page.invited_users, &mut old);
                for user in old {
                    if user.id != removed_id {
                        page.invited_users.push(user);
                    }
                }
            }
            WebSocketChanged::WsMsg(WsMsg::Invitation(WsMsgInvitation::InvitationSendSuccess)) => {
                send_ws_msg(
                    WsMsgInvitation::InvitationListLoad.into(),
                    model.ws.as_ref(),
                    orders,
                );
                page.form_state = InvitationFormState::Succeed;
            }
            WebSocketChanged::WsMsg(WsMsg::Invitation(WsMsgInvitation::InvitationSendFailure)) => {
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
                WsMsg::Invitation(WsMsgInvitation::InvitationSendRequest {
                    name: page.name.clone(),
                    email: page.email.clone(),
                    role,
                }),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::InviteRevokeRequest(invitation_id) => {
            send_ws_msg(
                WsMsg::Invitation(WsMsgInvitation::InvitationRevokeRequest(invitation_id)),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::InvitedUserRemove(user_id) => {
            send_ws_msg(
                WsMsg::Invitation(WsMsgInvitation::InvitedUserRemoveRequest(user_id)),
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
