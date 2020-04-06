use std::str::FromStr;

use chrono::NaiveDateTime;
#[cfg(feature = "backend")]
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "backend")]
pub use sql::*;

#[cfg(feature = "backend")]
pub mod sql;

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "IssueTypeType")]
#[derive(Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IssueType {
    Task,
    Bug,
    Story,
}

impl Default for IssueType {
    fn default() -> Self {
        IssueType::Task
    }
}

impl IssueType {
    pub fn to_label(&self) -> &str {
        match self {
            IssueType::Task => "Task",
            IssueType::Bug => "Bug",
            IssueType::Story => "Story",
        }
    }
}

impl Into<u32> for IssueType {
    fn into(self) -> u32 {
        match self {
            IssueType::Task => 1,
            IssueType::Bug => 2,
            IssueType::Story => 3,
        }
    }
}

impl Into<IssueType> for u32 {
    fn into(self) -> IssueType {
        match self {
            1 => IssueType::Task,
            2 => IssueType::Bug,
            3 => IssueType::Story,
            _ => IssueType::Task,
        }
    }
}

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueType::Task => f.write_str("task"),
            IssueType::Bug => f.write_str("bug"),
            IssueType::Story => f.write_str("story"),
        }
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "IssueStatusType")]
#[derive(Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IssueStatus {
    Backlog,
    Selected,
    InProgress,
    Done,
}

impl Default for IssueStatus {
    fn default() -> Self {
        IssueStatus::Backlog
    }
}

impl FromStr for IssueStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "backlog" => Ok(IssueStatus::Backlog),
            "selected" => Ok(IssueStatus::Selected),
            "in_progress" => Ok(IssueStatus::InProgress),
            "done" => Ok(IssueStatus::Done),
            _ => Err(format!("Invalid status {:?}", s)),
        }
    }
}

impl IssueStatus {
    pub fn to_label(&self) -> &str {
        match self {
            IssueStatus::Backlog => "Backlog",
            IssueStatus::Selected => "Selected for development",
            IssueStatus::InProgress => "In Progress",
            IssueStatus::Done => "Done",
        }
    }

    pub fn to_payload(&self) -> &str {
        match self {
            IssueStatus::Backlog => "backlog",
            IssueStatus::Selected => "selected",
            IssueStatus::InProgress => "in_progress",
            IssueStatus::Done => "done",
        }
    }

    pub fn match_name(&self, name: &str) -> bool {
        self.to_payload() == name
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "IssuePriorityType")]
#[derive(Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
pub enum IssuePriority {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
}

impl FromStr for IssuePriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "5" | "highest" => Ok(IssuePriority::Highest),
            "4" | "high" => Ok(IssuePriority::High),
            "3" | "medium" => Ok(IssuePriority::Medium),
            "2" | "low" => Ok(IssuePriority::Low),
            "1" | "lowest" => Ok(IssuePriority::Lowest),
            _ => Err(format!("Unknown priority {}", s)),
        }
    }
}

impl Default for IssuePriority {
    fn default() -> Self {
        IssuePriority::Medium
    }
}

impl std::fmt::Display for IssuePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssuePriority::Highest => f.write_str("highest"),
            IssuePriority::High => f.write_str("high"),
            IssuePriority::Medium => f.write_str("medium"),
            IssuePriority::Low => f.write_str("low"),
            IssuePriority::Lowest => f.write_str("lowest"),
        }
    }
}

impl IssuePriority {
    pub fn to_text_value(&self) -> &str {
        match self {
            IssuePriority::Highest => "5",
            IssuePriority::High => "4",
            IssuePriority::Medium => "3",
            IssuePriority::Low => "2",
            IssuePriority::Lowest => "1",
        }
    }

    pub fn to_value(&self) -> i32 {
        match self {
            IssuePriority::Highest => 5,
            IssuePriority::High => 4,
            IssuePriority::Medium => 3,
            IssuePriority::Low => 2,
            IssuePriority::Lowest => 1,
        }
    }
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullProject {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub issues: Vec<Issue>,
    pub users: Vec<User>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullIssue {
    pub id: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub list_position: f64,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub reporter_id: i32,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub user_ids: Vec<i32>,
    pub comments: Vec<Comment>,
}

impl Into<Issue> for FullIssue {
    fn into(self) -> Issue {
        Issue {
            id: self.id,
            title: self.title,
            issue_type: self.issue_type,
            status: self.status,
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
            user_ids: self.user_ids,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub list_position: f64,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub reporter_id: i32,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub user_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub user_id: i32,
    pub issue_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub user: Option<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub access_token: Uuid,
    pub refresh_token: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssuePayload {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub issue_type: Option<IssueType>,
    pub status: Option<IssueStatus>,
    pub priority: Option<IssuePriority>,
    pub list_position: Option<f64>,
    pub description: Option<Option<String>>,
    pub description_text: Option<Option<String>>,
    pub estimate: Option<Option<i32>>,
    pub time_spent: Option<Option<i32>>,
    pub time_remaining: Option<Option<i32>>,
    pub project_id: Option<i32>,
    pub user_ids: Option<Vec<i32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentPayload {
    pub user_id: Option<i32>,
    pub issue_id: i32,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentPayload {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssuePayload {
    pub title: String,
    #[serde(rename = "type")]
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: i32,
    pub user_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectPayload {
    pub name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsMsg {
    Ping,
    Pong,
    ProjectRequest,
    ProjectLoaded(Project),
    ProjectIssuesRequest(i32),
    ProjectIssuesLoaded(Vec<Issue>),
}
