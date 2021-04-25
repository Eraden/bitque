use derive_db_execute::Execute;
use diesel::prelude::*;
use jirs_data::{DescriptionString, EndsAt, Epic, EpicId, ProjectId, StartsAt};

use crate::{db_create, db_delete, db_load, db_update};

#[derive(Execute)]
#[db_exec(schema = "epics", result = "Epic", find = "epics.find(msg.epic_id)")]
pub struct FindEpic {
    pub epic_id: EpicId,
}

db_load! {
    LoadEpics,
    msg => epics => epics.distinct_on(id).filter(project_id.eq(msg.project_id)),
    Epic,
    project_id => ProjectId
}

db_create! {
    CreateEpic,
    msg => epics => diesel::insert_into(epics).values((
                name.eq(msg.name.as_str()),
                user_id.eq(msg.user_id),
                project_id.eq(msg.project_id),
                msg.description.map(|d| description.eq(d)),
                msg.description_html.map(|d| description_html.eq(d)),
            )),
    Epic,
    user_id => i32,
    project_id => i32,
    name => String,
    description => Option<DescriptionString>,
    description_html => Option<DescriptionString>
}

db_update! {
    UpdateEpicName,
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

db_update! {
    UpdateEpicStartsAt,
    msg => epics => diesel::update(
        epics
            .filter(project_id.eq(msg.project_id))
            .find(msg.epic_id),
    ).set(starts_at.eq(msg.starts_at)),
    Epic,
    epic_id => i32,
    project_id => i32,
    starts_at => Option<StartsAt>
}

db_update! {
    UpdateEpicEndsAt,
    msg => epics => diesel::update(
        epics
            .filter(project_id.eq(msg.project_id))
            .find(msg.epic_id),
    ).set(ends_at.eq(msg.ends_at)),
    Epic,
    epic_id => i32,
    project_id => i32,
    ends_at => Option<EndsAt>
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
