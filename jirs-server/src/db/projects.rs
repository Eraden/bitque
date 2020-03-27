use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::Project;
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoadCurrentProject {
    pub project_id: i32,
}

impl Message for LoadCurrentProject {
    type Result = Result<Project, ServiceErrors>;
}

impl Handler<LoadCurrentProject> for DbExecutor {
    type Result = Result<Project, ServiceErrors>;

    fn handle(&mut self, msg: LoadCurrentProject, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::projects::dsl::{id, projects};
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let project = projects
            .filter(id.eq(msg.project_id))
            .first::<Project>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("Project".to_string()))?;
        Ok(project)
    }
}
