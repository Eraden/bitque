use futures::executor::block_on;

use jirs_data::{UpdateProjectPayload, WsMsg};

use crate::db::projects::LoadCurrentProject;
use crate::ws::{WebSocketActor, WsHandler, WsResult};

pub struct CurrentProject;

impl WsHandler<CurrentProject> for WebSocketActor {
    fn handle_msg(&mut self, _msg: CurrentProject, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user()?.project_id;

        let m = match block_on(self.db.send(LoadCurrentProject { project_id })) {
            Ok(Ok(project)) => Some(WsMsg::ProjectLoaded(project)),
            Ok(Err(e)) => {
                error!("{:?}", e);
                None
            }
            Err(e) => {
                error!("{:?}", e);
                None
            }
        };
        Ok(m)
    }
}

impl WsHandler<UpdateProjectPayload> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateProjectPayload, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user()?.project_id;
        let project = match block_on(self.db.send(crate::db::projects::UpdateProject {
            project_id,
            name: msg.name,
            url: msg.url,
            description: msg.description,
            category: msg.category,
            time_tracking: None,
        })) {
            Ok(Ok(project)) => project,
            _ => return Ok(None),
        };
        Ok(Some(WsMsg::ProjectLoaded(project)))
    }
}
