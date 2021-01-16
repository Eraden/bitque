use {
    crate::models::Issue,
    derive_db_execute::Execute,
    diesel::{expression::sql_literal::sql, prelude::*},
    jirs_data::{IssueId, IssuePriority, IssueStatusId, IssueType, ProjectId, UserId},
};

#[derive(Default, Execute)]
#[db_exec(
    result = "Issue",
    schema = "issues",
    find = "issues.filter(id.eq(msg.issue_id)).distinct()"
)]
pub struct LoadIssue {
    pub issue_id: IssueId,
}

#[derive(Execute)]
#[db_exec(
    result = "Issue",
    schema = "issues",
    load = "issues.filter(project_id.eq(msg.project_id)).distinct()"
)]
pub struct LoadProjectIssues {
    pub project_id: ProjectId,
}

#[derive(Default, Execute)]
#[db_exec(result = "Issue", schema = "issues")]
pub struct UpdateIssue {
    pub issue_id: jirs_data::IssueId,
    pub title: Option<String>,
    pub issue_type: Option<IssueType>,
    pub priority: Option<IssuePriority>,
    pub list_position: Option<jirs_data::ListPosition>,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: Option<jirs_data::ProjectId>,
    pub user_ids: Option<Vec<jirs_data::UserId>>,
    pub reporter_id: Option<jirs_data::UserId>,
    pub issue_status_id: Option<jirs_data::IssueStatusId>,
    pub epic_id: Option<Option<jirs_data::EpicId>>,
}

impl UpdateIssue {
    fn execute(self, conn: &crate::DbPooledConn) -> Result<Issue, crate::DatabaseError> {
        let msg = self;
        use crate::schema::issues::dsl::*;
        if let Some(user_ids) = msg.user_ids {
            crate::issue_assignees::DropIssueAssignees {
                issue_id: msg.issue_id,
                user_ids: user_ids.clone(),
            }
            .execute(conn)?;

            let existing: Vec<UserId> = crate::issue_assignees::LoadAssigneesIds {
                issue_id: msg.issue_id,
            }
            .execute(conn)?;
            crate::issue_assignees::AsignMultiple {
                issue_id: msg.issue_id,
                user_ids: user_ids
                    .into_iter()
                    .filter(|u_id| !existing.contains(u_id))
                    .collect::<Vec<UserId>>(),
            }
            .execute(conn)?;
        }
        diesel::update(issues.find(msg.issue_id))
            .set((
                msg.title.map(|v| title.eq(v)),
                msg.issue_type.map(|v| issue_type.eq(v)),
                msg.issue_status_id.map(|v| issue_status_id.eq(v)),
                msg.priority.map(|p| priority.eq(p)),
                msg.list_position.map(|pos| list_position.eq(pos)),
                msg.description.map(|desc| description.eq(desc)),
                msg.description_text.map(|t| description_text.eq(t)),
                msg.estimate.map(|v| estimate.eq(v)),
                msg.time_spent.map(|v| time_spent.eq(v)),
                msg.time_remaining.map(|v| time_remaining.eq(v)),
                msg.project_id.map(|v| project_id.eq(v)),
                msg.reporter_id.map(|v| reporter_id.eq(v)),
                msg.epic_id.map(|v| epic_id.eq(v)),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(conn)
            .map_err(|e| {
                log::debug!("{:?}", e);
                crate::DatabaseError::GenericFailure(
                    crate::OperationError::Create,
                    crate::ResourceKind::Issue,
                )
            })
    }
}

#[derive(Execute)]
#[db_exec(
    result = "Issue",
    schema = "issues",
    destroy = r#"{
        crate::issue_assignees::DeleteIssueAssignees { issue_id: msg.issue_id }
            .execute(conn)?;
        diesel::delete(issues.find(msg.issue_id))
    }"#
)]
pub struct DeleteIssue {
    pub issue_id: IssueId,
}

