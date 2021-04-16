use database_actor as db;
use futures::executor::block_on;
use jirs_data::{UpdateProjectPayload, UserProject, WsMsg};

use crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

impl WsHandler<UpdateProjectPayload> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateProjectPayload, _ctx: &mut Self::Context) -> WsResult {
        let UserProject {
            user_id,
            project_id,
            ..
        } = self.require_user_project()?;
        let _ = db_or_debug_and_return!(
            self,
            database_actor::projects::UpdateProject {
                project_id: *project_id,
                name: msg.name,
                url: msg.url,
                description: msg.description,
                category: msg.category,
                time_tracking: msg.time_tracking,
            }
        );
        let projects = db_or_debug_and_return!(
            self,
            database_actor::projects::LoadProjects { user_id: *user_id }
        );
        Ok(Some(WsMsg::ProjectsLoaded(projects)))
    }
}

pub struct LoadProjects;

impl WsHandler<LoadProjects> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadProjects, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        let v = db_or_debug_and_return!(self, db::projects::LoadProjects { user_id });
        Ok(Some(WsMsg::ProjectsLoaded(v)))
    }
}
