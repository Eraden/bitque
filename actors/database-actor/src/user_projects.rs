use {
  crate::{db_create, db_delete_with_conn, db_find, db_load, db_update_with_conn},
  diesel::prelude::*,
  jirs_data::{ProjectId, UserId, UserProject, UserProjectId, UserRole},
};

db_find! {
    CurrentUserProject,
    msg => user_projects => user_projects.filter(user_id.eq(msg.user_id).and(is_current.eq(true))),
    UserProject,
    user_id => UserId
}

db_find! {
    FindUserProject,
    msg => user_projects => user_projects.filter(id.eq(msg.id).and(user_id.eq(msg.user_id))),
    UserProject,
    id => UserProjectId,
    user_id => UserId
}

db_load! {
    LoadUserProjects,
    msg => user_projects => user_projects.filter(user_id.eq(msg.user_id)),
    UserProject,
    user_id => UserId
}

mod inner {
  use {
    crate::db_update,
    diesel::prelude::*,
    jirs_data::{UserId, UserProject, UserProjectId},
  };

  db_update! {
        ChangeProjectIsCurrent,
        msg => user_projects => {
            match msg.id {
                Some(v) => diesel::update(user_projects.filter(user_id.eq(msg.user_id).and(id.eq(v)))).set(is_current.eq(msg.is_current)).into_boxed(),
                _ => diesel::update(user_projects.filter(user_id.eq(msg.user_id))).set(is_current.eq(msg.is_current)).into_boxed(),
            }
        },
        UserProject,
        id => Option<UserProjectId>,
        user_id => UserId,
        is_current => bool
    }
}

db_update_with_conn! {
    ChangeCurrentUserProject,
    msg => conn => user_projects => {
        FindUserProject {
            id: msg.id,
            user_id: msg.user_id,
        }
        .execute(conn)?;

        inner::ChangeProjectIsCurrent {
            id: None,
            user_id: msg.user_id,
            is_current: false,
        }
        .execute(conn)?;

        inner::ChangeProjectIsCurrent {
            id: Some(msg.id),
            user_id: msg.user_id,
            is_current: false,
        }
        .execute(conn)?;
        user_projects.find(msg.id)
    },
    UserProject,
    id => UserProjectId,
    user_id => UserId
}

db_find! {
    FindByRole,
    msg => user_projects => user_projects
        .filter(user_id.eq(msg.user_id)
            .and(project_id.eq(msg.project_id))
            .and(role.eq(msg.role)
        )
    ),
    UserProject,
    user_id => UserId,
    project_id => ProjectId,
    role => UserRole
}

db_delete_with_conn! {
    RemoveInvitedUser,
    msg => conn => user_projects => {
        if msg.invited_id == msg.inviter_id {
            return Err(crate::DatabaseError::UserProject(crate::UserProjectError::InviteHimself));
        }
        FindByRole {
            user_id: msg.inviter_id,
            project_id: msg.project_id,
            role: UserRole::Owner,
        }
        .execute(conn)?;
        diesel::delete(user_projects)
            .filter(
                user_id.eq(msg.invited_id)
                    .and(project_id.eq(msg.project_id)
                )
        )
    },
    UserProject,
    invited_id => UserId,
    inviter_id => UserId,
    project_id => ProjectId
}

db_create! {
    CreateUserProject,
    msg => user_projects => diesel::insert_into(user_projects).values((
        user_id.eq(msg.user_id),
        project_id.eq(msg.project_id),
        is_current.eq(msg.is_current),
        is_default.eq(msg.is_default),
        role.eq(msg.role),
    )),
    UserProject,
    user_id => UserId,
    project_id => ProjectId,
    is_current => bool,
    is_default => bool,
    role => UserRole
}
