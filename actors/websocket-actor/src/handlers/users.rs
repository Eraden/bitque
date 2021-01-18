use {
    crate::{
        db_or_debug_and_return, handlers::auth::Authenticate, WebSocketActor, WsHandler, WsResult,
    },
    database_actor::{self, users::Register as DbRegister},
    futures::executor::block_on,
    jirs_data::{UserId, UserProject, UserRole, WsMsg},
};

pub struct LoadProjectUsers;

impl WsHandler<LoadProjectUsers> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadProjectUsers, _ctx: &mut Self::Context) -> WsResult {
        use database_actor::users::LoadProjectUsers as Msg;

        let project_id = self.require_user_project()?.project_id;
        let v = db_or_debug_and_return!(self, Msg { project_id });
        Ok(Some(WsMsg::ProjectUsersLoaded(v)))
    }
}

pub struct Register {
    pub name: String,
    pub email: String,
}

impl WsHandler<Register> for WebSocketActor {
    fn handle_msg(&mut self, msg: Register, ctx: &mut Self::Context) -> WsResult {
        let Register { name, email } = msg;
        let _ = db_or_debug_and_return!(
            self,
            DbRegister {
                name: name.clone(),
                email: email.clone(),
                project_id: None,
                role: UserRole::Owner,
            },
            Ok(Some(WsMsg::SignUpPairTaken)),
            Ok(None)
        );

        match self.handle_msg(Authenticate { name, email }, ctx) {
            Ok(_) => (),
            Err(e) => return Ok(Some(e)),
        };

        Ok(Some(WsMsg::SignUpSuccess))
    }
}

pub struct LoadInvitedUsers;

impl WsHandler<LoadInvitedUsers> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadInvitedUsers, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;

        let users =
            db_or_debug_and_return!(self, database_actor::users::LoadInvitedUsers { user_id });

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

        let _ = db_or_debug_and_return!(
            self,
            database_actor::users::ProfileUpdate {
                user_id,
                name,
                email,
            }
        );

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
        let _ = db_or_debug_and_return!(
            self,
            database_actor::user_projects::RemoveInvitedUser {
                invited_id,
                inviter_id,
                project_id,
            }
        );
        Ok(Some(WsMsg::InvitedUserRemoveSuccess(invited_id)))
    }
}
