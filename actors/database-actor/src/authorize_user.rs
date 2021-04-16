use diesel::prelude::*;
use jirs_data::User;

use crate::db_find;
use crate::tokens::FindAccessToken;

db_find! {
    AuthorizeUser,
    msg => conn => users => {
        let token = FindAccessToken {
            token: msg.access_token,
        }
        .execute(conn)?;
        users.find(token.user_id)
    },
    User,
    access_token => uuid::Uuid
}
