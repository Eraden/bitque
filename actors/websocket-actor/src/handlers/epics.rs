use futures::executor::block_on;
use jirs_data::{DescriptionString, EpicId, IssueType, NameString, UserProject, WsMsg};

use crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

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
    pub description: Option<DescriptionString>,
    pub description_html: Option<DescriptionString>,
}

impl WsHandler<CreateEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateEpic, _ctx: &mut Self::Context) -> WsResult {
        let CreateEpic {
            name,
            description,
            description_html,
        } = msg;
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
                description,
                description_html,
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
                epic_id,
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
                epic_id,
            }
        );
        Ok(Some(WsMsg::EpicDeleted(epic_id, n)))
    }
}

pub struct TransformEpic {
    pub epic_id: EpicId,
    pub issue_type: IssueType,
}

impl WsHandler<TransformEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: TransformEpic, _ctx: &mut Self::Context) -> WsResult {
        let epic: jirs_data::Epic = db_or_debug_and_return!(
            self,
            database_actor::epics::FindEpic {
                epic_id: msg.epic_id
            }
        );
        let issue: database_actor::models::Issue = db_or_debug_and_return!(
            self,
            database_actor::issues::CreateIssue {
                title: epic.name,
                issue_type: msg.issue_type,
                issue_status_id: 0,
                priority: Default::default(),
                description: epic.description_html,
                description_text: epic.description,
                estimate: None,
                time_spent: None,
                time_remaining: None,
                project_id: epic.project_id,
                reporter_id: epic.user_id,
                user_ids: vec![epic.user_id],
                epic_id: None
            }
        );
        let n = db_or_debug_and_return!(
            self,
            database_actor::epics::DeleteEpic {
                user_id: epic.user_id,
                epic_id: epic.id
            }
        );
        self.broadcast(&WsMsg::EpicDeleted(msg.epic_id, n));
        self.broadcast(&WsMsg::IssueCreated(issue.into()));
        Ok(None)
    }
}
