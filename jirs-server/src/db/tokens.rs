use actix::{Handler, Message};
use diesel::prelude::*;
use uuid::Uuid;

use jirs_data::msg::WsError;
use jirs_data::{Token, UserId};

use crate::{
    db::{DbExecutor, DbPooledConn},
    db_pool,
    errors::ServiceErrors,
    q,
};

pub struct FindUserId {
    pub user_id: UserId,
}

impl FindUserId {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Token, ServiceErrors> {
        use crate::schema::tokens::dsl::*;

        q!(tokens.filter(user_id.eq(self.user_id)).order_by(id.desc()))
            .first(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::Error(WsError::NoBindToken)
            })
    }
}

impl Message for FindUserId {
    type Result = Result<Token, ServiceErrors>;
}

impl Handler<FindUserId> for DbExecutor {
    type Result = Result<Token, ServiceErrors>;

    fn handle(&mut self, msg: FindUserId, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct FindBindToken {
    pub token: Uuid,
}

impl FindBindToken {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Token, ServiceErrors> {
        use crate::schema::tokens::dsl::{bind_token, tokens};

        let token: Token = q!(tokens.filter(bind_token.eq(Some(self.token))))
            .first(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::Error(WsError::BindTokenNotExists)
            })?;

        q!(diesel::update(tokens.find(token.id)).set(bind_token.eq(None as Option<Uuid>)))
            .execute(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::Error(WsError::FailedToDisableBindToken)
            })?;

        Ok(token)
    }
}

impl Message for FindBindToken {
    type Result = Result<Token, ServiceErrors>;
}

impl Handler<FindBindToken> for DbExecutor {
    type Result = Result<Token, ServiceErrors>;

    fn handle(&mut self, msg: FindBindToken, _: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct FindAccessToken {
    pub token: Uuid,
}

impl FindAccessToken {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Token, ServiceErrors> {
        use crate::schema::tokens::dsl::{access_token, tokens};

        q!(tokens.filter(access_token.eq(self.token)))
            .first(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceErrors::Error(WsError::AccessTokenNotExists)
            })
    }
}

impl Message for FindAccessToken {
    type Result = Result<Token, ServiceErrors>;
}

impl Handler<FindAccessToken> for DbExecutor {
    type Result = Result<Token, ServiceErrors>;

    fn handle(&mut self, msg: FindAccessToken, _: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct CreateBindToken {
    pub user_id: UserId,
}

impl CreateBindToken {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Token, ServiceErrors> {
        use crate::schema::tokens::dsl::*;

        q!(diesel::insert_into(tokens).values((
            user_id.eq(self.user_id),
            access_token.eq(Uuid::new_v4()),
            refresh_token.eq(Uuid::new_v4()),
            bind_token.eq(Some(Uuid::new_v4())),
        )))
        .get_result(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::Error(WsError::FailedToCreateBindToken)
        })
    }
}

impl Message for CreateBindToken {
    type Result = Result<Token, ServiceErrors>;
}

impl Handler<CreateBindToken> for DbExecutor {
    type Result = Result<Token, ServiceErrors>;

    fn handle(&mut self, msg: CreateBindToken, _: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}
