use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::{IssueAssignee, Project, User};

use crate::db::{DbExecutor, DbPooledConn};
use crate::errors::ServiceErrors;
use crate::models::{CreateProjectForm, UserForm};

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
        use crate::schema::projects::dsl::projects;
        use crate::schema::users::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let matching = count_matching_users(msg.name.as_str(), msg.email.as_str(), conn);

        if matching > 0 {
            return Err(ServiceErrors::RegisterCollision);
        }

        let project: Project = match projects.first(conn) {
            Ok(project) => project,
            _ => {
                let form = CreateProjectForm {
                    name: "initial".to_string(),
                    url: "".to_string(),
                    description: "".to_string(),
                    category: Default::default(),
                };
                diesel::insert_into(projects)
                    .values(form)
                    .get_result(conn)
                    .map_err(|_| ServiceErrors::RegisterCollision)?
            }
        };

        let form = UserForm {
            name: msg.name,
            email: msg.email,
            avatar_url: None,
            project_id: project.id,
        };

        match diesel::insert_into(users).values(form).execute(conn) {
            Ok(_) => (),
            _ => return Err(ServiceErrors::RegisterCollision),
        };

        Ok(())
    }
}

fn count_matching_users(name: &str, email: &str, conn: &DbPooledConn) -> i64 {
    use crate::schema::users::dsl;

    let query = dsl::users
        .filter(dsl::email.eq(email).and(dsl::name.ne(name)))
        .or_filter(dsl::email.ne(email).and(dsl::name.eq(name)))
        .or_filter(dsl::email.eq(email).and(dsl::name.eq(name)))
        .count();

    info!(
        "{}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    query.get_result::<i64>(conn).unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use crate::db::build_pool;
    use crate::models::CreateProjectForm;

    use super::*;

    #[test]
    fn check_collision() {
        use crate::schema::projects::dsl::projects;
        use crate::schema::users::dsl::users;

        let pool = build_pool();
        let conn = &pool.get().unwrap();

        diesel::delete(users).execute(conn).unwrap();
        diesel::delete(projects).execute(conn).unwrap();

        let project_form = CreateProjectForm {
            name: "baz".to_string(),
            url: "/uz".to_string(),
            description: "None".to_string(),
            category: Default::default(),
        };
        let project: Project = diesel::insert_into(projects)
            .values(project_form)
            .get_result(conn)
            .unwrap();

        let user_form = UserForm {
            name: "Foo".to_string(),
            email: "foo@example.com".to_string(),
            avatar_url: None,
            project_id: project.id,
        };
        diesel::insert_into(users)
            .values(user_form)
            .execute(conn)
            .unwrap();

        assert_eq!(count_matching_users("Foo", "bar@example.com", conn), 1);
        assert_eq!(count_matching_users("Bar", "foo@example.com", conn), 1);
        assert_eq!(count_matching_users("Foo", "foo@example.com", conn), 1);
    }
}
