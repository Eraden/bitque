use diesel::prelude::*;
use jirs_data::{Token, UserId};

use crate::{db_create, db_find, db_update};

db_find! {
    FindUserId,
    msg => tokens => tokens.filter(user_id.eq(msg.user_id)).order_by(id.desc()),
    Token,
    user_id => UserId
}

db_find! {
    FindBindToken,
    msg => tokens => tokens.filter(bind_token.eq(Some(msg.token))),
    Token,
    token => uuid::Uuid
}

db_update! {
    UseBindToken,
    msg => conn => tokens => {
        let token = FindBindToken { token: msg.token }.execute(conn)?;
        diesel::update(tokens.find(token.id)).set(bind_token.eq(None as Option<uuid::Uuid>))
    },
    Token,
    token => uuid::Uuid
}

db_find! {
    FindAccessToken,
    msg => tokens => tokens.filter(access_token.eq(msg.token)),
    Token,
    token => uuid::Uuid
}

db_create! {
    CreateBindToken,
    msg => tokens => diesel::insert_into(tokens).values((
        user_id.eq(msg.user_id),
        access_token.eq(uuid::Uuid::new_v4()),
        refresh_token.eq(uuid::Uuid::new_v4()),
        bind_token.eq(Some(uuid::Uuid::new_v4())),
    )),
    Token,
    user_id => UserId
}
