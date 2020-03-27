use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::{Issue, User};
use actix::{Handler, Message};
use chrono::NaiveDateTime;
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
        use crate::schema::issues::dsl::{issues, project_id, reporter_id};
        use crate::schema::users::dsl::*;

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let rows: Vec<(User, Option<Issue>)> = users
            .distinct()
            .left_outer_join(issues.on(reporter_id.eq(id)))
            .filter(project_id.eq(msg.project_id))
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project issues".to_string()))?;
        let mut vec: Vec<User> = vec![];
        for row in rows {
            vec.push(row.0);
        }
        Ok(vec)
    }
}
