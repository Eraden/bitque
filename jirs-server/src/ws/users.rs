use futures::executor::block_on;

use jirs_data::WsMsg;

use crate::db::users::Register as DbRegister;
use crate::ws::auth::Authenticate;
use crate::ws::{current_user, WebSocketActor, WsHandler, WsResult};

pub struct LoadProjectUsers;

impl WsHandler<LoadProjectUsers> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadProjectUsers, _ctx: &mut Self::Context) -> WsResult {
        use crate::db::users::LoadProjectUsers as Msg;

        let project_id = current_user(&self.current_user).map(|u| u.project_id)?;
        let m = match block_on(self.db.send(Msg { project_id })) {
            Ok(Ok(v)) => Some(WsMsg::ProjectUsersLoaded(
                v.into_iter().map(|i| i.into()).collect(),
            )),
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