mod inner {
    use {
        crate::models::Issue,
        derive_db_execute::Execute,
        diesel::prelude::*,
        jirs_data::{IssuePriority, IssueStatusId, IssueType},
    };

    #[derive(Default, Execute)]
    #[db_exec(
        result = "Issue",
        schema = "issues",
        create = r#"
            diesel::insert_into(issues)
                .values((
                    title.eq(msg.title),
                    issue_type.eq(msg.issue_type),
                    issue_status_id.eq(msg.issue_status_id),
                    priority.eq(msg.priority),
                    list_position.eq(msg.list_position),
                    description.eq(msg.description),
                    description_text.eq(msg.description_text),
                    estimate.eq(msg.estimate),
                    time_spent.eq(msg.time_spent),
                    time_remaining.eq(msg.time_remaining),
                    reporter_id.eq(msg.reporter_id),
                    project_id.eq(msg.project_id),
                    epic_id.eq(msg.epic_id)
                ))
                .on_conflict_do_nothing()
        "#
    )]
    pub struct CreateIssue {
        pub title: String,
        pub list_position: i32,
        pub issue_type: IssueType,
        pub issue_status_id: IssueStatusId,
        pub priority: IssuePriority,
        pub description: Option<String>,
        pub description_text: Option<String>,
        pub estimate: Option<i32>,
        pub time_spent: Option<i32>,
        pub time_remaining: Option<i32>,
        pub project_id: jirs_data::ProjectId,
        pub reporter_id: jirs_data::UserId,
        pub epic_id: Option<jirs_data::EpicId>,
    }
}

#[derive(Execute)]
#[db_exec(result = "Issue", schema = "issues")]
pub struct CreateIssue {
    pub title: String,
    pub issue_type: IssueType,
    pub issue_status_id: IssueStatusId,
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: jirs_data::ProjectId,
    pub reporter_id: jirs_data::UserId,
    pub user_ids: Vec<jirs_data::UserId>,
    pub epic_id: Option<jirs_data::EpicId>,
}

impl CreateIssue {
    fn execute(self, conn: &crate::DbPooledConn) -> Result<Issue, crate::DatabaseError> {
        use crate::schema::issues::dsl::*;
        let msg = self;

        let pos = issues
            .select(sql("COALESCE(max(list_position), 0) + 1"))
            .get_result::<i32>(conn)
            .map_err(|e| {
                log::error!("resolve new issue position failed {}", e);
                crate::DatabaseError::Issue(crate::IssueError::BadListPosition)
            })?;
        let i_s_id: IssueStatusId = if msg.issue_status_id == 0 {
            crate::issue_statuses::LoadIssueStatuses {
                project_id: msg.project_id,
            }
            .execute(conn)?
            .first()
            .ok_or_else(|| crate::DatabaseError::Issue(crate::IssueError::NoIssueStatuses))?
            .id
        } else {
            msg.issue_status_id
        };
        let assign_users = msg
            .user_ids
            .iter()
            .cloned()
            .filter(|u_id| *u_id != msg.reporter_id)
            .collect::<Vec<UserId>>();
        let issue = inner::CreateIssue {
            title: msg.title,
            list_position: pos,
            issue_type: msg.issue_type,
            issue_status_id: i_s_id,
            priority: msg.priority,
            description: msg.description,
            description_text: msg.description_text,
            estimate: msg.estimate,
            time_spent: msg.time_spent,
            time_remaining: msg.time_remaining,
            project_id: msg.project_id,
            reporter_id: msg.reporter_id,
            epic_id: msg.epic_id,
        }
        .execute(conn)?;
        crate::issue_assignees::AsignMultiple {
            issue_id: issue.id,
            user_ids: assign_users,
        };
        issues.find(issue.id).get_result(conn).map_err(|e| {
            log::error!("{:?}", e);
            crate::DatabaseError::GenericFailure(
                crate::OperationError::Create,
                crate::ResourceKind::Issue,
            )
        })
    }
}
