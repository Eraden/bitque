use database_actor::invitations;
use database_actor::messages::CreateMessageReceiver;
use futures::executor::block_on;
use jirs_data::msg::WsMsgInvitation;
use jirs_data::{
    EmailString, InvitationId, InvitationToken, MessageType, UserRole, UsernameString, WsMsg,
};

use crate::handlers::{LoadInvitedUsers, RemoveInvitedUser};
use crate::server::InnerMsg;
use crate::{
    db_or_debug_and_return, mail_or_debug_and_return, AsyncHandler, WebSocketActor, WsHandler,
    WsMessageSender, WsResult,
};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgInvitation> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgInvitation) -> WsResult {
        match msg {
            WsMsgInvitation::InvitationSendRequest { name, email, role } => {
                self.exec(CreateInvitation { email, name, role }).await
            }
            WsMsgInvitation::InvitationListLoad => self.exec(ListInvitation).await,

            WsMsgInvitation::InvitationRevokeRequest(id) => {
                self.exec(RevokeInvitation { id }).await
            }
            WsMsgInvitation::InvitedUsersLoad => self.exec(LoadInvitedUsers).await,

            WsMsgInvitation::InvitedUserRemoveRequest(user_id) => {
                self.exec(RemoveInvitedUser { user_id }).await
            }
            WsMsgInvitation::InvitationListLoaded(_) => Ok(None),
            WsMsgInvitation::InvitedUsersLoaded(_) => Ok(None),
            WsMsgInvitation::InvitationSendSuccess => Ok(None),
            WsMsgInvitation::InvitationSendFailure => Ok(None),
            WsMsgInvitation::InvitationRevokeSuccess(_) => Ok(None),
            WsMsgInvitation::InvitationAcceptRequest(_) => Ok(None),
            WsMsgInvitation::InvitationAcceptSuccess(_) => Ok(None),
            WsMsgInvitation::InvitationAcceptFailure(_) => Ok(None),
            WsMsgInvitation::InvitationRejectRequest(_) => Ok(None),
            WsMsgInvitation::InvitationRejectSuccess => Ok(None),
            WsMsgInvitation::InvitationRejectFailure(_) => Ok(None),
            WsMsgInvitation::InvitedUserRemoveSuccess(_) => Ok(None),
        }
    }
}

pub struct ListInvitation;

#[async_trait::async_trait]
impl AsyncHandler<ListInvitation> for WebSocketActor {
    async fn exec(&mut self, _msg: ListInvitation) -> WsResult {
        let user_id = match self.current_user.as_ref().map(|u| u.id) {
            Some(id) => id,
            _ => return Ok(None),
        };
        let v = db_or_debug_and_return!(self, invitations::ListInvitation { user_id }; async);
        Ok(Some(WsMsg::Invitation(
            WsMsgInvitation::InvitationListLoaded(v),
        )))
    }
}

pub struct CreateInvitation {
    pub email: EmailString,
    pub name: UsernameString,
    pub role: UserRole,
}

#[async_trait::async_trait]
impl AsyncHandler<CreateInvitation> for WebSocketActor {
    async fn exec(&mut self, msg: CreateInvitation) -> WsResult {
        let project_id = match self.current_user_project.as_ref() {
            Some(up) => up.project_id,
            _ => return Ok(None),
        };
        let (user_id, inviter_name) = self.require_user().map(|u| (u.id, u.name.clone()))?;

        let CreateInvitation { email, name, role } = msg;
        let invitation = db_or_debug_and_return!(
            self,
            database_actor::invitations::CreateInvitation {
                user_id,
                project_id,
                email: email.clone(),
                name: name.clone(),
                role,
            },
            Ok(Some(WsMsg::Invitation(WsMsgInvitation::InvitationSendFailure))),
            Ok(Some(WsMsg::Invitation(WsMsgInvitation::InvitationSendFailure))); async
        );
        let _ = mail_or_debug_and_return!(
            self,
            mail_actor::invite::Invite {
                bind_token: invitation.bind_token,
                email: invitation.email,
                inviter_name,
            },
            Ok(Some(WsMsg::Invitation(WsMsgInvitation::InvitationSendFailure))),
            Ok(Some(WsMsg::Invitation(WsMsgInvitation::InvitationSendFailure))); async
        );

        // If user exists then send message to him
        if let Ok(Ok(message)) = block_on(self.db.send(database_actor::messages::CreateMessage {
            receiver: CreateMessageReceiver::Lookup { name, email },
            sender_id: user_id,
            summary: "You have been invited to project".to_string(),
            description: "You have been invited to project".to_string(),
            message_type: MessageType::ReceivedInvitation,
            hyper_link: format!("#{}", invitation.bind_token),
        })) {
            self.addr.do_send(InnerMsg::SendToUser(
                message.receiver_id,
                WsMsg::MessageUpdated(message),
            ));
        }

        Ok(Some(WsMsg::Invitation(
            WsMsgInvitation::InvitationSendSuccess,
        )))
    }
}

pub struct DeleteInvitation {
    pub id: InvitationId,
}

#[async_trait::async_trait]
impl AsyncHandler<DeleteInvitation> for WebSocketActor {
    async fn exec(&mut self, msg: DeleteInvitation) -> WsResult {
        self.require_user()?;
        let DeleteInvitation { id } = msg;
        let _ = db_or_debug_and_return!(self, invitations::DeleteInvitation { id }; async);
        Ok(None)
    }
}

pub struct RevokeInvitation {
    pub id: InvitationId,
}

#[async_trait::async_trait]
impl AsyncHandler<RevokeInvitation> for WebSocketActor {
    async fn exec(&mut self, msg: RevokeInvitation) -> WsResult {
        self.require_user()?;
        let RevokeInvitation { id } = msg;
        let _ = db_or_debug_and_return!(self, invitations::RevokeInvitation { id }; async);
        Ok(Some(WsMsg::Invitation(
            WsMsgInvitation::InvitationRevokeSuccess(id),
        )))
    }
}

pub struct AcceptInvitation {
    pub invitation_token: InvitationToken,
}

impl WsHandler<AcceptInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: AcceptInvitation, ctx: &mut Self::Context) -> WsResult {
        let AcceptInvitation { invitation_token } = msg;
        let token = db_or_debug_and_return!(
            self,
            invitations::AcceptInvitation { invitation_token },
            Ok(Some(WsMsg::Invitation(
                WsMsgInvitation::InvitationAcceptFailure(invitation_token)
            ))),
            Ok(Some(WsMsg::Invitation(
                WsMsgInvitation::InvitationAcceptFailure(invitation_token)
            )))
        );

        for message in crate::actor_or_debug_and_fallback!(
            self,
            db,
            database_actor::messages::LookupMessagesByToken {
                token: invitation_token,
                user_id: token.user_id,
            },
            vec![],
            vec![]
        ) {
            crate::actor_or_debug_and_ignore!(
                self,
                db,
                database_actor::messages::MarkMessageSeen {
                    user_id: token.user_id,
                    message_id: message.id,
                },
                |n| {
                    ctx.send_msg(&WsMsg::MessageMarkedSeen(message.id, n));
                }
            );
        }

        Ok(Some(WsMsg::Invitation(
            WsMsgInvitation::InvitationAcceptSuccess(token.access_token),
        )))
    }
}
