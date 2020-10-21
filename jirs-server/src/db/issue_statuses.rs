use actix::{Handler, Message};
use diesel::prelude::*;

use jirs_data::{IssueStatus, IssueStatusId, Position, ProjectId, TitleString};

use crate::db::DbPooledConn;
use crate::{db::DbExecutor, db_pool, errors::ServiceErrors, q};

pub struct LoadIssueStatuses {
    pub project_id: ProjectId,
}

impl LoadIssueStatuses {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Vec<IssueStatus>, ServiceErrors> {
        use crate::schema::issue_statuses::dsl::{id, issue_statuses, project_id};

        q!(issue_statuses
            .distinct_on(id)
            .filter(project_id.eq(self.project_id)))
        .load::<IssueStatus>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::RecordNotFound("issue users".to_string())
        })
    }
}

impl Message for LoadIssueStatuses {
    type Result = Result<Vec<IssueStatus>, ServiceErrors>;
}

impl Handler<LoadIssueStatuses> for DbExecutor {
    type Result = Result<Vec<IssueStatus>, ServiceErrors>;

    fn handle(&mut self, msg: LoadIssueStatuses, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}

pub struct CreateIssueStatus {
    pub project_id: ProjectId,
    pub position: i32,
    pub name: TitleString,
}

impl CreateIssueStatus {
    pub fn execute(self, conn: &DbPooledConn) -> Result<IssueStatus, ServiceErrors> {
        use crate::schema::issue_statuses::dsl::{issue_statuses, name, position, project_id};
        q!(diesel::insert_into(issue_statuses).values((
            project_id.eq(self.project_id),
            name.eq(self.name),
            position.eq(self.position),
        )))
        .get_result::<IssueStatus>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::RecordNotFound("issue users".to_string())
        })
    }
}

impl Message for CreateIssueStatus {
    type Result = Result<IssueStatus, ServiceErrors>;
}

impl Handler<CreateIssueStatus> for DbExecutor {
    type Result = Result<IssueStatus, ServiceErrors>;

    fn handle(&mut self, msg: CreateIssueStatus, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}

pub struct DeleteIssueStatus {
    pub project_id: ProjectId,
    pub issue_status_id: IssueStatusId,
}

impl Message for DeleteIssueStatus {
    type Result = Result<IssueStatusId, ServiceErrors>;
}

impl Handler<DeleteIssueStatus> for DbExecutor {
    type Result = Result<IssueStatusId, ServiceErrors>;

    fn handle(&mut self, msg: DeleteIssueStatus, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_statuses::dsl::{id, issue_statuses, project_id};

        let conn = db_pool!(self);

        q!(diesel::delete(issue_statuses)
            .filter(id.eq(msg.issue_status_id))
            .filter(project_id.eq(msg.project_id)))
        .execute(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::RecordNotFound("issue users".to_string())
        })?;
        Ok(msg.issue_status_id)
    }
}

pub struct UpdateIssueStatus {
    pub issue_status_id: IssueStatusId,
    pub project_id: ProjectId,
    pub position: Position,
    pub name: TitleString,
}

impl Message for UpdateIssueStatus {
    type Result = Result<IssueStatus, ServiceErrors>;
}

impl Handler<UpdateIssueStatus> for DbExecutor {
    type Result = Result<IssueStatus, ServiceErrors>;

    fn handle(&mut self, msg: UpdateIssueStatus, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_statuses::dsl::{
            id, issue_statuses, name, position, project_id, updated_at,
        };

        let conn = db_pool!(self);

        q!(diesel::update(issue_statuses)
            .set((
                name.eq(msg.name),
                position.eq(msg.position),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .filter(id.eq(msg.issue_status_id))
            .filter(project_id.eq(msg.project_id)))
        .get_result::<IssueStatus>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::RecordNotFound("issue users".to_string())
        })
    }
}
