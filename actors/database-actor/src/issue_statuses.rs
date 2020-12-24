use {
  crate::{db_create, db_delete, db_load, db_update},
  diesel::prelude::*,
  jirs_data::{IssueStatus, IssueStatusId, Position, ProjectId, TitleString},
};

db_load! {
    LoadIssueStatuses,
    msg => issue_statuses => issue_statuses
            .distinct_on(id)
            .filter(project_id.eq(msg.project_id)),
    IssueStatus,
    project_id => ProjectId
}

db_create! {
    CreateIssueStatus,
    msg => issue_statuses => diesel::insert_into(issue_statuses).values((
            project_id.eq(msg.project_id),
            name.eq(msg.name),
            position.eq(msg.position),
    )),
    IssueStatus,
    project_id => ProjectId,
    position => i32,
    name => TitleString
}

db_delete! {
    DeleteIssueStatus,
    msg => issue_statuses => diesel::delete(issue_statuses)
        .filter(id.eq(msg.issue_status_id))
        .filter(project_id.eq(msg.project_id)
    ),
    IssueStatus,
    project_id => ProjectId,
    issue_status_id => IssueStatusId
}

db_update! {
    UpdateIssueStatus,
    msg => issue_statuses => diesel::update(issue_statuses)
        .set((
            name.eq(msg.name),
            position.eq(msg.position),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .filter(id.eq(msg.issue_status_id))
        .filter(project_id.eq(msg.project_id)),
    IssueStatus,
    issue_status_id => IssueStatusId,
    project_id => ProjectId,
    position => Position,
    name => TitleString
}
