use std::cmp::Ordering;
use std::str::FromStr;

use chrono::NaiveDateTime;
use derive_enum_iter::EnumIter;
use derive_enum_primitive::EnumPrimitive;
#[cfg(feature = "backend")]
use derive_enum_sql::EnumSql;
#[cfg(feature = "backend")]
use diesel::*;
pub use fields::*;
pub use msg::WsMsg;
pub use payloads::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod fields;
pub mod msg;
mod payloads;

pub type NumberOfDeleted = usize;
pub type IssueId = i32;
pub type ListPosition = i32;
pub type ProjectId = i32;
pub type ProjectName = String;
pub type UserId = i32;
pub type UserProjectId = i32;
pub type CommentId = i32;
pub type TokenId = i32;
pub type IssueStatusId = i32;
pub type IssueStatusName = String;
pub type InvitationId = i32;
pub type Position = i32;
pub type MessageId = i32;
pub type EpicId = i32;
pub type EpicName = String;

pub type EmailString = String;
pub type UsernameString = String;
pub type TitleString = String;
pub type NameString = String;
pub type AvatarUrl = String;
pub type DescriptionString = String;

pub type Code = String;
pub type Lang = String;

pub type BindToken = Uuid;
pub type InvitationToken = Uuid;

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression, EnumSql))]
#[cfg_attr(feature = "backend", sql_type = "IssueTypeType")]
#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash, EnumIter, EnumPrimitive,
)]
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

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression, EnumSql))]
#[cfg_attr(feature = "backend", sql_type = "IssuePriorityType")]
#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash, EnumIter, EnumPrimitive,
)]
pub enum IssuePriority {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
}

impl Default for IssuePriority {
    fn default() -> Self {
        IssuePriority::Medium
    }
}

impl std::fmt::Display for IssuePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression, EnumSql))]
#[cfg_attr(feature = "backend", sql_type = "UserRoleType")]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Hash, EnumIter, EnumPrimitive)]
pub enum UserRole {
    User,
    Manager,
    Owner,
}

impl PartialOrd for UserRole {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use UserRole::*;

        if self == other {
            return Some(Ordering::Equal);
        }
        let order = match (self, other) {
            (User, Manager) | (User, Owner) | (Manager, Owner) => Ordering::Less,
            _ => Ordering::Greater,
        };
        Some(order)
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression, EnumSql))]
#[cfg_attr(feature = "backend", sql_type = "ProjectCategoryType")]
#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash, EnumIter, EnumPrimitive,
)]
pub enum ProjectCategory {
    Software,
    Marketing,
    Business,
}

impl Default for ProjectCategory {
    fn default() -> Self {
        ProjectCategory::Software
    }
}

impl std::fmt::Display for ProjectCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression, EnumSql))]
#[cfg_attr(feature = "backend", sql_type = "InvitationStateType")]
#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash, EnumIter, EnumPrimitive,
)]
pub enum InvitationState {
    Sent,
    Accepted,
    Revoked,
}

impl Default for InvitationState {
    fn default() -> Self {
        InvitationState::Sent
    }
}

impl std::fmt::Display for InvitationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression, EnumSql))]
#[cfg_attr(feature = "backend", sql_type = "TimeTrackingType")]
#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash, EnumIter, EnumPrimitive,
)]
pub enum TimeTracking {
    Untracked,
    Fibonacci,
    Hourly,
}

impl Default for TimeTracking {
    fn default() -> Self {
        Self::Untracked
    }
}

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}

impl ErrorResponse {
    pub fn single<S: Into<String>>(err: S) -> Self {
        Self {
            errors: vec![err.into()],
        }
    }
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: ProjectCategory,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub time_tracking: TimeTracking,
}

impl Project {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Issue {
    pub id: EpicId,
    pub title: String,
    pub issue_type: IssueType,
    pub priority: IssuePriority,
    pub list_position: ListPosition,
    pub description: Option<DescriptionString>,
    pub description_text: Option<DescriptionString>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub reporter_id: UserId,
    pub project_id: ProjectId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub issue_status_id: IssueStatusId,
    pub epic_id: Option<EpicId>,

    pub user_ids: Vec<i32>,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct IssueStatus {
    pub id: IssueStatusId,
    pub name: String,
    pub position: ProjectId,
    pub project_id: ProjectId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invitation {
    pub id: InvitationId,
    pub name: String,
    pub email: String,
    pub state: InvitationState,
    pub project_id: ProjectId,
    pub invited_by_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub bind_token: Uuid,
    pub role: UserRole,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub id: CommentId,
    pub body: String,
    pub user_id: UserId,
    pub issue_id: EpicId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserProject {
    pub id: UserProjectId,
    pub user_id: UserId,
    pub project_id: ProjectId,
    pub is_default: bool,
    pub is_current: bool,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Token {
    pub id: TokenId,
    pub user_id: UserId,
    pub access_token: Uuid,
    pub refresh_token: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub bind_token: Option<Uuid>,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IssueAssignee {
    pub id: i32,
    pub issue_id: EpicId,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression, EnumSql))]
#[cfg_attr(feature = "backend", sql_type = "MessageTypeType")]
#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash, EnumPrimitive,
)]
pub enum MessageType {
    ReceivedInvitation,
    AssignedToIssue,
    Mention,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Mention
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_label())
    }
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: MessageId,
    pub receiver_id: UserId,
    pub sender_id: UserId,
    pub summary: String,
    pub description: String,
    pub message_type: MessageType,
    pub hyper_link: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Epic {
    pub id: EpicId,
    pub name: NameString,
    pub user_id: UserId,
    pub project_id: ProjectId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub starts_at: Option<NaiveDateTime>,
    pub ends_at: Option<NaiveDateTime>,
    pub description: Option<DescriptionString>,
    pub description_html: Option<DescriptionString>,
}

pub type FontStyle = u8;

pub static BOLD: FontStyle = 1;
pub static UNDERLINE: FontStyle = 2;
pub static ITALIC: FontStyle = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha (transparency) component
    pub a: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Style {
    /// Foreground color
    pub foreground: Color,
    /// Background color
    pub background: Color,
    /// Style of the font
    pub font_style: FontStyle,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HighlightedCode {
    pub parts: Vec<(Style, String)>,
}
