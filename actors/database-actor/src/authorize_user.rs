use {
  crate::{db_find, tokens::FindAccessToken},
  diesel::prelude::*,
  jirs_data::User,
};

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
