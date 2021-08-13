use database_actor as db;
use jirs_data::msg::WsMsgProject;
use jirs_data::{UpdateProjectPayload, UserProject, WsMsg};

use crate::handlers::{LoadIssues, LoadProjectUsers};
use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgProject> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgProject) -> WsResult {
        match msg {
            WsMsgProject::ProjectIssuesLoad => self.exec(LoadIssues).await,
            WsMsgProject::ProjectsLoad => self.exec(LoadProjects).await,
            WsMsgProject::ProjectUpdateLoad(payload) => self.exec(payload).await,
            WsMsgProject::ProjectUsersLoad => self.exec(LoadProjectUsers).await,
            WsMsgProject::ProjectsLoaded(_) => Ok(None),
            WsMsgProject::ProjectIssuesLoaded(_) => Ok(None),
            WsMsgProject::ProjectUsersLoaded(_) => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl AsyncHandler<UpdateProjectPayload> for WebSocketActor {
    async fn exec(&mut self, msg: UpdateProjectPayload) -> WsResult {
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
            }; async
        );
        let projects = db_or_debug_and_return!(
            self,
            database_actor::projects::LoadProjects { user_id: *user_id }; async
        );
        Ok(Some(WsMsg::Project(WsMsgProject::ProjectsLoaded(projects))))
    }
}

pub struct LoadProjects;

#[async_trait::async_trait]
impl AsyncHandler<LoadProjects> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadProjects) -> WsResult {
        let user_id = self.require_user()?.id;
        let v = db_or_debug_and_return!(self, db::projects::LoadProjects { user_id }; async);
        Ok(Some(WsMsg::Project(WsMsgProject::ProjectsLoaded(v))))
    }
}
