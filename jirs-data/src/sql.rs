use std::io::Write;

use diesel::{deserialize::*, pg::*, serialize::*, *};

use crate::{
    InvitationState, IssuePriority, IssueStatus, IssueType, ProjectCategory, TimeTracking, UserRole,
};

#[derive(SqlType)]
#[postgres(type_name = "IssuePriorityType")]
pub struct IssuePriorityType;

impl ToSql<IssuePriorityType, Pg> for IssuePriority {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            IssuePriority::Highest => out.write_all(b"highest")?,
            IssuePriority::High => out.write_all(b"high")?,
            IssuePriority::Medium => out.write_all(b"medium")?,
            IssuePriority::Low => out.write_all(b"low")?,
            IssuePriority::Lowest => out.write_all(b"lowest")?,
        }
        Ok(IsNull::No)
    }
}

fn issue_priority_from_sql(bytes: Option<&[u8]>) -> deserialize::Result<IssuePriority> {
    match not_none!(bytes) {
        b"5" | b"highest" => Ok(IssuePriority::Highest),
        b"4" | b"high" => Ok(IssuePriority::High),
        b"3" | b"medium" => Ok(IssuePriority::Medium),
        b"2" | b"low" => Ok(IssuePriority::Low),
        b"1" | b"lowest" => Ok(IssuePriority::Lowest),
        _ => Ok(IssuePriority::Lowest),
    }
}

impl FromSql<IssuePriorityType, Pg> for IssuePriority {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        issue_priority_from_sql(bytes)
    }
}

impl FromSql<sql_types::Text, Pg> for IssuePriority {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        issue_priority_from_sql(bytes)
    }
}

#[derive(SqlType)]
#[postgres(type_name = "IssueTypeType")]
pub struct IssueTypeType;

fn issue_type_from_sql(bytes: Option<&[u8]>) -> deserialize::Result<IssueType> {
    match not_none!(bytes) {
        b"task" => Ok(IssueType::Task),
        b"bug" => Ok(IssueType::Bug),
        b"story" => Ok(IssueType::Story),
        _ => Ok(IssueType::Task),
    }
}

impl FromSql<IssueTypeType, Pg> for IssueType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        issue_type_from_sql(bytes)
    }
}

impl FromSql<sql_types::Text, Pg> for IssueType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        issue_type_from_sql(bytes)
    }
}

impl ToSql<IssueTypeType, Pg> for IssueType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            IssueType::Task => out.write_all(b"task")?,
            IssueType::Story => out.write_all(b"story")?,
            IssueType::Bug => out.write_all(b"bug")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(SqlType)]
#[postgres(type_name = "IssueStatusType")]
pub struct IssueStatusType;

impl diesel::query_builder::QueryId for IssueStatusType {
    type QueryId = IssueStatus;
}

fn issue_status_from_sql(bytes: Option<&[u8]>) -> deserialize::Result<IssueStatus> {
    match not_none!(bytes) {
        b"backlog" => Ok(IssueStatus::Backlog),
        b"selected" => Ok(IssueStatus::Selected),
        b"in_progress" | b"inprogress" => Ok(IssueStatus::InProgress),
        b"done" => Ok(IssueStatus::Done),
        _ => Ok(IssueStatus::Backlog),
    }
}

impl FromSql<IssueStatusType, Pg> for IssueStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        issue_status_from_sql(bytes)
    }
}

impl FromSql<sql_types::Text, Pg> for IssueStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        issue_status_from_sql(bytes)
    }
}

impl ToSql<IssueStatusType, Pg> for IssueStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            IssueStatus::Backlog => out.write_all(b"backlog")?,
            IssueStatus::Selected => out.write_all(b"selected")?,
            IssueStatus::InProgress => out.write_all(b"in_progress")?,
            IssueStatus::Done => out.write_all(b"done")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(SqlType)]
#[postgres(type_name = "ProjectCategoryType")]
pub struct ProjectCategoryType;

impl diesel::query_builder::QueryId for ProjectCategoryType {
    type QueryId = ProjectCategory;
}

