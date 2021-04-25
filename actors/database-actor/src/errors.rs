use jirs_data::{EmailString, UsernameString};

#[derive(Debug)]
pub enum OperationError {
    LoadCollection,
    LoadSingle,
    Create,
    Update,
    Delete,
}

#[derive(Debug)]
pub enum ResourceKind {
    Epic,
    Invitation,
    IssueAssignee,
    IssueStatus,
    Issue,
    Message,
    Project,
    Token,
    UserProject,
    UserSetting,
    User,
    Comment,
}

#[derive(Debug)]
pub enum InvitationError {
    InvitationRevoked,
}

#[derive(Debug)]
pub enum TokenError {
    FailedToDisable,
}

#[derive(Debug)]
pub enum UserError {
    TakenPair(UsernameString, EmailString),
    InvalidPair(UsernameString, EmailString),
    UpdateProfile,
}

#[derive(Debug)]
pub enum IssueError {
    BadListPosition,
    NoIssueStatuses,
}

#[derive(Debug)]
pub enum UserProjectError {
    InviteHimself,
}

#[derive(Debug)]
pub enum DatabaseError {
    DatabaseConnectionLost,
    GenericFailure(OperationError, ResourceKind),
    Invitation(InvitationError),
    Token(TokenError),
    User(UserError),
    Issue(IssueError),
    UserProject(UserProjectError),
}
