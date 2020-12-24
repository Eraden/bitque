use {
    crate::{server::InnerMsg, WebSocketActor, WsHandler, WsMessageSender, WsResult},
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
        let res = match block_on(self.db.send(invitations::ListInvitation { user_id })) {
            Ok(Ok(v)) => Some(WsMsg::InvitationListLoaded(v)),
            Ok(Err(e)) => {
                log::error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                log::error!("{}", e);
                return Ok(None);
            }
        };
        Ok(res)
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
        let invitation =
            match block_on(self.db.send(database_actor::invitations::CreateInvitation {
                user_id,
                project_id,
                email: email.clone(),
                name: name.clone(),
                role,
            })) {
                Ok(Ok(invitation)) => invitation,
                Ok(Err(e)) => {
                    error!("{:?}", e);
                    return Ok(Some(WsMsg::InvitationSendFailure));
                }
                Err(e) => {
                    error!("{}", e);
                    return Ok(Some(WsMsg::InvitationSendFailure));
                }
            };
        match block_on(self.mail.send(mail_actor::invite::Invite {
            bind_token: invitation.bind_token,
            email: invitation.email,
            inviter_name,
        })) {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(Some(WsMsg::InvitationSendFailure));
            }
            Err(e) => {
                error!("{}", e);
                return Ok(Some(WsMsg::InvitationSendFailure));
            }
        }

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
                WsMsg::Message(message),
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
        let res = match block_on(self.db.send(invitations::DeleteInvitation { id })) {
            Ok(Ok(_)) => None,
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        };
        Ok(res)
    }
}

pub struct RevokeInvitation {
    pub id: InvitationId,
}

impl WsHandler<RevokeInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: RevokeInvitation, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;
        let RevokeInvitation { id } = msg;
        let res = match block_on(self.db.send(invitations::RevokeInvitation { id })) {
            Ok(Ok(_)) => Some(WsMsg::InvitationRevokeSuccess(id)),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        };
        Ok(res)
    }
}

pub struct AcceptInvitation {
    pub invitation_token: InvitationToken,
}

impl WsHandler<AcceptInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: AcceptInvitation, ctx: &mut Self::Context) -> WsResult {
        let AcceptInvitation { invitation_token } = msg;
        let token = match block_on(
            self.db
                .send(invitations::AcceptInvitation { invitation_token }),
        ) {
            Ok(Ok(token)) => token,
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(Some(WsMsg::InvitationAcceptFailure(invitation_token)));
            }
            Err(e) => {
                error!("{}", e);
                return Ok(Some(WsMsg::InvitationAcceptFailure(invitation_token)));
            }
        };

        for message in block_on(
            self.db
                .send(database_actor::messages::LookupMessagesByToken {
                    token: invitation_token,
                    user_id: token.user_id,
                }),
        )
        .unwrap_or_else(|_| Ok(vec![]))
        .unwrap_or_default()
        {
            match block_on(self.db.send(database_actor::messages::MarkMessageSeen {
                user_id: token.user_id,
                message_id: message.id,
            })) {
                Ok(Ok(n)) => {
                    ctx.send_msg(&WsMsg::MessageMarkedSeen(message.id, n));
                }
                Ok(Err(e)) => {
                    error!("{:?}", e);
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }

        Ok(Some(WsMsg::InvitationAcceptSuccess(token.access_token)))
    }
}
