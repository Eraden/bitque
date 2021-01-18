use {
    crate::{
        db_or_debug_and_return, mail_or_debug_and_return, server::InnerMsg, WebSocketActor,
        WsHandler, WsMessageSender, WsResult,
    },
    database_actor::{invitations, messages::CreateMessageReceiver},
    futures::executor::block_on,
    jirs_data::{
        EmailString, InvitationId, InvitationToken, MessageType, UserRole, UsernameString, WsMsg,
    },
};

pub struct ListInvitation;

impl WsHandler<ListInvitation> for WebSocketActor {
    fn handle_msg(&mut self, _msg: ListInvitation, _ctx: &mut Self::Context) -> WsResult {
        let user_id = match self.current_user.as_ref().map(|u| u.id) {
            Some(id) => id,
            _ => return Ok(None),
        };
        let v = db_or_debug_and_return!(self, invitations::ListInvitation { user_id });
        Ok(Some(WsMsg::InvitationListLoaded(v)))
    }
}

pub struct CreateInvitation {
    pub email: EmailString,
    pub name: UsernameString,
    pub role: UserRole,
}

impl WsHandler<CreateInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateInvitation, _ctx: &mut Self::Context) -> WsResult {
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
            Ok(Some(WsMsg::InvitationSendFailure)),
            Ok(Some(WsMsg::InvitationSendFailure))
        );
        let _ = mail_or_debug_and_return!(
            self,
            mail_actor::invite::Invite {
                bind_token: invitation.bind_token,
                email: invitation.email,
                inviter_name,
            },
            Ok(Some(WsMsg::InvitationSendFailure)),
            Ok(Some(WsMsg::InvitationSendFailure))
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

        Ok(Some(WsMsg::InvitationSendSuccess))
    }
}

pub struct DeleteInvitation {
    pub id: InvitationId,
}

impl WsHandler<DeleteInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteInvitation, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;
        let DeleteInvitation { id } = msg;
        let _ = db_or_debug_and_return!(self, invitations::DeleteInvitation { id });
        Ok(None)
    }
}

pub struct RevokeInvitation {
    pub id: InvitationId,
}

impl WsHandler<RevokeInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: RevokeInvitation, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;
        let RevokeInvitation { id } = msg;
        let _ = db_or_debug_and_return!(self, invitations::RevokeInvitation { id });
        Ok(Some(WsMsg::InvitationRevokeSuccess(id)))
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
            Ok(Some(WsMsg::InvitationAcceptFailure(invitation_token))),
            Ok(Some(WsMsg::InvitationAcceptFailure(invitation_token)))
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

        Ok(Some(WsMsg::InvitationAcceptSuccess(token.access_token)))
    }
}
