use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::IssueAssignee;

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

#[derive(Serialize, Deserialize)]
pub struct LoadAssignees {
    pub issue_id: i32,
}

impl Message for LoadAssignees {
    type Result = Result<Vec<IssueAssignee>, ServiceErrors>;
}

impl Handler<LoadAssignees> for DbExecutor {
    type Result = Result<Vec<IssueAssignee>, ServiceErrors>;

    fn handle(&mut self, msg: LoadAssignees, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_assignees::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        issue_assignees
            .distinct_on(id)
            .filter(issue_id.eq(msg.issue_id))
            .load::<IssueAssignee>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue users".to_string()))
    }
}
