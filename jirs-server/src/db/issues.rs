use actix::{Handler, Message};
use diesel::expression::dsl::not;
use diesel::expression::sql_literal::sql;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::{IssuePriority, IssueStatus, IssueType};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::Issue;

const FAILED_CONNECT_USER_AND_ISSUE: &str = "Failed to create connection between user and issue";

#[derive(Serialize, Deserialize)]
pub struct LoadIssue {
    pub issue_id: i32,
}

impl Message for LoadIssue {
    type Result = Result<Issue, ServiceErrors>;
}

impl Handler<LoadIssue> for DbExecutor {
    type Result = Result<Issue, ServiceErrors>;

    fn handle(&mut self, msg: LoadIssue, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issues::dsl::{id, issues};
        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let record = issues
            .filter(id.eq(msg.issue_id))
            .distinct()
            .first::<Issue>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project issues".to_string()))?;
        Ok(record)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoadProjectIssues {
    pub project_id: i32,
}

impl Message for LoadProjectIssues {
    type Result = Result<Vec<Issue>, ServiceErrors>;
}

impl Handler<LoadProjectIssues> for DbExecutor {
    type Result = Result<Vec<Issue>, ServiceErrors>;

    fn handle(&mut self, msg: LoadProjectIssues, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issues::dsl::{issues, project_id};
        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let chain = issues.filter(project_id.eq(msg.project_id)).distinct();
        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&chain).to_string()
        );
        let vec = chain
            .load::<Issue>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project issues".to_string()))?;
        Ok(vec)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct UpdateIssue {
    pub issue_id: i32,
    pub title: Option<String>,
    pub issue_type: Option<IssueType>,
    pub status: Option<IssueStatus>,
    pub priority: Option<IssuePriority>,
    pub list_position: Option<i32>,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: Option<i32>,
    pub user_ids: Option<Vec<i32>>,
    pub reporter_id: Option<i32>,
}

impl Message for UpdateIssue {
    type Result = Result<Issue, ServiceErrors>;
}

impl Handler<UpdateIssue> for DbExecutor {
    type Result = Result<Issue, ServiceErrors>;

    fn handle(&mut self, msg: UpdateIssue, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issues::dsl::{self, issues};
        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let current_issue_id = msg.issue_id;

        let chain = diesel::update(issues.find(current_issue_id)).set((
            msg.title.map(|title| dsl::title.eq(title)),
            msg.issue_type
                .map(|issue_type| dsl::issue_type.eq(issue_type)),
            msg.status.map(|status| dsl::status.eq(status)),
            msg.priority.map(|priority| dsl::priority.eq(priority)),
            msg.list_position
                .map(|list_position| dsl::list_position.eq(list_position)),
            msg.description
                .map(|description| dsl::description.eq(description)),
            msg.description_text
                .map(|description_text| dsl::description_text.eq(description_text)),
            msg.estimate.map(|estimate| dsl::estimate.eq(estimate)),
            msg.time_spent
                .map(|time_spent| dsl::time_spent.eq(time_spent)),
            msg.time_remaining
                .map(|time_remaining| dsl::time_remaining.eq(time_remaining)),
            msg.project_id
                .map(|project_id| dsl::project_id.eq(project_id)),
            msg.reporter_id
                .map(|reporter_id| dsl::reporter_id.eq(reporter_id)),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
        ));
        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&chain).to_string()
        );
        chain.get_result::<Issue>(conn).map_err(|_| {
            ServiceErrors::DatabaseQueryFailed("Failed to update issue".to_string())
        })?;

        if let Some(user_ids) = msg.user_ids.as_ref() {
            use crate::schema::issue_assignees::dsl;
            diesel::delete(dsl::issue_assignees)
                .filter(not(dsl::user_id.eq_any(user_ids)).and(dsl::issue_id.eq(current_issue_id)))
                .execute(conn)
                .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
            let existing: Vec<i32> = dsl::issue_assignees
                .select(dsl::user_id)
                .filter(dsl::issue_id.eq(current_issue_id))
                .get_results::<i32>(conn)
                .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
            let mut values = vec![];
            for user_id in user_ids.iter() {
                if !existing.contains(user_id) {
                    values.push(crate::models::CreateIssueAssigneeForm {
                        issue_id: current_issue_id,
                        user_id: *user_id,
                    })
                }
            }
            diesel::insert_into(dsl::issue_assignees)
                .values(values)
                .execute(conn)
                .map_err(|_| {
                    ServiceErrors::DatabaseQueryFailed(FAILED_CONNECT_USER_AND_ISSUE.to_string())
                })?;
        }

        issues
            .find(msg.issue_id)
            .first::<Issue>(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteIssue {
    pub issue_id: i32,
}

impl Message for DeleteIssue {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<DeleteIssue> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: DeleteIssue, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_assignees::dsl::{issue_assignees, issue_id};
        use crate::schema::issues::dsl::issues;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        diesel::delete(issue_assignees.filter(issue_id.eq(msg.issue_id)))
            .execute(conn)
            .map_err(|e| ServiceErrors::RecordNotFound(format!("issue {}. {}", msg.issue_id, e)))?;
        diesel::delete(issues.find(msg.issue_id))
            .execute(conn)
            .map_err(|e| ServiceErrors::RecordNotFound(format!("issue {}. {}", msg.issue_id, e)))?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateIssue {
    pub title: String,
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: i32,
    pub reporter_id: i32,
    pub user_ids: Vec<i32>,
}

impl Message for CreateIssue {
    type Result = Result<Issue, ServiceErrors>;
}

impl Handler<CreateIssue> for DbExecutor {
    type Result = Result<Issue, ServiceErrors>;

    fn handle(&mut self, msg: CreateIssue, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issue_assignees::dsl;
        use crate::schema::issues::dsl::{issues, status};

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let list_position = issues
            .filter(status.eq(IssueStatus::Backlog))
            .select(sql("max(list_position) + 1"))
            .get_result::<i32>(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let form = crate::models::CreateIssueForm {
            title: msg.title,
            issue_type: msg.issue_type,
            status: msg.status,
            priority: msg.priority,
            list_position,
            description: msg.description,
            description_text: msg.description_text,
            estimate: msg.estimate,
            time_spent: msg.time_spent,
            time_remaining: msg.time_remaining,
            reporter_id: msg.reporter_id,
            project_id: msg.project_id,
        };

        let issue = diesel::insert_into(issues)
            .values(form)
            .on_conflict_do_nothing()
            .get_result::<Issue>(conn)
            .map_err(|e| {
                error!("{}", e);
                ServiceErrors::DatabaseConnectionLost
            })?;

        let mut values = vec![];
        for user_id in msg.user_ids.iter() {
            values.push(crate::models::CreateIssueAssigneeForm {
                issue_id: issue.id,
                user_id: *user_id,
            });
        }
        if !msg.user_ids.contains(&msg.reporter_id) {
            values.push(crate::models::CreateIssueAssigneeForm {
                issue_id: issue.id,
                user_id: msg.reporter_id,
            });
        }

        diesel::insert_into(dsl::issue_assignees)
            .values(values)
            .execute(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        Ok(issue)
    }
}
