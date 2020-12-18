use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::{NameString, Project, ProjectCategory, ProjectId, TimeTracking, UserId};

use crate::db::DbPooledConn;
use crate::{db::DbExecutor, db_pool, errors::ServiceError, q, schema::projects::all_columns};

#[derive(Serialize, Deserialize)]
pub struct LoadCurrentProject {
    pub project_id: ProjectId,
}

impl LoadCurrentProject {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Project, ServiceError> {
        use crate::schema::projects::dsl::projects;

        q!(projects.find(self.project_id))
            .first::<Project>(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceError::RecordNotFound("Project".to_string())
            })
    }
}

impl Message for LoadCurrentProject {
    type Result = Result<Project, ServiceError>;
}

impl Handler<LoadCurrentProject> for DbExecutor {
    type Result = Result<Project, ServiceError>;

    fn handle(&mut self, msg: LoadCurrentProject, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}

pub struct CreateProject {
    pub name: NameString,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<ProjectCategory>,
    pub time_tracking: Option<TimeTracking>,
}

impl CreateProject {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Project, ServiceError> {
        use crate::schema::projects::dsl::*;

        crate::db::Guard::new(conn)?.run(|_guard| {
            let p = q!(diesel::insert_into(projects)
                .values((
                    name.eq(self.name),
                    self.url.map(|v| url.eq(v)),
                    self.description.map(|v| description.eq(v)),
                    self.category.map(|v| category.eq(v)),
                    self.time_tracking.map(|v| time_tracking.eq(v)),
                ))
                .returning(all_columns))
            .get_result::<Project>(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceError::DatabaseQueryFailed(format!("{}", e))
            })?;

            crate::db::issue_statuses::CreateIssueStatus {
                project_id: p.id,
                position: 0,
                name: "TODO".to_string(),
            }
            .execute(conn)?;

            Ok(p)
        })
    }
}

impl Message for CreateProject {
    type Result = Result<Project, ServiceError>;
}

impl Handler<CreateProject> for DbExecutor {
    type Result = Result<Project, ServiceError>;

    fn handle(&mut self, msg: CreateProject, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}

pub struct UpdateProject {
    pub project_id: ProjectId,
    pub name: Option<NameString>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<ProjectCategory>,
    pub time_tracking: Option<TimeTracking>,
}

impl UpdateProject {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Project, ServiceError> {
        use crate::schema::projects::dsl::*;

        q!(diesel::update(projects.find(self.project_id)).set((
            self.name.map(|v| name.eq(v)),
            self.url.map(|v| url.eq(v)),
            self.description.map(|v| description.eq(v)),
            self.category.map(|v| category.eq(v)),
            self.time_tracking.map(|v| time_tracking.eq(v)),
        )))
        .execute(conn)
        .map_err(|e| ServiceError::DatabaseQueryFailed(format!("{}", e)))?;

        LoadCurrentProject {
            project_id: self.project_id,
        }
        .execute(conn)
    }
}

impl Message for UpdateProject {
    type Result = Result<Project, ServiceError>;
}

impl Handler<UpdateProject> for DbExecutor {
    type Result = Result<Project, ServiceError>;

    fn handle(&mut self, msg: UpdateProject, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}

pub struct LoadProjects {
    pub user_id: UserId,
}

impl LoadProjects {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Vec<Project>, ServiceError> {
        use crate::schema::projects::dsl::*;
        use crate::schema::user_projects::dsl::{project_id, user_id, user_projects};

        q!(projects
            .inner_join(user_projects.on(project_id.eq(id)))
            .filter(user_id.eq(self.user_id))
            .distinct_on(id)
            .select(all_columns))
        .load::<Project>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::RecordNotFound("Project".to_string())
        })
    }
}

impl Message for LoadProjects {
    type Result = Result<Vec<Project>, ServiceError>;
}

impl Handler<LoadProjects> for DbExecutor {
    type Result = Result<Vec<Project>, ServiceError>;

    fn handle(&mut self, msg: LoadProjects, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}
