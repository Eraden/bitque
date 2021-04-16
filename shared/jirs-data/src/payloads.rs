use serde::{Deserialize, Serialize};

use crate::{
    CommentId, EpicId, Issue, IssueId, IssuePriority, IssueStatusId, IssueType, ProjectCategory,
    ProjectId, TimeTracking, UserId,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateCommentPayload {
    pub user_id: Option<UserId>,
    pub issue_id: IssueId,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UpdateCommentPayload {
    pub id: CommentId,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateIssuePayload {
    pub title: String,
    pub issue_type: IssueType,
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: ProjectId,
    pub user_ids: Vec<UserId>,
    pub reporter_id: UserId,
    pub issue_status_id: IssueStatusId,
    pub epic_id: Option<EpicId>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateProjectPayload {
    pub id: ProjectId,
    pub name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<ProjectCategory>,
    pub time_tracking: Option<TimeTracking>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PayloadVariant {
    OptionI32(Option<i32>),
    VecI32(Vec<i32>),
    I32(i32),
    String(String),
    IssueType(IssueType),
    IssuePriority(IssuePriority),
    ProjectCategory(ProjectCategory),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct UpdateIssuePayload {
    pub title: String,
    pub issue_type: IssueType,
    pub priority: IssuePriority,
    pub list_position: i32,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: ProjectId,
    pub reporter_id: UserId,
    pub issue_status_id: IssueStatusId,
    pub user_ids: Vec<UserId>,
}

impl From<Issue> for UpdateIssuePayload {
    fn from(issue: Issue) -> Self {
        Self {
            title: issue.title,
            issue_type: issue.issue_type,
            priority: issue.priority,
            list_position: issue.list_position,
            description: issue.description,
            description_text: issue.description_text,
            estimate: issue.estimate,
            time_spent: issue.time_spent,
            time_remaining: issue.time_remaining,
            project_id: issue.project_id,
            reporter_id: issue.reporter_id,
            user_ids: issue.user_ids,
            issue_status_id: issue.issue_status_id,
        }
    }
}
