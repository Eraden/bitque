use {
    crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult},
    futures::executor::block_on,
    jirs_data::{EpicId, NameString, UserProject, WsMsg},
};

pub struct LoadEpics;

impl WsHandler<LoadEpics> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadEpics, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user_project()?.project_id;
        let epics = db_or_debug_and_return!(self, database_actor::epics::LoadEpics { project_id });
        Ok(Some(WsMsg::EpicsLoaded(epics)))
    }
}

pub struct CreateEpic {
    pub name: NameString,
}

impl WsHandler<CreateEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateEpic, _ctx: &mut Self::Context) -> WsResult {
        let CreateEpic { name } = msg;
        let UserProject {
            user_id,
            project_id,
            ..
        } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::CreateEpic {
                user_id: *user_id,
                project_id: *project_id,
                name,
            }
        );
        Ok(Some(WsMsg::EpicCreated(epic)))
    }
}

pub struct UpdateEpic {
    pub epic_id: EpicId,
    pub name: NameString,
}

impl WsHandler<UpdateEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateEpic, _ctx: &mut Self::Context) -> WsResult {
        let UpdateEpic { epic_id, name } = msg;
        let UserProject { project_id, .. } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::UpdateEpic {
                project_id: *project_id,
                epic_id: epic_id,
                name: name.clone(),
            }
        );
        Ok(Some(WsMsg::EpicUpdated(epic)))
    }
}

pub struct DeleteEpic {
    pub epic_id: EpicId,
}

impl WsHandler<DeleteEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteEpic, _ctx: &mut Self::Context) -> WsResult {
        let DeleteEpic { epic_id } = msg;
        let UserProject { user_id, .. } = self.require_user_project()?;
        let n = db_or_debug_and_return!(
            self,
            database_actor::epics::DeleteEpic {
                user_id: *user_id,
                epic_id: epic_id,
            }
        );
        Ok(Some(WsMsg::EpicDeleted(epic_id, n)))
    }
}
