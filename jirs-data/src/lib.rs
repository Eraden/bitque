use std::cmp::Ordering;
use std::str::FromStr;

use chrono::NaiveDateTime;
#[cfg(feature = "backend")]
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use fields::*;
pub use msg::WsMsg;
pub use payloads::*;
#[cfg(feature = "backend")]
pub use sql::*;

mod fields;
pub mod msg;
mod payloads;

#[cfg(feature = "backend")]
pub mod sql;

pub trait ToVec {
    type Item;
    fn ordered() -> Vec<Self::Item>;
}

pub type IssueId = i32;
pub type ProjectId = i32;
pub type UserId = i32;
pub type UserProjectId = i32;
pub type CommentId = i32;
pub type TokenId = i32;
pub type IssueStatusId = i32;
pub type InvitationId = i32;
pub type Position = i32;
pub type MessageId = i32;
pub type EpicId = i32;

pub type EmailString = String;
pub type UsernameString = String;
pub type TitleString = String;
pub type NameString = String;

pub type BindToken = Uuid;
pub type InvitationToken = Uuid;

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "IssueTypeType")]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash)]
pub enum IssueType {
    Task,
    Bug,
    Story,
}

impl ToVec for IssueType {
    type Item = IssueType;

    fn ordered() -> Vec<Self> {
        use IssueType::*;
        vec![Task, Bug, Story]
    }
}

impl Default for IssueType {
    fn default() -> Self {
        IssueType::Task
    }
}

impl IssueType {
    pub fn to_label(&self) -> &str {
        use IssueType::*;
        match self {
            Task => "Task",
            Bug => "Bug",
            Story => "Story",
        }
    }
}

#[cfg(feature = "frontend")]
impl Into<u32> for IssueType {
    fn into(self) -> u32 {
        match self {
            IssueType::Task => 1,
            IssueType::Bug => 2,
            IssueType::Story => 3,
        }
    }
}

#[cfg(feature = "frontend")]
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

impl IssueType {
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            IssueType::Task => "task",
            IssueType::Bug => "bug",
            IssueType::Story => "story",
        }
    }
}

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "IssuePriorityType")]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash)]
pub enum IssuePriority {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
}

impl ToVec for IssuePriority {
    type Item = IssuePriority;

    fn ordered() -> Vec<Self> {
        vec![
            IssuePriority::Highest,
            IssuePriority::High,
            IssuePriority::Medium,
            IssuePriority::Low,
            IssuePriority::Lowest,
        ]
    }
}

impl FromStr for IssuePriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "highest" => Ok(IssuePriority::Highest),
            "high" => Ok(IssuePriority::High),
            "medium" => Ok(IssuePriority::Medium),
            "low" => Ok(IssuePriority::Low),
            "lowest" => Ok(IssuePriority::Lowest),
            _ => Err(format!("Unknown priority {}", s)),
        }
    }
}

impl Default for IssuePriority {
    fn default() -> Self {
        IssuePriority::Medium
    }
}

impl IssuePriority {
    pub fn to_str(&self) -> &'static str {
        match self {
            IssuePriority::Highest => "highest",
            IssuePriority::High => "high",
            IssuePriority::Medium => "medium",
            IssuePriority::Low => "low",
            IssuePriority::Lowest => "lowest",
        }
    }
}

impl std::fmt::Display for IssuePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl Into<u32> for IssuePriority {
    fn into(self) -> u32 {
        match self {
            IssuePriority::Highest => 5,
            IssuePriority::High => 4,
            IssuePriority::Medium => 3,
            IssuePriority::Low => 2,
            IssuePriority::Lowest => 1,
        }
    }
}

impl Into<IssuePriority> for u32 {
    fn into(self) -> IssuePriority {
        match self {
            5 => IssuePriority::Highest,
            4 => IssuePriority::High,
            3 => IssuePriority::Medium,
            2 => IssuePriority::Low,
            1 => IssuePriority::Lowest,
            _ => IssuePriority::Medium,
        }
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "UserRoleType")]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Hash)]
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

impl ToVec for UserRole {
    type Item = UserRole;

