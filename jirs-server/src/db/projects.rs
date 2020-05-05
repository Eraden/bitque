use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::{Project, ProjectCategory, TimeTracking};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

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
        use crate::schema::projects::dsl::projects;
        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = projects.find(msg.project_id);

        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );

        query
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
    pub category: Option<ProjectCategory>,
    pub time_tracking: Option<TimeTracking>,
}

impl Message for UpdateProject {
    type Result = Result<Project, ServiceErrors>;
}

impl Handler<UpdateProject> for DbExecutor {
    type Result = Result<Project, ServiceErrors>;

    fn handle(&mut self, msg: UpdateProject, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::projects::dsl::*;
        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let update_query = diesel::update(projects.find(msg.project_id)).set((
            msg.name.map(|v| name.eq(v)),
            msg.url.map(|v| url.eq(v)),
            msg.description.map(|v| description.eq(v)),
            msg.category.map(|v| category.eq(v)),
            msg.time_tracking.map(|v| time_tracking.eq(v)),
        ));
        debug!("{}", diesel::debug_query::<Pg, _>(&update_query));
        update_query
            .execute(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;

        let project_query = projects.find(msg.project_id);
        debug!("{}", diesel::debug_query::<Pg, _>(&project_query));
        project_query
            .first::<Project>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("Project".to_string()))
    }
}
