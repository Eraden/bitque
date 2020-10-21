use actix::{Handler, Message};

use jirs_data::User;

use crate::{
    db::{tokens::FindAccessToken, DbExecutor, DbPool, DbPooledConn, SyncQuery},
    db_pool,
    errors::ServiceErrors,
};

pub struct AuthorizeUser {
    pub access_token: uuid::Uuid,
}

impl Message for AuthorizeUser {
    type Result = Result<User, ServiceErrors>;
}

impl AuthorizeUser {
    pub fn execute(&self, conn: &DbPooledConn) -> Result<User, ServiceErrors> {
        let token = FindAccessToken {
            token: self.access_token,
        }
        .execute(conn)?;

        crate::db::users::FindUser {
            user_id: token.user_id,
        }
        .execute(conn)
    }
}

impl Handler<AuthorizeUser> for DbExecutor {
    type Result = Result<User, ServiceErrors>;

    fn handle(&mut self, msg: AuthorizeUser, _: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

impl SyncQuery for AuthorizeUser {
    type Result = std::result::Result<User, crate::errors::ServiceErrors>;

    fn handle(&self, pool: &DbPool) -> Self::Result {
        let conn = pool.get().map_err(|e| {
            error!("{:?}", e);
            crate::errors::ServiceErrors::DatabaseConnectionLost
        })?;
        self.execute(&conn)
    }
}
