use actix::{Handler, Message};
use futures::executor::block_on;

use jirs_data::{EmailString, InvitationId, UsernameString, WsMsg};

use crate::db::invitations;
use crate::ws::{WebSocketActor, WsResult};

pub struct ListInvitation;

impl Message for ListInvitation {
    type Result = WsResult;
}

impl Handler<ListInvitation> for WebSocketActor {
    type Result = WsResult;

    fn handle(&mut self, _msg: ListInvitation, _ctx: &mut Self::Context) -> Self::Result {
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

impl Message for CreateInvitation {
    type Result = WsResult;
}

impl Handler<CreateInvitation> for WebSocketActor {
    type Result = WsResult;

    fn handle(&mut self, msg: CreateInvitation, _ctx: &mut Self::Context) -> Self::Result {
        let (user_id, project_id) = match self.current_user.as_ref().map(|u| (u.id, u.project_id)) {
            Some(id) => id,
            _ => return Ok(None),
        };
        let CreateInvitation { email, name } = msg;
        let res = match block_on(self.db.send(invitations::CreateInvitation {
            user_id,
            project_id,
            email,
            name,
        })) {
            Ok(Ok(_invitation)) => Some(WsMsg::InvitationSendSuccess),
            _ => Some(WsMsg::InvitationSendFailure),
        };
        Ok(res)
    }
}

pub struct DeleteInvitation {
    pub id: InvitationId,
}

impl Message for DeleteInvitation {
    type Result = WsResult;
}

impl Handler<DeleteInvitation> for WebSocketActor {
    type Result = WsResult;

    fn handle(&mut self, msg: DeleteInvitation, _ctx: &mut Self::Context) -> Self::Result {
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

impl Message for RevokeInvitation {
    type Result = WsResult;
}

impl Handler<RevokeInvitation> for WebSocketActor {
    type Result = WsResult;

    fn handle(&mut self, msg: RevokeInvitation, _ctx: &mut Self::Context) -> Self::Result {
        self.require_user()?;
        let RevokeInvitation { id } = msg;
        let res = match block_on(self.db.send(invitations::RevokeInvitation { id })) {
            Ok(Ok(_)) => Some(WsMsg::InvitationRevokeSuccess(id)),
            _ => None,
        };
        Ok(res)
    }
}
