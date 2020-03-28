use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::{IssueAssignee, User};
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoadProjectUsers {
    pub project_id: i32,
}

impl Message for LoadProjectUsers {
    type Result = Result<Vec<User>, ServiceErrors>;
}

impl Handler<LoadProjectUsers> for DbExecutor {
    type Result = Result<Vec<User>, ServiceErrors>;

    fn handle(&mut self, msg: LoadProjectUsers, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let rows: Vec<User> = users
            .distinct_on(id)
            .filter(project_id.eq(msg.project_id))
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project users".to_string()))?;
        Ok(rows)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoadIssueAssignees {
    pub issue_id: i32,
}

impl Message for LoadIssueAssignees {
    type Result = Result<Vec<User>, ServiceErrors>;
}

impl Handler<LoadIssueAssignees> for DbExecutor {
    type Result = Result<Vec<User>, ServiceErrors>;

    fn handle(&mut self, msg: LoadIssueAssignees, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_assignees::dsl::{issue_assignees, issue_id, user_id};
        use crate::schema::users::dsl::*;

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let rows: Vec<(User, IssueAssignee)> = users
            .distinct_on(id)
            .inner_join(issue_assignees.on(user_id.eq(id)))
            .filter(issue_id.eq(msg.issue_id))
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue users".to_string()))?;
        let mut vec: Vec<User> = vec![];
        for row in rows {
            vec.push(row.0);
        }
        Ok(vec)
    }
}
