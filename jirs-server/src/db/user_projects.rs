use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;

use jirs_data::{ProjectId, UserId, UserProject, UserProjectId, UserRole};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

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

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let user_query = user_projects.filter(user_id.eq(msg.user_id).and(is_current.eq(true)));
        debug!("{}", diesel::debug_query::<Pg, _>(&user_query));
        user_query
            .first(conn)
            .map_err(|_e| ServiceErrors::RecordNotFound(format!("user project {}", msg.user_id)))
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

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let user_query = user_projects.filter(user_id.eq(msg.user_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&user_query));
        user_query
            .load(conn)
            .map_err(|_e| ServiceErrors::RecordNotFound(format!("user project {}", msg.user_id)))
    }
}

pub struct ChangeCurrentUserProject {
    pub user_id: UserId,
    pub id: UserProjectId,
}

impl Message for ChangeCurrentUserProject {
    type Result = Result<UserProject, ServiceErrors>;
}

impl Handler<ChangeCurrentUserProject> for DbExecutor {
    type Result = Result<UserProject, ServiceErrors>;

    fn handle(&mut self, msg: ChangeCurrentUserProject, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::user_projects::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = user_projects.filter(id.eq(msg.id).and(user_id.eq(msg.user_id)));
        debug!("{}", diesel::debug_query::<Pg, _>(&query));
        let mut user_project: UserProject = query
            .first(conn)
            .map_err(|_e| ServiceErrors::RecordNotFound(format!("user project {}", msg.user_id)))?;

        let query = diesel::update(user_projects)
            .set(is_current.eq(false))
            .filter(user_id.eq(msg.user_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&query));
        query
            .execute(conn)
            .map(|_| ())
            .map_err(|_e| ServiceErrors::RecordNotFound(format!("user project {}", msg.user_id)))?;

        let query = diesel::update(user_projects)
            .set(is_current.eq(true))
            .filter(id.eq(msg.id).and(user_id.eq(msg.user_id)));
        debug!("{}", diesel::debug_query::<Pg, _>(&query));
        query
            .execute(conn)
            .map(|_| ())
            .map_err(|_e| ServiceErrors::RecordNotFound(format!("user project {}", msg.user_id)))?;

        user_project.is_current = true;
        Ok(user_project)
    }
}

pub struct RemoveInvitedUser {
    pub invited_id: UserId,
    pub inviter_id: UserId,
    pub project_id: ProjectId,
}

impl Message for RemoveInvitedUser {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<RemoveInvitedUser> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: RemoveInvitedUser, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::user_projects::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        if msg.invited_id == msg.inviter_id {
            return Err(ServiceErrors::Unauthorized);
        }

        {
            let owner = UserRole::Owner;
            let query = user_projects.filter(
                user_id
                    .eq(msg.inviter_id)
                    .and(project_id.eq(msg.project_id))
                    .and(role.eq(owner)),
            );
            debug!("{}", diesel::debug_query::<Pg, _>(&query));
            query
                .first::<UserProject>(conn)
                .map_err(|_e| ServiceErrors::Unauthorized)?;
        }

        {
            let query = diesel::delete(user_projects).filter(
                user_id
                    .eq(msg.invited_id)
                    .and(project_id.eq(msg.project_id)),
            );
            debug!("{}", diesel::debug_query::<Pg, _>(&query));
            query.execute(conn).map_err(|_e| {
                ServiceErrors::RecordNotFound(format!(
                    "user project user with id {} for project {}",
                    msg.invited_id, msg.project_id
                ))
            })?;
        }

        Ok(())
    }
}
