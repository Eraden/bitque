use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::Issue;
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoadProjectIssues {
    pub project_id: i32,
}

impl Message for LoadProjectIssues {
    type Result = Result<Vec<Issue>, ServiceErrors>;
}

impl Handler<LoadProjectIssues> for DbExecutor {
    type Result = Result<Vec<Issue>, ServiceErrors>;

    fn handle(&mut self, msg: LoadProjectIssues, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issues::dsl::{issues, project_id};
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let vec = issues
            .filter(project_id.eq(msg.project_id))
            .distinct()
            .load::<Issue>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project issues".to_string()))?;
        Ok(vec)
    }
}
