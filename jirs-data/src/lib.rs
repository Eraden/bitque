use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait ResponseData {
    type Response: Serialize;

    fn into_response(self) -> Self::Response;
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
