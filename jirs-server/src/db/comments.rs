use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::Comment;
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoadIssueComments {
    pub issue_id: i32,
}

impl Message for LoadIssueComments {
    type Result = Result<Vec<Comment>, ServiceErrors>;
}

impl Handler<LoadIssueComments> for DbExecutor {
    type Result = Result<Vec<Comment>, ServiceErrors>;

    fn handle(&mut self, msg: LoadIssueComments, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let rows: Vec<Comment> = comments
            .distinct_on(id)
            .filter(issue_id.eq(msg.issue_id))
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))?;
        Ok(rows)
    }
}
