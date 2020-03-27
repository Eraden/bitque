use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub user_id: Option<i32>,
    pub issue_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "comments"]
pub struct CommentForm {
    pub body: String,
    pub user_id: Option<i32>,
    pub issue_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Issue {
    pub id: i32,
    pub title: String,
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
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "issues"]
pub struct IssueForm {
    pub title: String,
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
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "projects"]
pub struct ProjectForm {
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm {
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub project_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub access_token: Uuid,
    pub refresh_token: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tokens"]
pub struct TokenForm {
    pub user_id: i32,
    pub access_token: Uuid,
    pub refresh_token: Uuid,
}
