use futures::executor::block_on;

use database_actor as db;
use jirs_data::{UpdateProjectPayload, UserProject, WsMsg};

use crate::{WebSocketActor, WsHandler, WsResult};

impl WsHandler<UpdateProjectPayload> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateProjectPayload, _ctx: &mut Self::Context) -> WsResult {
        let UserProject {
            user_id,
            project_id,
            ..
        } = self.require_user_project()?;
        match block_on(self.db.send(database_actor::projects::UpdateProject {
            project_id: *project_id,
            name: msg.name,
            url: msg.url,
            description: msg.description,
            category: msg.category,
            time_tracking: msg.time_tracking,
        })) {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{:?}", e);
                return Ok(None);
            }
        };
        let projects = match block_on(
            self.db
                .send(database_actor::projects::LoadProjects { user_id: *user_id }),
        ) {
            Ok(Ok(projects)) => projects,
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{:?}", e);
                return Ok(None);
            }
        };
        Ok(Some(WsMsg::ProjectsLoaded(projects)))
    }
}

pub struct LoadProjects;

impl WsHandler<LoadProjects> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadProjects, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        match block_on(self.db.send(db::projects::LoadProjects { user_id })) {
            Ok(Ok(v)) => Ok(Some(WsMsg::ProjectsLoaded(v))),
            Ok(Err(e)) => {
                error!("{:?}", e);
                Ok(None)
            }
            Err(e) => {
                error!("{:?}", e);
                Ok(None)
            }
        }
    }
}
