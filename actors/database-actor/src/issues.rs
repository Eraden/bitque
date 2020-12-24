use {
    crate::{
        db_create_with_conn, db_delete_with_conn, db_find, db_load, db_update_with_conn,
        models::Issue,
    },
    diesel::{expression::sql_literal::sql, prelude::*},
    jirs_data::{IssueId, IssuePriority, IssueStatusId, IssueType, ProjectId, UserId},
};

db_find! {
    LoadIssue,
    msg => issues => issues.filter(id.eq(msg.issue_id)).distinct(),
    Issue,
    issue_id => IssueId
}

db_load! {
    LoadProjectIssues,
    msg => issues => issues.filter(project_id.eq(msg.project_id)).distinct(),
    Issue,
    project_id => ProjectId
}

db_update_with_conn! {
    UpdateIssue,
    msg => conn => issues => {
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
        diesel::update(issues.find(msg.issue_id)).set((
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
    },
    Issue,
    issue_id => i32,
    title => Option<String>,
    issue_type => Option<IssueType>,
    priority => Option<IssuePriority>,
    list_position => Option<i32>,
    description => Option<String>,
    description_text => Option<String>,
    estimate => Option<i32>,
    time_spent => Option<i32>,
    time_remaining => Option<i32>,
    project_id => Option<i32>,
    user_ids => Option<Vec<i32>>,
    reporter_id => Option<i32>,
    issue_status_id => Option<i32>,
    epic_id => Option<Option<i32>>
}

db_delete_with_conn! {
    DeleteIssue,
    msg => conn => issues => {
        crate::issue_assignees::DeleteIssueAssignees { issue_id: msg.issue_id }
            .execute(conn)?;
        diesel::delete(issues.find(msg.issue_id))
    },
    Issue,
    issue_id => IssueId
}

mod inner {
    use {
        crate::{db_create, models::Issue},
        diesel::prelude::*,
        jirs_data::{IssuePriority, IssueStatusId, IssueType},
    };

    db_create! {
        CreateIssue,
        msg => issues => diesel::insert_into(issues)
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
            .on_conflict_do_nothing(),
        Issue,
        title => String,
        list_position => i32,
        issue_type => IssueType,
        issue_status_id => IssueStatusId,
        priority => IssuePriority,
        description => Option<String>,
        description_text => Option<String>,
        estimate => Option<i32>,
        time_spent => Option<i32>,
        time_remaining => Option<i32>,
        project_id => jirs_data::ProjectId,
        reporter_id => jirs_data::UserId,
        epic_id => Option<jirs_data::EpicId>
    }
}

db_create_with_conn! {
    CreateIssue,
    msg => conn => issues => {
        let pos = issues
            .select(sql("COALESCE(max(list_position), 0) + 1"))
            .get_result::<i32>(conn)
            .map_err(|e| {
                log::error!("resolve new issue position failed {}", e);
                crate::DatabaseError::Issue(crate::IssueError::BadListPosition)
        })?;
        let i_s_id: IssueStatusId = if msg.issue_status_id == 0 {
            crate::issue_statuses::LoadIssueStatuses { project_id: msg.project_id }
                .execute(conn)?
                .first()
                .ok_or_else(|| crate::DatabaseError::Issue(crate::IssueError::NoIssueStatuses))?
                .id
        } else {
            msg.issue_status_id
        };
        let assign_users = msg.user_ids
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
        }.execute(conn)?;
        crate::issue_assignees::AsignMultiple {
            issue_id: issue.id,
            user_ids: assign_users,
        };
        issues.find(issue.id)
    },
    Issue,
    title => String,
    issue_type => IssueType,
    issue_status_id => IssueStatusId,
    priority => IssuePriority,
    description => Option<String>,
    description_text => Option<String>,
    estimate => Option<i32>,
    time_spent => Option<i32>,
    time_remaining => Option<i32>,
    project_id => jirs_data::ProjectId,
    reporter_id => jirs_data::UserId,
    user_ids => Vec<jirs_data::UserId>,
    epic_id => Option<jirs_data::EpicId>
}

// impl Handler<CreateIssue> for DbExecutor {
//     type Result = Result<Issue, crate::DatabaseError>;
//
//     fn handle(&mut self, msg: CreateIssue, ctx: &mut Self::Context) -> Self::Result {
//         use crate::schema::issue_assignees::dsl;
//         use crate::schema::issues::dsl::issues;
//
//         let mut values = vec![];
//         for user_id in msg.user_ids.iter() {
//             values.push(crate::models::CreateIssueAssigneeForm {
//                 issue_id: issue.id,
//                 user_id: *user_id,
//             });
//         }
//         if !msg.user_ids.contains(&msg.reporter_id) {
//             values.push(crate::models::CreateIssueAssigneeForm {
//                 issue_id: issue.id,
//                 user_id: msg.reporter_id,
//             });
//         }
//
//         diesel::insert_into(dsl::issue_assignees)
//             .values(values)
//             .execute(conn)
//             .map_err(|e| {
//                 log::error!("{:?}", e);
//                 crate::DatabaseError::DatabaseConnectionLost
//             })?;
//
//         Ok(issue)
//     }
// }
