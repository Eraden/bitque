use diesel::prelude::*;
use jirs_data::{TextEditorMode, UserId, UserSetting};

use crate::{db_find, db_update};

db_find! {
    FindUserSetting,
    msg => user_settings => user_settings
            .distinct_on(id)
            .filter(user_id.eq(msg.user_id))
            .limit(1),
    UserSetting,
    user_id => UserId
}

db_update! {
    UpdateUserSetting,
    msg => conn => user_settings => {
        inner::Update { user_id: msg.user_id, mode: msg.mode }
        .execute(conn).or_else(|_|
            inner::Create { user_id: msg.user_id, mode: msg.mode }
            .execute(conn)
        )?;
        user_settings.filter(user_id.eq(msg.user_id))
    },
    UserSetting,
    user_id => UserId,
    mode => TextEditorMode
}

mod inner {
    use diesel::prelude::*;
    use jirs_data::{TextEditorMode, UserId, UserSetting};

    use crate::{db_create, db_update};

    db_update! {
        Update,
        msg => user_settings => {
            diesel::update(user_settings.filter(user_id.eq(msg.user_id))).set(text_editor_mode.eq(msg.mode))
        },
        UserSetting,
        user_id => UserId,
        mode => TextEditorMode
    }

    db_create! {
        Create,
        msg => user_settings => diesel::insert_into(user_settings).values((
            user_id.eq(msg.user_id),
            text_editor_mode.eq(msg.mode)
        )),
        UserSetting,
        user_id => UserId,
        mode => TextEditorMode
    }
}
