use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::{ProjectId, User, UserId};

use crate::db::projects::CreateProject;
use crate::db::{DbExecutor, DbPooledConn};
use crate::errors::ServiceErrors;
use crate::schema::users::all_columns;

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

        let users_query = users
            .distinct_on(id)
            .filter(email.eq(msg.email.as_str()))
            .filter(name.eq(msg.name.as_str()));
        debug!("{}", diesel::debug_query::<Pg, _>(&users_query));
        users_query
            .first(conn)
            .map_err(|_| ServiceErrors::RecordNotFound(format!("user {} {}", msg.name, msg.email)))
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
        use crate::schema::user_projects::dsl::{project_id, user_id, user_projects};
        use crate::schema::users::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let users_query = users
            .distinct_on(id)
            .inner_join(user_projects.on(user_id.eq(id)))
            .filter(project_id.eq(msg.project_id))
            .select(all_columns);
        debug!("{}", diesel::debug_query::<Pg, _>(&users_query));
        users_query
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project users".to_string()))
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

        let users_query = users
            .distinct_on(id)
            .inner_join(issue_assignees.on(user_id.eq(id)))
            .filter(issue_id.eq(msg.issue_id))
            .select(users::all_columns());
        debug!("{}", diesel::debug_query::<Pg, _>(&users_query));
        users_query
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue users".to_string()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub project_id: Option<ProjectId>,
}

impl Message for Register {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<Register> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: Register, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let matching = count_matching_users(msg.name.as_str(), msg.email.as_str(), conn);

        if matching > 0 {
            return Err(ServiceErrors::RegisterCollision);
        }

        let current_project_id: ProjectId = match msg.project_id.as_ref().cloned() {
            Some(current_project_id) => current_project_id,
            _ => {
                self.handle(
                    CreateProject {
                        name: "initial".to_string(),
                        url: None,
                        description: None,
                        category: None,
                        time_tracking: None,
                    },
                    ctx,
                )?
                .id
            }
        };

        let user: User = {
            let insert_user_query =
                diesel::insert_into(users).values((name.eq(msg.name), email.eq(msg.email)));
            debug!("{}", diesel::debug_query::<Pg, _>(&insert_user_query));
            insert_user_query
                .get_result(conn)
                .map_err(|_| ServiceErrors::RegisterCollision)?
        };

        {
            use crate::schema::user_projects::dsl::*;
            let insert_user_project_query = diesel::insert_into(user_projects).values((
                user_id.eq(user.id),
                project_id.eq(current_project_id),
                is_current.eq(true),
                is_default.eq(true),
            ));
            debug!(
                "{}",
                diesel::debug_query::<Pg, _>(&insert_user_project_query)
            );
            insert_user_project_query
                .execute(conn)
                .map_err(|_| ServiceErrors::RegisterCollision)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadInvitedUsers {
    pub user_id: UserId,
}

impl Message for LoadInvitedUsers {
    type Result = Result<Vec<User>, ServiceErrors>;
}

impl Handler<LoadInvitedUsers> for DbExecutor {
    type Result = Result<Vec<User>, ServiceErrors>;

    fn handle(&mut self, msg: LoadInvitedUsers, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::{email as i_email, invitations, invited_by_id};
        use crate::schema::users::dsl::{email as u_email, users};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = users
            .inner_join(invitations.on(i_email.eq(u_email)))
            .filter(invited_by_id.eq(msg.user_id))
            .select(users::all_columns());
        debug!("{}", diesel::debug_query::<Pg, _>(&query));
        query
            .load(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))
    }
}

fn count_matching_users(name: &str, email: &str, conn: &DbPooledConn) -> i64 {
    use crate::schema::users::dsl;

    let query = dsl::users
        .filter(dsl::email.eq(email).and(dsl::name.ne(name)))
        .or_filter(dsl::email.ne(email).and(dsl::name.eq(name)))
        .or_filter(dsl::email.eq(email).and(dsl::name.eq(name)))
        .count();
    info!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
    query.get_result::<i64>(conn).unwrap_or(1)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAvatarUrl {
    pub user_id: UserId,
    pub avatar_url: Option<String>,
}

impl Message for UpdateAvatarUrl {
    type Result = Result<User, ServiceErrors>;
}

impl Handler<UpdateAvatarUrl> for DbExecutor {
    type Result = Result<User, ServiceErrors>;

    fn handle(&mut self, msg: UpdateAvatarUrl, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::{avatar_url, id, users};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let update_query = diesel::update(users)
            .set(avatar_url.eq(msg.avatar_url))
            .filter(id.eq(msg.user_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&update_query));
        update_query
            .execute(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;

        let user_query = users.find(msg.user_id);
        debug!("{}", diesel::debug_query::<Pg, _>(&user_query));
        user_query
            .first(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileUpdate {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
}

impl Message for ProfileUpdate {
    type Result = Result<User, ServiceErrors>;
}

impl Handler<ProfileUpdate> for DbExecutor {
    type Result = Result<User, ServiceErrors>;

    fn handle(&mut self, msg: ProfileUpdate, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::{email, id, name, users};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let update_query = diesel::update(users)
            .set((email.eq(msg.email), name.eq(msg.name)))
            .filter(id.eq(msg.user_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&update_query));
        update_query
            .execute(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;

        let user_query = users.find(msg.user_id);
        debug!("{}", diesel::debug_query::<Pg, _>(&user_query));
        user_query
            .first(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))
    }
}

#[cfg(test)]
mod tests {
    use crate::db::build_pool;
    use crate::models::CreateProjectForm;

    use super::*;

    #[test]
    fn check_collision() {
        use crate::schema::projects::dsl::projects;
        use crate::schema::user_projects::dsl::user_projects;
        use crate::schema::users::dsl::users;

        let pool = build_pool();
        let conn = &pool.get().unwrap();

        diesel::delete(user_projects).execute(conn).unwrap();
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

        let user: User = {
            use crate::schema::users::dsl::*;
            diesel::insert_into(users)
                .values((
                    name.eq("Foo".to_string()),
                    email.eq("foo@example.com".to_string()),
                ))
                .get_result(conn)
                .unwrap()
        };
        {
            use crate::schema::user_projects::dsl::*;
            diesel::insert_into(user_projects)
                .values((
                    user_id.eq(user.id),
                    project_id.eq(project.id),
                    is_current.eq(true),
                    is_default.eq(true),
                ))
                .execute(conn)
                .unwrap();
        }

        assert_eq!(count_matching_users("Foo", "bar@example.com", conn), 1);
        assert_eq!(count_matching_users("Bar", "foo@example.com", conn), 1);
        assert_eq!(count_matching_users("Foo", "foo@example.com", conn), 1);
    }
}
