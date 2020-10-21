use actix::{Handler, Message};
use diesel::prelude::*;

use jirs_data::msg::WsError;
use jirs_data::{ProjectId, UserId, UserProject, UserProjectId, UserRole};

use crate::{
    db::{DbExecutor, DbPooledConn},
    db_pool,
    errors::ServiceErrors,
    q,
};

pub struct CurrentUserProject {
    pub user_id: UserId,
}

impl Message for CurrentUserProject {
    type Result = Result<UserProject, ServiceErrors>;
}

impl Handler<CurrentUserProject> for DbExecutor {
    type Result = Result<UserProject, ServiceErrors>;

    fn handle(&mut self, msg: CurrentUserProject, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user_projects::dsl::*;

        let conn = db_pool!(self);

        q!(user_projects.filter(user_id.eq(msg.user_id).and(is_current.eq(true))))
            .first(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::RecordNotFound(format!("user project {}", msg.user_id))
            })
    }
}

pub struct LoadUserProjects {
    pub user_id: UserId,
}

impl Message for LoadUserProjects {
    type Result = Result<Vec<UserProject>, ServiceErrors>;
}

impl Handler<LoadUserProjects> for DbExecutor {
    type Result = Result<Vec<UserProject>, ServiceErrors>;

    fn handle(&mut self, msg: LoadUserProjects, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::user_projects::dsl::*;

        let conn = db_pool!(self);

        q!(user_projects.filter(user_id.eq(msg.user_id)))
            .load(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::RecordNotFound(format!("user project {}", msg.user_id))
            })
    }
}

pub struct ChangeCurrentUserProject {
    pub user_id: UserId,
    pub id: UserProjectId,
}

impl ChangeCurrentUserProject {
    pub fn execute(self, conn: &DbPooledConn) -> Result<UserProject, ServiceErrors> {
        use crate::schema::user_projects::dsl::*;

        crate::db::Guard::new(conn)?.run(|_guard| {
            let mut user_project: UserProject =
                q!(user_projects.filter(id.eq(self.id).and(user_id.eq(self.user_id))))
                    .first(conn)
                    .map_err(|e| {
                        error!("{:?}", e);
                        ServiceErrors::RecordNotFound(format!("user project {}", self.user_id))
                    })?;

            q!(diesel::update(user_projects)
                .set(is_current.eq(false))
                .filter(user_id.eq(self.user_id)))
            .execute(conn)
            .map(|_| ())
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::DatabaseQueryFailed(format!(
                    "setting current flag to false while updating current project {}",
                    self.user_id
                ))
            })?;

            q!(diesel::update(user_projects)
                .set(is_current.eq(true))
                .filter(id.eq(self.id).and(user_id.eq(self.user_id))))
            .execute(conn)
            .map(|_| ())
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::DatabaseQueryFailed(format!(
                    "set current flag on project while updating current project {}",
                    self.user_id
                ))
            })?;

            user_project.is_current = true;
            Ok(user_project)
        })
    }
}

impl Message for ChangeCurrentUserProject {
    type Result = Result<UserProject, ServiceErrors>;
}

impl Handler<ChangeCurrentUserProject> for DbExecutor {
    type Result = Result<UserProject, ServiceErrors>;

    fn handle(&mut self, msg: ChangeCurrentUserProject, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct RemoveInvitedUser {
    pub invited_id: UserId,
    pub inviter_id: UserId,
    pub project_id: ProjectId,
}

impl RemoveInvitedUser {
    pub fn execute(self, conn: &DbPooledConn) -> Result<usize, ServiceErrors> {
        use crate::schema::user_projects::dsl::*;

        if self.invited_id == self.inviter_id {
            return Err(ServiceErrors::Unauthorized);
        }

        q!(user_projects.filter(
            user_id
                .eq(self.inviter_id)
                .and(project_id.eq(self.project_id))
                .and(role.eq(UserRole::Owner)),
        ))
        .first::<UserProject>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::Unauthorized
        })?;

        q!(diesel::delete(user_projects).filter(
            user_id
                .eq(self.invited_id)
                .and(project_id.eq(self.project_id)),
        ))
        .execute(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::RecordNotFound(format!(
                "user project user with id {} for project {}",
                self.invited_id, self.project_id
            ))
        })
    }
}

impl Message for RemoveInvitedUser {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<RemoveInvitedUser> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: RemoveInvitedUser, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)?;
        Ok(())
    }
}

pub struct CreateUserProject {
    pub user_id: UserId,
    pub project_id: ProjectId,
    pub is_current: bool,
    pub is_default: bool,
    pub role: UserRole,
}

impl CreateUserProject {
    pub fn execute(self, conn: &DbPooledConn) -> Result<usize, ServiceErrors> {
        use crate::schema::user_projects::dsl::*;
        q!(diesel::insert_into(user_projects).values((
            user_id.eq(self.user_id),
            project_id.eq(self.project_id),
            is_current.eq(self.is_current),
            is_default.eq(self.is_default),
            role.eq(self.role),
        )))
        .execute(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::Error(WsError::InvalidUserProject)
        })
    }
}

impl Message for CreateUserProject {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<CreateUserProject> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: CreateUserProject, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)?;
        Ok(())
    }
}
