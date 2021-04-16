use database_actor as db;
use futures::executor::block_on;
use jirs_data::{UserProjectId, WsMsg};

use crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

pub struct LoadUserProjects;

impl WsHandler<LoadUserProjects> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadUserProjects, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        let v = db_or_debug_and_return!(self, db::user_projects::LoadUserProjects { user_id });
        Ok(Some(WsMsg::UserProjectsLoaded(v)))
    }
}

pub struct SetCurrentUserProject {
    pub id: UserProjectId,
}

impl WsHandler<SetCurrentUserProject> for WebSocketActor {
    fn handle_msg(&mut self, msg: SetCurrentUserProject, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        let user_project = db_or_debug_and_return!(
            self,
            db::user_projects::ChangeCurrentUserProject {
                user_id,
                id: msg.id,
            }
        );
        self.current_user_project = Some(user_project.clone());
        Ok(Some(WsMsg::UserProjectCurrentChanged(user_project)))
    }
}
