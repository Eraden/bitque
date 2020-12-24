use {
    crate::{db_create_with_conn, db_find, db_load, db_update},
    diesel::prelude::*,
    jirs_data::{NameString, Project, ProjectCategory, ProjectId, TimeTracking, UserId},
};

db_find! {
    LoadCurrentProject,
    msg => projects => projects.find(msg.project_id),
    Project,
    project_id => ProjectId
}

mod inner {
    use {
        crate::db_create,
        diesel::prelude::*,
        jirs_data::{NameString, Project, ProjectCategory, TimeTracking},
    };

    db_create! {
        CreateProject,
        msg => projects => diesel::insert_into(projects)
            .values((
                name.eq(msg.name),
                msg.url.map(|v| url.eq(v)),
                msg.description.map(|v| description.eq(v)),
                msg.category.map(|v| category.eq(v)),
                msg.time_tracking.map(|v| time_tracking.eq(v)),
            ))
            .returning(crate::schema::projects::all_columns),
        Project,
        name => NameString,
        url => Option<String>,
        description => Option<String>,
        category => Option<ProjectCategory>,
        time_tracking => Option<TimeTracking>
    }
}

db_create_with_conn! {
    CreateProject,
    msg => conn => projects => {
        let p = inner::CreateProject {
            name: msg.name,
            url: msg.url,
            description: msg.description,
            category: msg.category,
            time_tracking: msg.time_tracking,
        }.execute(conn)?;
        crate::issue_statuses::CreateIssueStatus {
            project_id: p.id,
            position: 0,
            name: "TODO".to_string(),
        }
        .execute(conn)?;
        projects.find(p.id)
    },
    Project,
    name => NameString,
    url => Option<String>,
    description => Option<String>,
    category => Option<ProjectCategory>,
    time_tracking => Option<TimeTracking>
}

db_update! {
    UpdateProject,
    msg => projects => diesel::update(projects.find(msg.project_id)).set((
        msg.name.map(|v| name.eq(v)),
        msg.url.map(|v| url.eq(v)),
        msg.description.map(|v| description.eq(v)),
        msg.category.map(|v| category.eq(v)),
        msg.time_tracking.map(|v| time_tracking.eq(v)),
    )),
    Project,
    project_id => ProjectId,
    name => Option<NameString>,
    url => Option<String>,
    description => Option<String>,
    category => Option<ProjectCategory>,
    time_tracking => Option<TimeTracking>
}

db_load! {
    LoadProjects,
    msg => projects => {
        use crate::schema::user_projects::{dsl::{user_projects, user_id, project_id}};
        projects
            .inner_join(user_projects.on(project_id.eq(id)))
            .filter(user_id.eq(msg.user_id))
            .distinct_on(id)
            .select(crate::schema::projects::all_columns)
    },
    Project,
    user_id => UserId
}
