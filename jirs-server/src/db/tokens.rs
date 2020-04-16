use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::UserId;

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

#[derive(Serialize, Deserialize, Debug)]
pub struct FindBindToken {
    pub token: Uuid,
}

impl Message for FindBindToken {
    type Result = Result<crate::models::Token, ServiceErrors>;
}

impl Handler<FindBindToken> for DbExecutor {
    type Result = Result<crate::models::Token, ServiceErrors>;

    fn handle(&mut self, msg: FindBindToken, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tokens::dsl::{bind_token, tokens};
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let token: crate::models::Token = tokens
            .filter(bind_token.eq(msg.token))
            .first(conn)
            .map_err(|_e| {
                ServiceErrors::RecordNotFound(format!("token for {}", msg.access_token))
            })?;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBindToken {
    pub user_id: UserId,
}

impl Message for CreateBindToken {
    type Result = Result<crate::models::Token, ServiceErrors>;
}

impl Handler<CreateBindToken> for DbExecutor {
    type Result = Result<crate::models::Token, ServiceErrors>;

    fn handle(&mut self, msg: CreateBindToken, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tokens::dsl::{access_token, tokens};
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
    }
}