fn project_category_from_sql(bytes: Option<&[u8]>) -> deserialize::Result<ProjectCategory> {
    match not_none!(bytes) {
        b"software" => Ok(ProjectCategory::Software),
        b"marketing" => Ok(ProjectCategory::Marketing),
        b"business" => Ok(ProjectCategory::Business),
        _ => Ok(ProjectCategory::Software),
    }
}

impl FromSql<ProjectCategoryType, Pg> for ProjectCategory {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        project_category_from_sql(bytes)
    }
}

impl FromSql<sql_types::Text, Pg> for ProjectCategory {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        project_category_from_sql(bytes)
    }
}

impl ToSql<ProjectCategoryType, Pg> for ProjectCategory {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            ProjectCategory::Software => out.write_all(b"software")?,
            ProjectCategory::Marketing => out.write_all(b"marketing")?,
            ProjectCategory::Business => out.write_all(b"business")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(SqlType)]
#[postgres(type_name = "UserRoleType")]
pub struct UserRoleType;

impl diesel::query_builder::QueryId for UserRoleType {
    type QueryId = UserRole;
}

fn user_role_from_sql(bytes: Option<&[u8]>) -> deserialize::Result<UserRole> {
    match not_none!(bytes) {
        b"user" => Ok(UserRole::User),
        b"manager" => Ok(UserRole::Manager),
        b"owner" => Ok(UserRole::Owner),
        _ => Ok(UserRole::User),
    }
}

impl FromSql<UserRoleType, Pg> for UserRole {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        user_role_from_sql(bytes)
    }
}

impl FromSql<sql_types::Text, Pg> for UserRole {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        user_role_from_sql(bytes)
    }
}

impl ToSql<UserRoleType, Pg> for UserRole {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            UserRole::User => out.write_all(b"user")?,
            UserRole::Manager => out.write_all(b"manager")?,
            UserRole::Owner => out.write_all(b"owner")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(SqlType)]
#[postgres(type_name = "InvitationStateType")]
pub struct InvitationStateType;

impl diesel::query_builder::QueryId for InvitationStateType {
    type QueryId = InvitationState;
}

fn invitation_state_from_sql(bytes: Option<&[u8]>) -> deserialize::Result<InvitationState> {
    match not_none!(bytes) {
        b"sent" => Ok(InvitationState::Sent),
        b"accepted" => Ok(InvitationState::Accepted),
        b"revoked" => Ok(InvitationState::Revoked),
        _ => Ok(InvitationState::Sent),
    }
}

impl FromSql<InvitationStateType, Pg> for InvitationState {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        invitation_state_from_sql(bytes)
    }
}

impl FromSql<sql_types::Text, Pg> for InvitationState {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        invitation_state_from_sql(bytes)
    }
}

impl ToSql<InvitationStateType, Pg> for InvitationState {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            InvitationState::Sent => out.write_all(b"sent")?,
            InvitationState::Accepted => out.write_all(b"accepted")?,
            InvitationState::Revoked => out.write_all(b"revoked")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(SqlType)]
#[postgres(type_name = "TimeTrackingType")]
pub struct TimeTrackingType;

impl diesel::query_builder::QueryId for TimeTrackingType {
    type QueryId = TimeTracking;
}

fn time_tracking_from_sql(bytes: Option<&[u8]>) -> deserialize::Result<TimeTracking> {
    match not_none!(bytes) {
        b"untracked" => Ok(TimeTracking::Untracked),
        b"fibnachi" => Ok(TimeTracking::Fibonacci),
        b"hourly" => Ok(TimeTracking::Hourly),
        _ => Ok(TimeTracking::Untracked),
    }
}

impl FromSql<TimeTrackingType, Pg> for TimeTracking {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<TimeTracking> {
        time_tracking_from_sql(bytes)
    }
}

impl FromSql<sql_types::Text, Pg> for TimeTracking {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<TimeTracking> {
        time_tracking_from_sql(bytes)
    }
}

impl ToSql<TimeTrackingType, Pg> for TimeTracking {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            TimeTracking::Untracked => out.write_all(b"untracked")?,
            TimeTracking::Fibonacci => out.write_all(b"fibnacci")?,
            TimeTracking::Hourly => out.write_all(b"hourly")?,
        }
        Ok(IsNull::No)
    }
}
