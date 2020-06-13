use futures::executor::block_on;

use jirs_data::{UserId, UserProject, WsMsg};

use crate::db;
use crate::db::users::Register as DbRegister;
use crate::ws::auth::Authenticate;
use crate::ws::{WebSocketActor, WsHandler, WsResult};

pub struct LoadProjectUsers;

impl WsHandler<LoadProjectUsers> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadProjectUsers, _ctx: &mut Self::Context) -> WsResult {
        use crate::db::users::LoadProjectUsers as Msg;

        let project_id = self.require_user_project()?.project_id;
        let m = match block_on(self.db.send(Msg { project_id })) {
            Ok(Ok(v)) => Some(WsMsg::ProjectUsersLoaded(v)),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
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
            project_id: None,
        })) {
            Ok(Ok(_)) => Some(WsMsg::SignUpSuccess),
            Ok(Err(_)) => Some(WsMsg::SignUpPairTaken),
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
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

pub struct ProfileUpdate {
    pub name: String,
    pub email: String,
}

impl WsHandler<ProfileUpdate> for WebSocketActor {
    fn handle_msg(&mut self, msg: ProfileUpdate, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        let ProfileUpdate { name, email } = msg;

        match block_on(self.db.send(crate::db::users::ProfileUpdate {
            user_id,
            name,
            email,
        })) {
            Ok(Ok(_users)) => (),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        };

        Ok(Some(WsMsg::ProfileUpdated))
    }
}

pub struct RemoveInvitedUser {
    pub user_id: UserId,
}

impl WsHandler<RemoveInvitedUser> for WebSocketActor {
    fn handle_msg(&mut self, msg: RemoveInvitedUser, _ctx: &mut Self::Context) -> WsResult {
        let RemoveInvitedUser {
            user_id: invited_id,
        } = msg;
        let UserProject {
            user_id: inviter_id,
            project_id,
            ..
        } = self.require_user_project()?.clone();
        match block_on(self.db.send(db::user_projects::RemoveInvitedUser {
            invited_id,
            inviter_id,
            project_id,
        })) {
            Ok(Ok(_users)) => Ok(Some(WsMsg::InvitedUserRemoveSuccess(invited_id))),
            Ok(Err(e)) => {
                error!("{:?}", e);
                Ok(None)
            }
            Err(e) => {
                error!("{}", e);
                Ok(None)
            }
        }
    }
}
