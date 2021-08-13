use jirs_data::msg::{WsMsgEpic, WsMsgIssue};
use jirs_data::{
    DescriptionString, EndsAt, EpicId, IssueType, NameString, StartsAt, UserProject, WsMsg,
};

use crate::{db_or_debug_and_return, *};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgEpic> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgEpic) -> WsResult {
        match msg {
            WsMsgEpic::EpicsLoad => self.exec(epics::LoadEpics).await,
            WsMsgEpic::EpicCreate(name, description, description_html) => {
                self.exec(epics::CreateEpic {
                    name,
                    description,
                    description_html,
                })
                .await
            }
            WsMsgEpic::EpicUpdateName(epic_id, name) => {
                self.exec(epics::UpdateEpicName { epic_id, name }).await
            }
            WsMsgEpic::EpicUpdateStartsAt(epic_id, starts_at) => {
                self.exec(epics::UpdateEpicStartsAt { epic_id, starts_at })
                    .await
            }
            WsMsgEpic::EpicUpdateEndsAt(epic_id, ends_at) => {
                self.exec(epics::UpdateEpicEndsAt { epic_id, ends_at })
                    .await
            }
            WsMsgEpic::EpicDelete(epic_id) => self.exec(epics::DeleteEpic { epic_id }).await,
            WsMsgEpic::EpicTransform(epic_id, issue_type) => {
                self.exec(epics::TransformEpic {
                    epic_id,
                    issue_type,
                })
                .await
            }
            WsMsgEpic::EpicsLoaded(_) => Ok(None),
            WsMsgEpic::EpicCreated(_) => Ok(None),
            WsMsgEpic::EpicUpdated(_) => Ok(None),
            WsMsgEpic::EpicDeleted(_, _) => Ok(None),
        }
    }
}

pub struct LoadEpics;

#[async_trait::async_trait]
impl AsyncHandler<LoadEpics> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadEpics) -> WsResult {
        let project_id = self.require_user_project()?.project_id;
        let epics =
            db_or_debug_and_return!(self, database_actor::epics::LoadEpics { project_id }; async);
        Ok(Some(WsMsg::Epic(WsMsgEpic::EpicsLoaded(epics))))
    }
}

pub struct CreateEpic {
    pub name: NameString,
    pub description: Option<DescriptionString>,
    pub description_html: Option<DescriptionString>,
}

#[async_trait::async_trait]
impl AsyncHandler<CreateEpic> for WebSocketActor {
    async fn exec(&mut self, msg: CreateEpic) -> WsResult {
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
            }; async
        );
        Ok(Some(WsMsg::Epic(WsMsgEpic::EpicCreated(epic))))
    }
}

pub struct UpdateEpicName {
    pub epic_id: EpicId,
    pub name: NameString,
}

#[async_trait::async_trait]
impl AsyncHandler<UpdateEpicName> for WebSocketActor {
    async fn exec(&mut self, msg: UpdateEpicName) -> WsResult {
        let UserProject { project_id, .. } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::UpdateEpicName {
                project_id: *project_id,
                epic_id: msg.epic_id,
                name: msg.name,
            }; async
        );
        Ok(Some(WsMsg::Epic(WsMsgEpic::EpicUpdated(epic))))
    }
}

pub struct UpdateEpicStartsAt {
    pub epic_id: EpicId,
    pub starts_at: Option<StartsAt>,
}

#[async_trait::async_trait]
impl AsyncHandler<UpdateEpicStartsAt> for WebSocketActor {
    async fn exec(&mut self, msg: UpdateEpicStartsAt) -> WsResult {
        let UserProject { project_id, .. } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::UpdateEpicStartsAt {
                project_id: *project_id,
                epic_id: msg.epic_id,
                starts_at: msg.starts_at,
            }; async
        );
        Ok(Some(WsMsg::Epic(WsMsgEpic::EpicUpdated(epic))))
    }
}

pub struct UpdateEpicEndsAt {
    pub epic_id: EpicId,
    pub ends_at: Option<EndsAt>,
}

#[async_trait::async_trait]
impl AsyncHandler<UpdateEpicEndsAt> for WebSocketActor {
    async fn exec(&mut self, msg: UpdateEpicEndsAt) -> WsResult {
        let UserProject { project_id, .. } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::UpdateEpicEndsAt {
                project_id: *project_id,
                epic_id: msg.epic_id,
                ends_at: msg.ends_at,
            }; async
        );
        Ok(Some(WsMsg::Epic(WsMsgEpic::EpicUpdated(epic))))
    }
}

pub struct DeleteEpic {
    pub epic_id: EpicId,
}

#[async_trait::async_trait]
impl AsyncHandler<DeleteEpic> for WebSocketActor {
    async fn exec(&mut self, msg: DeleteEpic) -> WsResult {
        let DeleteEpic { epic_id } = msg;
        let UserProject { user_id, .. } = self.require_user_project()?;
        let n = db_or_debug_and_return!(
            self,
            database_actor::epics::DeleteEpic {
                user_id: *user_id,
                epic_id,
            }; async
        );
        Ok(Some(WsMsg::Epic(WsMsgEpic::EpicDeleted(epic_id, n))))
    }
}

pub struct TransformEpic {
    pub epic_id: EpicId,
    pub issue_type: IssueType,
}

#[async_trait::async_trait]
impl AsyncHandler<TransformEpic> for WebSocketActor {
    async fn exec(&mut self, msg: TransformEpic) -> WsResult {
        let epic: jirs_data::Epic = db_or_debug_and_return!(
            self,
            database_actor::epics::FindEpic {
                epic_id: msg.epic_id
            }; async
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
            }; async
        );
        let n = db_or_debug_and_return!(
            self,
            database_actor::epics::DeleteEpic {
                user_id: epic.user_id,
                epic_id: epic.id
            }; async
        );
        self.broadcast(&WsMsg::Epic(WsMsgEpic::EpicDeleted(msg.epic_id, n)));
        self.broadcast(&WsMsg::Issue(WsMsgIssue::IssueCreated(issue.into())));
        Ok(None)
    }
}
