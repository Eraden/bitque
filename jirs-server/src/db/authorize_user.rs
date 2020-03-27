use crate::db::DbExecutor;
use crate::models::{Token, User};
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeUser {
    pub access_token: uuid::Uuid,
}

impl Message for AuthorizeUser {
    type Result = Result<crate::models::User, String>;
}

impl Handler<AuthorizeUser> for DbExecutor {
    type Result = Result<crate::models::User, String>;

    fn handle(&mut self, msg: AuthorizeUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tokens::dsl::{access_token, tokens};
        use crate::schema::users::dsl::{id, users};

        let conn: &PgConnection = &self.0.get().unwrap();
        let token = tokens
            .filter(access_token.eq(msg.access_token))
            .first::<Token>(conn)
            .map_err(|e| format!("{}", e))?;
        let user = users
            .filter(id.eq(token.user_id))
            .first::<User>(conn)
            .map_err(|e| format!("{}", e))?;
        Ok(user)
    }
}
