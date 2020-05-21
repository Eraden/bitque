use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::{
    InvitationState, IssuePriority, IssueStatusId, IssueType, ProjectCategory, ProjectId,
    TimeTracking, UserId,
};

use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "comments"]
pub struct CommentForm {
    pub body: String,
    pub user_id: i32,
    pub issue_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub issue_type: IssueType,
    pub priority: IssuePriority,
    pub list_position: i32,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub reporter_id: i32,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub issue_status_id: IssueStatusId,
}

impl Into<jirs_data::Issue> for Issue {
    fn into(self) -> jirs_data::Issue {
        jirs_data::Issue {
            id: self.id,
            title: self.title,
            issue_type: self.issue_type,
            priority: self.priority,
            list_position: self.list_position,
            description: self.description,
            description_text: self.description_text,
            estimate: self.estimate,
            time_spent: self.time_spent,
            time_remaining: self.time_remaining,
            reporter_id: self.reporter_id,
            project_id: self.project_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            issue_status_id: self.issue_status_id,

            user_ids: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "issues"]
pub struct CreateIssueForm {
    pub title: String,
    pub issue_type: IssueType,
    pub priority: IssuePriority,
    pub list_position: i32,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub reporter_id: UserId,
    pub project_id: ProjectId,
    pub issue_status_id: IssueStatusId,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "issue_assignees"]
pub struct CreateIssueAssigneeForm {
    pub issue_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "projects"]
pub struct UpdateProjectForm {
    pub name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<ProjectCategory>,
    pub time_tracking: Option<TimeTracking>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "projects"]
pub struct CreateProjectForm {
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: ProjectCategory,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm {
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tokens"]
pub struct TokenForm {
    pub user_id: i32,
    pub access_token: Uuid,
    pub refresh_token: Uuid,
    pub bind_token: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "invitations"]
pub struct InvitationForm {
    pub name: String,
    pub email: String,
    pub state: InvitationState,
    pub project_id: i32,
    pub invited_by_id: i32,
}
