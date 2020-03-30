use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::Project;

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

        projects
            .filter(id.eq(msg.project_id))
            .first::<Project>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("Project".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProject {
    pub project_id: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

impl Message for UpdateProject {
    type Result = Result<Project, ServiceErrors>;
}

impl Handler<UpdateProject> for DbExecutor {
    type Result = Result<Project, ServiceErrors>;

    fn handle(&mut self, msg: UpdateProject, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::projects::dsl::*;
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        diesel::update(projects.find(msg.project_id))
            .set((
                msg.name.map(|v| name.eq(v)),
                msg.url.map(|v| url.eq(v)),
                msg.description.map(|v| description.eq(v)),
                msg.category.map(|v| category.eq(v)),
            ))
            .execute(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        projects
            .filter(id.eq(msg.project_id))
            .first::<Project>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("Project".to_string()))
    }
}