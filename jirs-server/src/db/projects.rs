use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::{NameString, Project, ProjectCategory, ProjectId, TimeTracking, UserId};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::schema::projects::all_columns;

#[derive(Serialize, Deserialize)]
pub struct LoadCurrentProject {
    pub project_id: ProjectId,
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
pub struct CreateProject {
    pub name: NameString,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<ProjectCategory>,
    pub time_tracking: Option<TimeTracking>,
}

impl Message for CreateProject {
    type Result = Result<Project, ServiceErrors>;
}

impl Handler<CreateProject> for DbExecutor {
    type Result = Result<Project, ServiceErrors>;

    fn handle(&mut self, msg: CreateProject, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::projects::dsl::*;
        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = diesel::insert_into(projects)
            .values((
                name.eq(msg.name),
                msg.url.map(|v| url.eq(v)),
                msg.description.map(|v| description.eq(v)),
                msg.category.map(|v| category.eq(v)),
                msg.time_tracking.map(|v| time_tracking.eq(v)),
            ))
            .returning(all_columns);
        debug!("{}", diesel::debug_query::<Pg, _>(&query));
        query
            .get_result::<Project>(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProject {
    pub project_id: ProjectId,
    pub name: Option<NameString>,
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

pub struct LoadProjects {
    pub user_id: UserId,
}

impl Message for LoadProjects {
    type Result = Result<Vec<Project>, ServiceErrors>;
}

impl Handler<LoadProjects> for DbExecutor {
    type Result = Result<Vec<Project>, ServiceErrors>;

    fn handle(&mut self, msg: LoadProjects, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::projects::dsl::*;
        use crate::schema::user_projects::dsl::{project_id, user_id, user_projects};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = projects
            .inner_join(user_projects.on(project_id.eq(id)))
            .filter(user_id.eq(msg.user_id))
            .distinct_on(id)
            .select(all_columns);
        debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
        query
            .load::<Project>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("Project".to_string()))
    }
}
