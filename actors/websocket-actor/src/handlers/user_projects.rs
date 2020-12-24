use futures::executor::block_on;

use database_actor as db;
use jirs_data::{UserProjectId, WsMsg};

use crate::{WebSocketActor, WsHandler, WsResult};

pub struct LoadUserProjects;

impl WsHandler<LoadUserProjects> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadUserProjects, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        match block_on(
            self.db
                .send(db::user_projects::LoadUserProjects { user_id }),
        ) {
            Ok(Ok(v)) => Ok(Some(WsMsg::UserProjectsLoaded(v))),
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

pub struct SetCurrentUserProject {
    pub id: UserProjectId,
}

impl WsHandler<SetCurrentUserProject> for WebSocketActor {
    fn handle_msg(&mut self, msg: SetCurrentUserProject, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        match block_on(self.db.send(db::user_projects::ChangeCurrentUserProject {
            user_id,
            id: msg.id,
        })) {
            Ok(Ok(user_project)) => {
                self.current_user_project = Some(user_project.clone());
                Ok(Some(WsMsg::UserProjectCurrentChanged(user_project)))
            }
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
