use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum ProjectFieldId {
    Name,
    Url,
    Description,
    DescriptionMode,
    Category,
    TimeTracking,
    IssueStatusName,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum SignInFieldId {
    Username,
    Email,
    Token,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum SignUpFieldId {
    Username,
    Email,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum UsersFieldId {
    Username,
    Email,
    UserRole,
    Avatar,
    CurrentProject,
    TextEditorMode,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum InviteFieldId {
    Token,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum CommentFieldId {
    Body,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum IssueFieldId {
    Type,
    Title,
    Description,
    ListPosition,
    Assignees,
    Reporter,
    Priority,
    Estimate,
    TimeSpent,
    TimeRemaining,
    IssueStatusId,
    EpicName,
    EpicStartsAt,
    EpicEndsAt,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum EpicFieldId {
    Name,
    StartsAt,
    EndsAt,
    TransformInto,
}
