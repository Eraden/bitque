use actix::{Handler, Message};
use diesel::prelude::*;
use diesel::result::Error;

use jirs_data::{msg::WsError, EmailString, ProjectId, User, UserId, UserRole, UsernameString};

use crate::db::user_projects::CreateUserProject;
use crate::{
    db::{projects::CreateProject, DbExecutor, DbPooledConn},
    db_pool,
    errors::ServiceError,
    q,
    schema::users::all_columns,
};

#[derive(Debug)]
pub struct FindUser {
    pub user_id: UserId,
}

impl FindUser {
    pub fn execute(self, conn: &DbPooledConn) -> Result<User, ServiceError> {
        use crate::schema::users::dsl::*;

        q!(users.find(self.user_id)).first(conn).map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::UserNotExists(self.user_id))
        })
    }
}

impl Message for FindUser {
    type Result = Result<User, ServiceError>;
}

impl Handler<FindUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: FindUser, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct LookupUser {
    pub name: String,
    pub email: String,
}

impl LookupUser {
    pub fn execute(self, conn: &DbPooledConn) -> Result<User, ServiceError> {
        use crate::schema::users::dsl::*;

        q!(users
            .distinct_on(id)
            .filter(email.eq(self.email.as_str()))
            .filter(name.eq(self.name.as_str())))
        .first(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::NoMatchingPair(self.name, self.email))
        })
    }
}

impl Message for LookupUser {
    type Result = Result<User, ServiceError>;
}

impl Handler<LookupUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: LookupUser, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct LoadProjectUsers {
    pub project_id: i32,
}

impl LoadProjectUsers {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Vec<User>, ServiceError> {
        use crate::schema::user_projects::dsl::{project_id, user_id, user_projects};
        use crate::schema::users::dsl::*;

        q!(users
            .distinct_on(id)
            .inner_join(user_projects.on(user_id.eq(id)))
            .filter(project_id.eq(self.project_id))
            .select(all_columns))
        .load(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::FailedToLoadProjectUsers)
        })
    }
}

impl Message for LoadProjectUsers {
    type Result = Result<Vec<User>, ServiceError>;
}

impl Handler<LoadProjectUsers> for DbExecutor {
    type Result = Result<Vec<User>, ServiceError>;

    fn handle(&mut self, msg: LoadProjectUsers, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct LoadIssueAssignees {
    pub issue_id: i32,
}

impl LoadIssueAssignees {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Vec<User>, ServiceError> {
        use crate::schema::issue_assignees::dsl::{issue_assignees, issue_id, user_id};
        use crate::schema::users::dsl::*;

        q!(users
            .distinct_on(id)
            .inner_join(issue_assignees.on(user_id.eq(id)))
            .filter(issue_id.eq(self.issue_id))
            .select(users::all_columns()))
        .load(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::FailedToLoadAssignees)
        })
    }
}

impl Message for LoadIssueAssignees {
    type Result = Result<Vec<User>, ServiceError>;
}

impl Handler<LoadIssueAssignees> for DbExecutor {
    type Result = Result<Vec<User>, ServiceError>;

    fn handle(&mut self, msg: LoadIssueAssignees, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct CreateUser {
    pub name: UsernameString,
    pub email: EmailString,
}

impl CreateUser {
    pub fn execute(self, conn: &DbPooledConn) -> Result<User, ServiceError> {
        use crate::schema::users::dsl::*;

        q!(diesel::insert_into(users)
            .values((name.eq(self.name.as_str()), email.eq(self.email.as_str()))))
        .get_result(conn)
        .map_err(|e| {
            error!("{:?}", e);
            let ws = match e {
                Error::InvalidCString(_) => WsError::InvalidPair(self.name, self.email),
                Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                    WsError::TakenPair(self.name, self.email)
                }
                Error::DatabaseError(_, _) => WsError::InvalidPair(self.name, self.email),
                Error::NotFound => WsError::InvalidPair(self.name, self.email),
                Error::QueryBuilderError(_) => WsError::InvalidPair(self.name, self.email),
                Error::DeserializationError(_) => WsError::InvalidPair(self.name, self.email),
                Error::SerializationError(_) => WsError::InvalidPair(self.name, self.email),
                Error::RollbackTransaction => WsError::InvalidPair(self.name, self.email),
                Error::AlreadyInTransaction => WsError::InvalidPair(self.name, self.email),
                Error::__Nonexhaustive => WsError::InvalidPair(self.name, self.email),
            };
            ServiceError::Error(ws)
        })
    }
}

impl Message for CreateUser {
    type Result = Result<User, ServiceError>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct Register {
    pub name: UsernameString,
    pub email: EmailString,
    pub project_id: Option<ProjectId>,
    pub role: UserRole,
}

impl Register {
    pub fn execute(self, conn: &DbPooledConn) -> Result<(), ServiceError> {
        let Register {
            name: given_name,
            email: given_email,
            project_id: given_project_id,
            role: given_role,
        } = self;

        crate::db::Guard::new(conn)?.run(|_guard| {
            if count_matching_users(given_name.as_str(), given_email.as_str(), conn) > 0 {
                return Err(ServiceError::Error(WsError::InvalidLoginPair));
            }

            let current_project_id: ProjectId = match given_project_id {
                Some(current_project_id) => current_project_id,
                _ => {
                    CreateProject {
                        name: "initial".to_string(),
                        url: None,
                        description: None,
                        category: None,
                        time_tracking: None,
                    }
                    .execute(conn)?
                    .id
                }
            };

            let user: User = CreateUser {
                name: given_name,
                email: given_email,
            }
            .execute(conn)?;

            CreateUserProject {
                user_id: user.id,
                project_id: current_project_id,
                is_current: true,
                is_default: true,
                role: given_role,
            }
            .execute(conn)?;
            Ok(())
        })
    }
}

impl Message for Register {
    type Result = Result<(), ServiceError>;
}

impl Handler<Register> for DbExecutor {
    type Result = Result<(), ServiceError>;

    fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct LoadInvitedUsers {
    pub user_id: UserId,
}

impl Message for LoadInvitedUsers {
    type Result = Result<Vec<User>, ServiceError>;
}

impl Handler<LoadInvitedUsers> for DbExecutor {
    type Result = Result<Vec<User>, ServiceError>;

    fn handle(&mut self, msg: LoadInvitedUsers, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::{email as i_email, invitations, invited_by_id};
        use crate::schema::users::dsl::{email as u_email, users};

        let conn = db_pool!(self);

        q!(users
            .inner_join(invitations.on(i_email.eq(u_email)))
            .filter(invited_by_id.eq(msg.user_id))
            .select(users::all_columns()))
        .load(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::FailedToLoadInvitedUsers)
        })
    }
}

fn count_matching_users(name: &str, email: &str, conn: &DbPooledConn) -> i64 {
    use crate::schema::users::dsl;

    q!(dsl::users
        .filter(dsl::email.eq(email).and(dsl::name.ne(name)))
        .or_filter(dsl::email.ne(email).and(dsl::name.eq(name)))
        .or_filter(dsl::email.eq(email).and(dsl::name.eq(name)))
        .count())
    .get_result::<i64>(conn)
    .unwrap_or(1)
}

pub struct UpdateAvatarUrl {
    pub user_id: UserId,
    pub avatar_url: Option<String>,
}

impl Message for UpdateAvatarUrl {
    type Result = Result<User, ServiceError>;
}

impl Handler<UpdateAvatarUrl> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: UpdateAvatarUrl, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::{avatar_url, id, users};

        let conn = db_pool!(self);

        q!(diesel::update(users)
            .set(avatar_url.eq(msg.avatar_url))
            .filter(id.eq(msg.user_id)))
        .execute(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::FailedToChangeAvatar)
        })?;

        FindUser {
            user_id: msg.user_id,
        }
        .execute(conn)
    }
}

pub struct ProfileUpdate {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
}

impl Message for ProfileUpdate {
    type Result = Result<User, ServiceError>;
}

impl Handler<ProfileUpdate> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: ProfileUpdate, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::{email, id, name, users};

        let conn = db_pool!(self);

        q!(diesel::update(users)
            .set((email.eq(msg.email), name.eq(msg.name)))
            .filter(id.eq(msg.user_id)))
        .execute(conn)
        .map_err(|e| ServiceError::DatabaseQueryFailed(format!("{}", e)))?;

        q!(users.find(msg.user_id))
            .first(conn)
            .map_err(|e| ServiceError::DatabaseQueryFailed(format!("{}", e)))
    }
}

#[cfg(test)]
mod tests {
    use diesel::connection::TransactionManager;

    use jirs_data::{Project, ProjectCategory};

    use crate::db::build_pool;

    use super::*;

    #[test]
    fn check_collision() {
        use crate::schema::projects::dsl::projects;
        use crate::schema::user_projects::dsl::user_projects;
        use crate::schema::users::dsl::users;

        let pool = build_pool();
        let conn = &pool.get().unwrap();

        let tm = conn.transaction_manager();

        tm.begin_transaction(conn).unwrap();

        diesel::delete(user_projects).execute(conn).unwrap();
        diesel::delete(users).execute(conn).unwrap();
        diesel::delete(projects).execute(conn).unwrap();

        let project: Project = {
            use crate::schema::projects::dsl::*;

            diesel::insert_into(projects)
                .values((
                    name.eq("baz".to_string()),
                    url.eq("/uz".to_string()),
                    description.eq("None".to_string()),
                    category.eq(ProjectCategory::Software),
                ))
                .get_result::<Project>(conn)
                .unwrap()
        };

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

        let res1 = count_matching_users("Foo", "bar@example.com", conn);
        let res2 = count_matching_users("Bar", "foo@example.com", conn);
        let res3 = count_matching_users("Foo", "foo@example.com", conn);

        tm.rollback_transaction(conn).unwrap();

        assert_eq!(res1, 1);
        assert_eq!(res2, 1);
        assert_eq!(res3, 1);
    }
}
