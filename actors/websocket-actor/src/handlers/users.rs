use database_actor::users::Register as DbRegister;
use database_actor::{self};
use jirs_data::msg::{WsMsgInvitation, WsMsgProject, WsMsgSession, WsMsgUser};
use jirs_data::{UserId, UserProject, UserRole, WsMsg};

use crate::handlers::auth::Authenticate;
use crate::handlers::user_settings;
use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgUser> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgUser) -> WsResult {
        match msg {
            // user settings
            WsMsgUser::UserSettingSetEditorMode(mode) => {
                self.exec(user_settings::SetTextEditorMode { mode }).await
            }

            // users
            WsMsgUser::ProfileUpdate(email, name) => self.exec(ProfileUpdate { name, email }).await,

            WsMsgUser::AvatarUrlChanged(_, _) => Ok(None),
            WsMsgUser::ProfileUpdated => Ok(None),
            WsMsgUser::UserSettingUpdated(_) => Ok(None),
        }
    }
}

pub struct LoadProjectUsers;

#[async_trait::async_trait]
impl AsyncHandler<LoadProjectUsers> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadProjectUsers) -> WsResult {
        use database_actor::users::LoadProjectUsers as Msg;

        let project_id = self.require_user_project()?.project_id;
        let v = db_or_debug_and_return!(self, Msg { project_id }; async);
        Ok(Some(WsMsg::Project(WsMsgProject::ProjectUsersLoaded(v))))
    }
}

pub struct Register {
    pub name: String,
    pub email: String,
}

#[async_trait::async_trait]
impl AsyncHandler<Register> for WebSocketActor {
    async fn exec(&mut self, msg: Register) -> WsResult {
        let Register { name, email } = msg;
        let _ = db_or_debug_and_return!(
            self,
            DbRegister {
                name: name.clone(),
                email: email.clone(),
                project_id: None,
                role: UserRole::Owner,
            },
            Ok(Some(WsMsgSession::SignUpPairTaken.into())),
            Ok(None); async
        );

        match self.exec(Authenticate { name, email }).await {
            Ok(_) => (),
            Err(e) => return Ok(Some(e)),
        };

        Ok(Some(WsMsgSession::SignUpSuccess.into()))
    }
}

pub struct LoadInvitedUsers;

#[async_trait::async_trait]
impl AsyncHandler<LoadInvitedUsers> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadInvitedUsers) -> WsResult {
        let user_id = self.require_user()?.id;

        let users = db_or_debug_and_return!(self, database_actor::users::LoadInvitedUsers { user_id }; async);

        Ok(Some(WsMsg::Invitation(
            WsMsgInvitation::InvitedUsersLoaded(users),
        )))
    }
}

pub struct ProfileUpdate {
    pub name: String,
    pub email: String,
}

#[async_trait::async_trait]
impl AsyncHandler<ProfileUpdate> for WebSocketActor {
    async fn exec(&mut self, msg: ProfileUpdate) -> WsResult {
        let user_id = self.require_user()?.id;
        let ProfileUpdate { name, email } = msg;

        let _ = db_or_debug_and_return!(
            self,
            database_actor::users::ProfileUpdate {
                user_id,
                name,
                email,
            }; async
        );

        Ok(Some(WsMsgUser::ProfileUpdated.into()))
    }
}

pub struct RemoveInvitedUser {
    pub user_id: UserId,
}

#[async_trait::async_trait]
impl AsyncHandler<RemoveInvitedUser> for WebSocketActor {
    async fn exec(&mut self, msg: RemoveInvitedUser) -> WsResult {
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
            }; async
        );
        Ok(Some(WsMsg::Invitation(
            WsMsgInvitation::InvitedUserRemoveSuccess(invited_id),
        )))
    }
}
