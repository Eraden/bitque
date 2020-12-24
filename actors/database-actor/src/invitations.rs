use {
    crate::{
        db_create, db_delete, db_find, db_load, db_pool, db_update,
        tokens::CreateBindToken,
        users::{LookupUser, Register},
        DbExecutor, DbPooledConn, InvitationError,
    },
    actix::{Handler, Message},
    diesel::prelude::*,
    jirs_data::{
        EmailString, Invitation, InvitationId, InvitationState, InvitationToken, ProjectId, Token,
        User, UserId, UserRole, UsernameString,
    },
};

db_find! {
    FindByBindToken,
    msg => invitations => invitations.filter(bind_token.eq(msg.token)),
    Invitation,
    token => InvitationToken
}

db_load! {
    ListInvitation,
    msg => invitations => invitations
            .filter(invited_by_id.eq(msg.user_id))
            .filter(state.ne(InvitationState::Accepted))
            .order_by(state.asc())
            .then_order_by(updated_at.desc()),
    Invitation,
    user_id => UserId
}

db_create! {
    CreateInvitation,
    msg => invitations => diesel::insert_into(invitations).values((
            name.eq(msg.name),
            email.eq(msg.email),
            state.eq(InvitationState::Sent),
            project_id.eq(msg.project_id),
            invited_by_id.eq(msg.user_id),
            role.eq(msg.role),
        )),
    Invitation,
    user_id => UserId,
    project_id => ProjectId,
    email => EmailString,
    name => UsernameString,
    role => UserRole
}

db_delete! {
    DeleteInvitation,
    msg => invitations => diesel::delete(invitations).filter(id.eq(msg.id)),
    Invitation,
    id => InvitationId
}

db_update! {
    UpdateInvitationState,
    msg => invitations => diesel::update(invitations)
        .set((
            state.eq(msg.state),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .filter(id.eq(msg.id)),
    Invitation,
    id => InvitationId,
    state => InvitationState
}

pub struct RevokeInvitation {
    pub id: InvitationId,
}

impl Message for RevokeInvitation {
    type Result = Result<(), crate::DatabaseError>;
}

impl Handler<RevokeInvitation> for DbExecutor {
    type Result = Result<(), crate::DatabaseError>;

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
    pub fn execute(self, conn: &DbPooledConn) -> Result<Token, crate::DatabaseError> {
        crate::Guard::new(conn)?.run::<Token, _>(|_guard| {
            let invitation = crate::invitations::FindByBindToken {
                token: self.invitation_token,
            }
            .execute(conn)?;

            if invitation.state == InvitationState::Revoked {
                return Err(crate::DatabaseError::Invitation(
                    InvitationError::InvitationRevoked,
                ));
            }

            crate::invitations::UpdateInvitationState {
                id: invitation.id,
                state: InvitationState::Accepted,
            }
            .execute(conn)?;

            UpdateInvitationState {
                id: invitation.id,
                state: InvitationState::Accepted,
            }
            .execute(conn)?;

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
                Err(crate::DatabaseError::User(crate::UserError::InvalidPair(..))) => (),
                Err(e) => return Err(e),
            };

            let user: User = LookupUser {
                name: invitation.name.clone(),
                email: invitation.email.clone(),
            }
            .execute(conn)?;
            CreateBindToken { user_id: user.id }.execute(conn)?;

            crate::user_projects::CreateUserProject {
                user_id: user.id,
                project_id: invitation.project_id,
                is_current: false,
                is_default: false,
                role: invitation.role,
            }
            .execute(conn)?;

            crate::tokens::FindUserId { user_id: user.id }.execute(conn)
        })
    }
}

impl Message for AcceptInvitation {
    type Result = Result<Token, crate::DatabaseError>;
}

impl Handler<AcceptInvitation> for DbExecutor {
    type Result = Result<Token, crate::DatabaseError>;

    fn handle(&mut self, msg: AcceptInvitation, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}
