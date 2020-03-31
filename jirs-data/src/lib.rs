use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub trait ResponseData {
    type Response: Serialize;

    fn into_response(self) -> Self::Response;
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IssueType {
    Task,
    Bug,
    Story,
}

impl FromStr for IssueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "task" => Ok(IssueType::Task),
            "bug" => Ok(IssueType::Bug),
            "story" => Ok(IssueType::Story),
            _ => Err(format!("Unknown type {:?}", s)),
        }
    }
}

impl ToString for IssueType {
    fn to_string(&self) -> String {
        match self {
            IssueType::Task => "Task",
            IssueType::Bug => "Bug",
            IssueType::Story => "Story",
        }
        .to_string()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
pub enum IssueStatus {
    Backlog,
    Selected,
    InProgress,
    Done,
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

    #[deprecated]
    pub fn to_deprecated_payload(&self) -> &str {
        match self {
            IssueStatus::Backlog => "backlog",
            IssueStatus::Selected => "selected",
            IssueStatus::InProgress => "inprogress",
            IssueStatus::Done => "done",
        }
    }

    pub fn match_name(&self, name: &str) -> bool {
        self.to_payload() == name || self.to_deprecated_payload() == name
    }
}

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
        match s.to_lowercase().as_str() {
            "highest" => Ok(IssuePriority::Highest),
            "high" => Ok(IssuePriority::High),
            "medium" => Ok(IssuePriority::Medium),
            "low" => Ok(IssuePriority::Low),
            "lowest" => Ok(IssuePriority::Lowest),
            _ => Err(format!("Unknown priority {}", s)),
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
pub struct FullProjectResponse {
    pub project: FullProject,
}

impl ResponseData for FullProject {
    type Response = FullProjectResponse;

    fn into_response(self) -> Self::Response {
        FullProjectResponse { project: self }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullIssue {
    pub id: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub issue_type: String,
    pub status: String,
    pub priority: String,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullIssueResponse {
    pub issue: FullIssue,
}

impl ResponseData for FullIssue {
    type Response = FullIssueResponse;

    fn into_response(self) -> Self::Response {
        FullIssueResponse { issue: self }
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
    pub issue_type: String,
    pub status: String,
    pub priority: String,
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
    pub issue_type: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub list_position: Option<f64>,
    pub description: Option<Option<String>>,
    pub description_text: Option<Option<String>>,
    pub estimate: Option<Option<i32>>,
    pub time_spent: Option<Option<i32>>,
    pub time_remaining: Option<Option<i32>>,
    pub project_id: Option<i32>,

    pub users: Option<Vec<User>>,
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
    pub issue_type: String,
    pub status: String,
    pub priority: String,
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
