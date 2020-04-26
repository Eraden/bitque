use std::cmp::Ordering;
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

pub trait ToVec {
    type Item;
    fn ordered() -> Vec<Self::Item>;
}

pub type IssueId = i32;
pub type ProjectId = i32;
pub type UserId = i32;
pub type CommentId = i32;
pub type TokenId = i32;
pub type InvitationId = i32;
pub type EmailString = String;
pub type UsernameString = String;

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
        vec![IssueType::Task, IssueType::Bug, IssueType::Story]
    }
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
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Hash)]
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

impl Into<u32> for IssueStatus {
    fn into(self) -> u32 {
        match self {
            IssueStatus::Backlog => 0,
            IssueStatus::Selected => 1,
            IssueStatus::InProgress => 2,
            IssueStatus::Done => 3,
        }
    }
}

impl Into<IssueStatus> for u32 {
    fn into(self) -> IssueStatus {
        match self {
            0 => IssueStatus::Backlog,
            1 => IssueStatus::Selected,
            2 => IssueStatus::InProgress,
            3 => IssueStatus::Done,
            _ => IssueStatus::Backlog,
        }
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

impl ToVec for IssueStatus {
    type Item = IssueStatus;

    fn ordered() -> Vec<Self> {
        vec![
            IssueStatus::Backlog,
            IssueStatus::Selected,
            IssueStatus::InProgress,
            IssueStatus::Done,
        ]
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

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => f.write_str("user"),
            UserRole::Manager => f.write_str("manager"),
            UserRole::Owner => f.write_str("owner"),
        }
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

impl std::fmt::Display for ProjectCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectCategory::Software => f.write_str("software"),
            ProjectCategory::Marketing => f.write_str("marketing"),
            ProjectCategory::Business => f.write_str("business"),
        }
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
    pub status: IssueStatus,
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

    pub user_ids: Vec<i32>,
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
    pub project_id: ProjectId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_role: UserRole,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct UpdateIssuePayload {
    pub title: String,
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub list_position: i32,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: ProjectId,
    pub reporter_id: UserId,
    pub user_ids: Vec<UserId>,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IssueAssignee {
    pub id: i32,
    pub issue_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Issue> for UpdateIssuePayload {
    fn from(issue: Issue) -> Self {
        Self {
            title: issue.title,
            issue_type: issue.issue_type,
            status: issue.status,
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
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateCommentPayload {
    pub user_id: Option<UserId>,
    pub issue_id: IssueId,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UpdateCommentPayload {
    pub id: i32,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateIssuePayload {
    pub title: String,
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: ProjectId,
    pub user_ids: Vec<UserId>,
    pub reporter_id: UserId,
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
    IssueStatus(IssueStatus),
    IssuePriority(IssuePriority),
    ProjectCategory(ProjectCategory),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum ProjectFieldId {
    Name,
    Url,
    Description,
    Category,
    TimeTracking,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum SignInFieldId {
    Username,
    Email,
    Token,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum SignUpFieldId {
    Username,
    Email,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum UsersFieldId {
    Username,
    Email,
    UserRole,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum InviteFieldId {
    Token,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum CommentFieldId {
    Body,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum IssueFieldId {
    Type,
    Title,
    Description,
    Status,
    ListPosition,
    Assignees,
    Reporter,
    Priority,
    Estimate,
    TimeSpent,
    TimeRemaining,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsg {
    Ping,
    Pong,

    // auth
    AuthorizeRequest(Uuid),
    AuthorizeLoaded(Result<User, String>),
    AuthorizeExpired,
    AuthenticateRequest(EmailString, UsernameString),
    AuthenticateSuccess,
    BindTokenCheck(Uuid),
    BindTokenBad,
    BindTokenOk(Uuid),

    // Sign up
    SignUpRequest(EmailString, UsernameString),
    SignUpSuccess,
    SignUpPairTaken,

    // invitations
    InvitationListRequest,
    InvitationListLoaded(Vec<Invitation>),
    //
    InvitedUsersRequest,
    InvitedUsersLoaded(Vec<User>),
    //
    InvitationSendRequest {
        name: UsernameString,
        email: EmailString,
    },
    InvitationSendSuccess,
    InvitationSendFailure,
    //
    InvitationRevokeRequest(InvitationId),
    InvitationRevokeSuccess(InvitationId),
    //
    InvitationAcceptRequest(InvitationId),
    InvitationAcceptSuccess(InvitationId),
    InvitedUserRemoveRequest(EmailString),
    InvitedUserRemoveSuccess(EmailString),

    // project page
    ProjectRequest,
    ProjectLoaded(Project),
    ProjectIssuesRequest,
    ProjectIssuesLoaded(Vec<Issue>),
    ProjectUsersRequest,
    ProjectUsersLoaded(Vec<User>),
    ProjectUpdateRequest(UpdateProjectPayload),

    // issue
    IssueUpdateRequest(IssueId, IssueFieldId, PayloadVariant),
    IssueUpdated(Issue),
    IssueDeleteRequest(IssueId),
    IssueDeleted(IssueId),
    IssueCreateRequest(CreateIssuePayload),
    IssueCreated(Issue),

    // comments
    IssueCommentsRequest(IssueId),
    IssueCommentsLoaded(Vec<Comment>),
    CreateComment(CreateCommentPayload),
    UpdateComment(UpdateCommentPayload),
    CommentDeleteRequest(CommentId),
    CommentDeleted(CommentId),
}
