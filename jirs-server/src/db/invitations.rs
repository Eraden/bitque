use actix::{Handler, Message};
use diesel::prelude::*;

use jirs_data::{
    msg::WsError, EmailString, Invitation, InvitationId, InvitationState, InvitationToken,
    ProjectId, Token, User, UserId, UserRole, UsernameString,
};

use crate::db::DbPooledConn;
use crate::{
    db::{
        tokens::CreateBindToken,
        users::{LookupUser, Register},
        DbExecutor,
    },
    db_pool,
    errors::ServiceErrors,
    q,
};

pub struct FindByBindToken {
    pub token: InvitationToken,
}

impl FindByBindToken {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Invitation, ServiceErrors> {
        use crate::schema::invitations::dsl::*;
        q!(invitations.filter(bind_token.eq(self.token)))
            .first(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))
    }
}

impl Message for FindByBindToken {
    type Result = Result<Invitation, ServiceErrors>;
}

impl Handler<FindByBindToken> for DbExecutor {
    type Result = Result<Invitation, ServiceErrors>;

    fn handle(&mut self, msg: FindByBindToken, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct ListInvitation {
    pub user_id: UserId,
}

impl Message for ListInvitation {
    type Result = Result<Vec<Invitation>, ServiceErrors>;
}

impl Handler<ListInvitation> for DbExecutor {
    type Result = Result<Vec<Invitation>, ServiceErrors>;

    fn handle(&mut self, msg: ListInvitation, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;

        let conn = db_pool!(self);

        q!(invitations
            .filter(invited_by_id.eq(msg.user_id))
            .filter(state.ne(InvitationState::Accepted))
            .order_by(state.asc())
            .then_order_by(updated_at.desc()))
        .load(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::Error(WsError::FailedToLoadInvitations)
        })
    }
}

pub struct CreateInvitation {
    pub user_id: UserId,
    pub project_id: ProjectId,
    pub email: EmailString,
    pub name: UsernameString,
    pub role: UserRole,
}

impl CreateInvitation {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Invitation, ServiceErrors> {
        use crate::schema::invitations::dsl::*;
        q!(diesel::insert_into(invitations).values((
            name.eq(self.name),
            email.eq(self.email),
            state.eq(InvitationState::Sent),
            project_id.eq(self.project_id),
            invited_by_id.eq(self.user_id),
            role.eq(self.role),
        )))
        .get_result(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::Error(WsError::InvalidInvitation)
        })
    }
}

impl Message for CreateInvitation {
    type Result = Result<Invitation, ServiceErrors>;
}

impl Handler<CreateInvitation> for DbExecutor {
    type Result = Result<Invitation, ServiceErrors>;

    fn handle(&mut self, msg: CreateInvitation, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct DeleteInvitation {
    pub id: InvitationId,
}

impl DeleteInvitation {
    pub fn execute(self, conn: &DbPooledConn) -> Result<usize, ServiceErrors> {
        use crate::schema::invitations::dsl::*;
        q!(diesel::delete(invitations).filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::Error(WsError::UnableToDeleteInvitation)
            })
    }
}

impl Message for DeleteInvitation {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<DeleteInvitation> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: DeleteInvitation, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)?;
        Ok(())
    }
}

struct UpdateInvitationState {
    pub id: InvitationId,
    pub state: InvitationState,
}

impl UpdateInvitationState {
    pub fn execute(self, conn: &DbPooledConn) -> Result<usize, ServiceErrors> {
        use crate::schema::invitations::dsl::*;

        q!(diesel::update(invitations)
            .set((
                state.eq(self.state),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .filter(id.eq(self.id)))
        .execute(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::Error(WsError::FailedToUpdateInvitation)
        })
    }
}

impl Message for UpdateInvitationState {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<UpdateInvitationState> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: UpdateInvitationState, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)?;
        Ok(())
    }
}

pub struct RevokeInvitation {
    pub id: InvitationId,
}

impl Message for RevokeInvitation {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<RevokeInvitation> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: RevokeInvitation, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        UpdateInvitationState {
            id: msg.id,
            state: InvitationState::Revoked,
        }
        .execute(conn)?;
        Ok(())
    }
}

pub struct AcceptInvitation {
    pub invitation_token: InvitationToken,
}

impl AcceptInvitation {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Token, ServiceErrors> {
        use crate::schema::invitations::dsl::*;

        crate::db::Guard::new(conn)?.run::<Token, _>(|_guard| {
            let invitation = crate::db::invitations::FindByBindToken {
                token: self.invitation_token,
            }
            .execute(conn)?;

            if invitation.state == InvitationState::Revoked {
                return Err(ServiceErrors::Error(WsError::InvitationRevoked));
            }

            crate::db::invitations::UpdateInvitationState {
                id: invitation.id,
                state: InvitationState::Accepted,
            }
            .execute(conn)?;

            q!(diesel::update(invitations)
                .set((
                    state.eq(InvitationState::Accepted),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .filter(id.eq(invitation.id))
                .filter(state.eq(InvitationState::Sent)))
            .execute(conn)
            .map_err(|e| {
                ServiceErrors::DatabaseQueryFailed(format!(
                    "update invitation {} {}",
                    invitation.id, e
                ))
            })?;

            match {
                Register {
                    name: invitation.name.clone(),
                    email: invitation.email.clone(),
                    project_id: Some(invitation.project_id),
                    role: UserRole::User,
                }
                .execute(conn)
            } {
                Ok(_) => (),
                Err(ServiceErrors::Error(WsError::InvalidPair(..))) => (),
                Err(e) => return Err(e),
            };

            let user: User = LookupUser {
                name: invitation.name.clone(),
                email: invitation.email.clone(),
            }
            .execute(conn)?;
            CreateBindToken { user_id: user.id }.execute(conn)?;

            self.bind_to_default_project(conn, &invitation, &user)?;

            crate::db::tokens::FindUserId { user_id: user.id }.execute(conn)
        })
    }

    fn bind_to_default_project(
        &self,
        conn: &DbPooledConn,
        invitation: &Invitation,
        user: &User,
    ) -> Result<usize, ServiceErrors> {
        crate::db::user_projects::CreateUserProject {
            user_id: user.id,
            project_id: invitation.project_id,
            is_current: false,
            is_default: false,
            role: invitation.role,
        }
        .execute(conn)
    }
}

impl Message for AcceptInvitation {
    type Result = Result<Token, ServiceErrors>;
}

impl Handler<AcceptInvitation> for DbExecutor {
    type Result = Result<Token, ServiceErrors>;

    fn handle(&mut self, msg: AcceptInvitation, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}
