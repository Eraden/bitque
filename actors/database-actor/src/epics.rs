use {
    crate::{db_create, db_delete, db_load, db_update},
    diesel::prelude::*,
    jirs_data::Epic,
};

db_load! {
    LoadEpics,
    msg => epics => epics.distinct_on(id).filter(project_id.eq(msg.project_id)),
    Epic,
    project_id => i32
}

db_create! {
    CreateEpic,
    msg => epics => diesel::insert_into(epics).values((
                name.eq(msg.name.as_str()),
                user_id.eq(msg.user_id),
                project_id.eq(msg.project_id),
            )),
    Epic,
    user_id => i32,
    project_id => i32,
    name => String
}

db_update! {
    UpdateEpic,
    msg => epics => diesel::update(
        epics
            .filter(project_id.eq(msg.project_id))
            .find(msg.epic_id),
    ).set(name.eq(msg.name)),
    Epic,
    epic_id => i32,
    project_id => i32,
    name => String
}

db_delete! {
    DeleteEpic,
    msg => epics => diesel::delete(
        epics.filter(user_id.eq(msg.user_id)).find(msg.epic_id)
    ),
    Epic,
    user_id => i32,
    epic_id => i32
}
