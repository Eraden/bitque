use futures::executor::block_on;

use jirs_data::{EmailString, InvitationId, UsernameString, WsMsg};

use crate::db::invitations;
use crate::ws::{WebSocketActor, WsHandler, WsResult};

pub struct ListInvitation;

impl WsHandler<ListInvitation> for WebSocketActor {
    fn handle_msg(&mut self, _msg: ListInvitation, _ctx: &mut Self::Context) -> WsResult {
        let user_id = match self.current_user.as_ref().map(|u| u.id) {
            Some(id) => id,
            _ => return Ok(None),
        };
        let res = match block_on(self.db.send(invitations::ListInvitation { user_id })) {
            Ok(Ok(v)) => Some(WsMsg::InvitationListLoaded(v)),
            _ => None,
        };
        Ok(res)
    }
}

pub struct CreateInvitation {
    pub email: EmailString,
    pub name: UsernameString,
}

impl WsHandler<CreateInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateInvitation, _ctx: &mut Self::Context) -> WsResult {
        let (user_id, inviter_name, project_id) = match self
            .current_user
            .as_ref()
            .map(|u| (u.id, u.name.clone(), u.project_id))
        {
            Some(id) => id,
            _ => return Ok(None),
        };

        let CreateInvitation { email, name } = msg;
        let invitation = match block_on(self.db.send(invitations::CreateInvitation {
            user_id,
            project_id,
            email,
            name,
        })) {
            Ok(Ok(invitation)) => invitation,
            _ => return Ok(Some(WsMsg::InvitationSendFailure)),
        };
        match block_on(self.mail.send(crate::mail::invite::Invite {
            bind_token: invitation.bind_token.clone(),
            email: invitation.email.clone(),
            inviter_name,
        })) {
            Ok(Ok(_)) => (),
            _ => return Ok(Some(WsMsg::InvitationSendFailure)),
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
            _ => None,
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
            _ => None,
        };
        Ok(res)
    }
}

pub struct AcceptInvitation {
    pub id: InvitationId,
}

impl WsHandler<AcceptInvitation> for WebSocketActor {
    fn handle_msg(&mut self, msg: AcceptInvitation, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;
        let AcceptInvitation { id } = msg;
        let res = match block_on(self.db.send(invitations::AcceptInvitation { id })) {
            Ok(Ok(_)) => Some(WsMsg::InvitationAcceptSuccess(id)),
            _ => None,
        };
        Ok(res)
    }
}
