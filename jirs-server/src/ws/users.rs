use futures::executor::block_on;

use jirs_data::WsMsg;

use crate::db::users::Register as DbRegister;
use crate::ws::auth::Authenticate;
use crate::ws::{WebSocketActor, WsHandler, WsResult};

pub struct LoadProjectUsers;

impl WsHandler<LoadProjectUsers> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadProjectUsers, _ctx: &mut Self::Context) -> WsResult {
        use crate::db::users::LoadProjectUsers as Msg;

        let project_id = self.require_user()?.project_id;
        let m = match block_on(self.db.send(Msg { project_id })) {
            Ok(Ok(v)) => Some(WsMsg::ProjectUsersLoaded(v)),
            _ => None,
        };
        Ok(m)
    }
}

pub struct Register {
    pub name: String,
    pub email: String,
}

impl WsHandler<Register> for WebSocketActor {
    fn handle_msg(&mut self, msg: Register, ctx: &mut Self::Context) -> WsResult {
        let Register { name, email } = msg;
        let msg = match block_on(self.db.send(DbRegister {
            name: name.clone(),
            email: email.clone(),
        })) {
            Ok(Ok(_)) => Some(WsMsg::SignUpSuccess),
            Ok(Err(_)) => Some(WsMsg::SignUpPairTaken),
            _ => None,
        };

        match self.handle_msg(Authenticate { name, email }, ctx) {
            Ok(_) => (),
            Err(e) => return Ok(Some(e)),
        };

        Ok(msg)
    }
}

pub struct LoadInvitedUsers;

impl WsHandler<LoadInvitedUsers> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadInvitedUsers, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;

        let users = match block_on(self.db.send(crate::db::users::LoadInvitedUsers { user_id })) {
            Ok(Ok(users)) => users,
            _ => return Ok(None),
        };

        Ok(Some(WsMsg::InvitedUsersLoaded(users)))
    }
}