    fn ordered() -> Vec<Self::Item> {
        vec![UserRole::User, UserRole::Manager, UserRole::Owner]
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "user" => Ok(UserRole::User),
            "manager" => Ok(UserRole::Manager),
            "owner" => Ok(UserRole::Owner),
            _ => Err(format!("Unknown user role {}", s)),
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

impl UserRole {
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            UserRole::User => "user",
            UserRole::Manager => "manager",
            UserRole::Owner => "owner",
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl Into<u32> for UserRole {
    fn into(self) -> u32 {
        match self {
            UserRole::User => 0,
            UserRole::Manager => 1,
            UserRole::Owner => 2,
        }
    }
}

impl Into<UserRole> for u32 {
    fn into(self) -> UserRole {
        match self {
            0 => UserRole::User,
            1 => UserRole::Manager,
            2 => UserRole::Owner,
            _ => UserRole::User,
        }
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "ProjectCategoryType")]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash)]
pub enum ProjectCategory {
    Software,
    Marketing,
    Business,
}

impl ToVec for ProjectCategory {
    type Item = ProjectCategory;

    fn ordered() -> Vec<Self> {
        vec![
            ProjectCategory::Software,
            ProjectCategory::Marketing,
            ProjectCategory::Business,
        ]
    }
}

impl FromStr for ProjectCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "software" => Ok(ProjectCategory::Software),
            "marketing" => Ok(ProjectCategory::Marketing),
            "business" => Ok(ProjectCategory::Business),
            _ => Err(format!("Unknown project category {}", s)),
        }
    }
}

impl Default for ProjectCategory {
    fn default() -> Self {
        ProjectCategory::Software
    }
}

impl ProjectCategory {
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            ProjectCategory::Software => "software",
            ProjectCategory::Marketing => "marketing",
            ProjectCategory::Business => "business",
        }
    }
}

impl std::fmt::Display for ProjectCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl Into<u32> for ProjectCategory {
    fn into(self) -> u32 {
        match self {
            ProjectCategory::Software => 0,
            ProjectCategory::Marketing => 1,
            ProjectCategory::Business => 2,
        }
    }
}

impl Into<ProjectCategory> for u32 {
    fn into(self) -> ProjectCategory {
        match self {
            0 => ProjectCategory::Software,
            1 => ProjectCategory::Marketing,
            2 => ProjectCategory::Business,
            _ => ProjectCategory::Software,
        }
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "InvitationStateType")]
#[derive(Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash)]
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
        match self {
            InvitationState::Sent => f.write_str("sent"),
            InvitationState::Accepted => f.write_str("accepted"),
            InvitationState::Revoked => f.write_str("revoked"),
        }
    }
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "TimeTrackingType")]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash)]
pub enum TimeTracking {
    Untracked,
    Fibonacci,
    Hourly,
}

impl Into<u32> for TimeTracking {
    fn into(self) -> u32 {
        match self {
            TimeTracking::Untracked => 0,
            TimeTracking::Fibonacci => 1,
            TimeTracking::Hourly => 2,
        }
    }
}

impl Into<TimeTracking> for u32 {
    fn into(self) -> TimeTracking {
        match self {
            0 => TimeTracking::Untracked,
            1 => TimeTracking::Fibonacci,
            2 => TimeTracking::Hourly,
            _ => TimeTracking::Untracked,
        }
    }
}

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Issue {
    pub id: IssueId,
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
    pub id: i32,
    pub name: String,
    pub email: String,
    pub state: InvitationState,
    pub project_id: i32,
    pub invited_by_id: i32,
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
    pub issue_id: IssueId,
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
    pub issue_id: IssueId,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", sql_type = "MessageTypeType")]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash)]
pub enum MessageType {
    ReceivedInvitation,
    AssignedToIssue,
    Mention,
}

impl Into<u32> for MessageType {
    fn into(self) -> u32 {
        match self {
            MessageType::ReceivedInvitation => 0,
            MessageType::AssignedToIssue => 1,
            MessageType::Mention => 2,
        }
    }
}

impl Into<MessageType> for u32 {
    fn into(self) -> MessageType {
        match self {
            0 => MessageType::ReceivedInvitation,
            1 => MessageType::AssignedToIssue,
            2 => MessageType::Mention,
            _ => MessageType::Mention,
        }
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::ReceivedInvitation => f.write_str("ReceivedInvitation"),
            MessageType::AssignedToIssue => f.write_str("AssignedToIssue"),
            MessageType::Mention => f.write_str("Mention"),
        }
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
}
