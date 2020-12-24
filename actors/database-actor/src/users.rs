use {
    crate::{
        db_create, db_create_with_conn, db_find, db_load, db_update, projects::CreateProject, q,
        user_projects::CreateUserProject, DbPooledConn,
    },
    diesel::prelude::*,
    jirs_data::{EmailString, IssueId, ProjectId, User, UserId, UserRole, UsernameString},
};

db_find! {
    FindUser,
    msg => users => users.find(msg.user_id),
    User,
    user_id => UserId
}

db_find! {
    LookupUser,
    msg => users => users
            .distinct_on(id)
            .filter(email.eq(msg.email.as_str()))
            .filter(name.eq(msg.name.as_str())),
    User,
    name => UsernameString,
    email => EmailString
}

db_load! {
    LoadProjectUsers,
    msg => users => {
        use crate::schema::user_projects::dsl::{project_id, user_id, user_projects};
        use crate::schema::users::all_columns;

        users
            .distinct_on(id)
            .inner_join(user_projects.on(user_id.eq(id)))
            .filter(project_id.eq(msg.project_id))
            .select(all_columns)
    },
    User,
    project_id => ProjectId
}

db_load! {
    LoadIssueAssignees,
    msg => users => {
        use crate::schema::issue_assignees::dsl::{issue_assignees, issue_id, user_id};
        users
            .distinct_on(id)
            .inner_join(issue_assignees.on(user_id.eq(id)))
            .filter(issue_id.eq(msg.issue_id))
            .select(users::all_columns())
    },
    User,
    issue_id => IssueId
}

db_create! {
    CreateUser,
    msg => users => diesel::insert_into(users)
        .values((name.eq(msg.name.as_str()), email.eq(msg.email.as_str()))),
    User,
    name => UsernameString,
    email => EmailString
}

/*impl CreateUser {
    pub fn execute(self, conn: &DbPooledConn) -> Result<User, crate::DatabaseError> {
        use crate::schema::users::dsl::*;

        q!(diesel::insert_into(users)
            .values((name.eq(self.name.as_str()), email.eq(self.email.as_str()))))
        .get_result(conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            let ws = match e {
                Error::InvalidCString(_) => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                    crate::DatabaseError::User(UserError::TakenPair(self.name, self.email))
                }
                Error::DatabaseError(_, _) => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::NotFound => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::QueryBuilderError(_) => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::DeserializationError(_) => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::SerializationError(_) => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::RollbackTransaction => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::AlreadyInTransaction => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
                Error::__Nonexhaustive => {
                    crate::DatabaseError::User(UserError::InvalidPair(self.name, self.email))
                }
            };
            crate::DatabaseError::Error(ws)
        })
    }
}*/

db_create_with_conn! {
    Register,
    msg => conn => users => {
        if count_matching_users(msg.name.as_str(), msg.email.as_str(), conn) > 0 {
            return Err(crate::DatabaseError::User(crate::UserError::InvalidPair(msg.name, msg.email)));
        }

        let current_project_id: ProjectId = match msg.project_id {
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
            name: msg.name,
            email: msg.email,
        }
        .execute(conn)?;

        CreateUserProject {
            user_id: user.id,
            project_id: current_project_id,
            is_current: true,
            is_default: true,
            role: msg.role,
        }
        .execute(conn)?;
        users.find(user.id)
    },
    User,
    name => UsernameString,
    email => EmailString,
    project_id => Option<ProjectId>,
    role => UserRole
}

db_load! {
    LoadInvitedUsers,
    msg => users => {
        use crate::schema::invitations::dsl::{email as i_email, invitations, invited_by_id};
        users
            .inner_join(invitations.on(i_email.eq(email)))
            .filter(invited_by_id.eq(msg.user_id))
            .select(users::all_columns())
    },
    User,
    user_id => UserId
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

db_update! {
    UpdateAvatarUrl,
    msg => users => diesel::update(users.find(msg.user_id))
            .set(avatar_url.eq(msg.avatar_url)),
    User,
    user_id => UserId,
    avatar_url => Option<String>
}

db_update! {
    ProfileUpdate,
    msg => users => diesel::update(users.find(msg.user_id))
        .set((email.eq(msg.email), name.eq(msg.name))),
    User,
    user_id => UserId,
    name => String,
    email => String
}

#[cfg(test)]
mod tests {
    use diesel::connection::TransactionManager;

    use jirs_data::{Project, ProjectCategory};

    use crate::build_pool;

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
