use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::{IssueAssignee, User, UserForm};

#[derive(Serialize, Deserialize, Debug)]
pub struct FindUser {
    pub name: String,
    pub email: String,
}

impl Message for FindUser {
    type Result = Result<User, ServiceErrors>;
}

impl Handler<FindUser> for DbExecutor {
    type Result = Result<User, ServiceErrors>;

    fn handle(&mut self, msg: FindUser, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let row: User = users
            .distinct_on(id)
            .filter(email.eq(msg.email.as_str()))
            .filter(name.eq(msg.name.as_str()))
            .first(conn)
            .map_err(|_| {
                ServiceErrors::RecordNotFound(format!("user {} {}", msg.name, msg.email))
            })?;
        Ok(row)
    }
}

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
            .pool
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
            .pool
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub email: String,
}

impl Message for Register {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<Register> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = users
            .filter(
                email
                    .eq(msg.email.as_str())
                    .and(name.ne(msg.name.as_str()))
                    .or(email.ne(msg.email.as_str()).and(name.eq(msg.name.as_str())))
                    .or(email.eq(msg.email.as_str()).and(name.eq(msg.name.as_str()))),
            )
            .count();

        info!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );

        let matching: i64 = query.get_result(conn).unwrap_or(1);

        if matching > 0 {
            return Err(ServiceErrors::RegisterCollision);
        }

        let form = UserForm {
            name: msg.name,
            email: msg.email,
            avatar_url: None,
            project_id: None,
        };

        match diesel::insert_into(users).values(form).execute(conn) {
            Ok(_) => (),
            _ => return Err(ServiceErrors::RegisterCollision),
        };

        Ok(())
    }
}
