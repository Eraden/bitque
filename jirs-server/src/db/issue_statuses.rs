use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;

use jirs_data::{IssueStatus, IssueStatusId, ProjectId};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

pub struct LoadIssueStatuses {
    pub project_id: ProjectId,
}

impl Message for LoadIssueStatuses {
    type Result = Result<Vec<IssueStatus>, ServiceErrors>;
}

impl Handler<LoadIssueStatuses> for DbExecutor {
    type Result = Result<Vec<IssueStatus>, ServiceErrors>;

    fn handle(&mut self, msg: LoadIssueStatuses, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_statuses::dsl::{id, issue_statuses, project_id};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let issue_assignees_query = issue_statuses
            .distinct_on(id)
            .filter(project_id.eq(msg.project_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&issue_assignees_query));
        issue_assignees_query
            .load::<IssueStatus>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue users".to_string()))
    }
}

pub struct CreateIssueStatus {
    pub project_id: ProjectId,
    pub position: i32,
    pub name: String,
}

impl Message for CreateIssueStatus {
    type Result = Result<IssueStatus, ServiceErrors>;
}

impl Handler<CreateIssueStatus> for DbExecutor {
    type Result = Result<IssueStatus, ServiceErrors>;

    fn handle(&mut self, msg: CreateIssueStatus, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_statuses::dsl::{issue_statuses, name, position, project_id};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let issue_assignees_query = diesel::insert_into(issue_statuses).values((
            project_id.eq(msg.project_id),
            name.eq(msg.name),
            position.eq(msg.position),
        ));
        debug!("{}", diesel::debug_query::<Pg, _>(&issue_assignees_query));
        issue_assignees_query
            .get_result::<IssueStatus>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue users".to_string()))
    }
}

pub struct DeleteIssueStatus {
    pub issue_status_id: IssueStatusId,
}

impl Message for DeleteIssueStatus {
    type Result = Result<usize, ServiceErrors>;
}

impl Handler<DeleteIssueStatus> for DbExecutor {
    type Result = Result<usize, ServiceErrors>;

    fn handle(&mut self, msg: DeleteIssueStatus, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_statuses::dsl::{id, issue_statuses};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let issue_assignees_query =
            diesel::delete(issue_statuses).filter(id.eq(msg.issue_status_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&issue_assignees_query));
        issue_assignees_query
            .execute(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue users".to_string()))
    }
}
