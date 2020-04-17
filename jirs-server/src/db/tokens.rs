use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::UserId;

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::{Token, TokenForm};

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
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let token: crate::models::Token = tokens
            .filter(bind_token.eq(Some(msg.token)))
            .first(conn)
            .map_err(|_e| ServiceErrors::RecordNotFound(format!("token for {}", msg.token)))?;

        let erase_value: Option<Uuid> = None;
        diesel::update(tokens.find(token.id))
            .set(bind_token.eq(erase_value))
            .execute(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        Ok(token)
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
        use crate::schema::tokens::dsl::tokens;
        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let access_token = Uuid::new_v4();
        let refresh_token = Uuid::new_v4();
        let bind_token = Some(Uuid::new_v4());

        let form = TokenForm {
            user_id: msg.user_id,
            access_token,
            refresh_token,
            bind_token,
        };
        let row: Token = diesel::insert_into(tokens)
            .values(form)
            .get_result(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))?;
        Ok(row)
    }
}
