use database_actor as db;
use jirs_data::{UserProjectId, WsMsg};

use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

pub struct LoadUserProjects;

#[async_trait::async_trait]
impl AsyncHandler<LoadUserProjects> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadUserProjects) -> WsResult {
        let user_id = self.require_user()?.id;
        let v =
            db_or_debug_and_return!(self, db::user_projects::LoadUserProjects { user_id }; async);
        Ok(Some(WsMsg::UserProjectsLoaded(v)))
    }
}

pub struct SetCurrentUserProject {
    pub id: UserProjectId,
}

#[async_trait::async_trait]
impl AsyncHandler<SetCurrentUserProject> for WebSocketActor {
    async fn exec(&mut self, msg: SetCurrentUserProject) -> WsResult {
        let user_id = self.require_user()?.id;
        let user_project = db_or_debug_and_return!(
            self,
            db::user_projects::ChangeCurrentUserProject {
                user_id,
                id: msg.id,
            }; async
        );
        self.current_user_project = Some(user_project.clone());
        Ok(Some(WsMsg::UserProjectCurrentChanged(user_project)))
    }
}
