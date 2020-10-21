use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    BindToken, Comment, CommentId, CreateCommentPayload, CreateIssuePayload, EmailString, Epic,
    EpicId, Invitation, InvitationId, InvitationToken, Issue, IssueFieldId, IssueId, IssueStatus,
    IssueStatusId, Message, MessageId, NameString, PayloadVariant, Position, Project, TitleString,
    UpdateCommentPayload, UpdateProjectPayload, User, UserId, UserProject, UserProjectId, UserRole,
    UsernameString,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[repr(C)]
pub enum WsError {
    InvalidLoginPair,
    InvalidSignInToken,

    // Issue status
    NoIssueStatuses,
    FailedToFetchIssueStatuses,

    // tokens
    FailedToDisableBindToken,
    BindTokenNotExists,
    NoBindToken,
    FailedToCreateBindToken,
    AccessTokenNotExists,

    // users
    UserNotExists(UserId),
    NoMatchingPair(UsernameString, EmailString),
    InvalidPair(UsernameString, EmailString),
    TakenPair(UsernameString, EmailString),
    FailedToLoadProjectUsers,
    FailedToLoadAssignees,
    FailedToChangeAvatar,
    FailedToLoadInvitedUsers,

    // user projects
    InvalidUserProject,

    // comments
    FailedToLoadComments,
    InvalidComment,
    FailedToUpdateComment,
    UnableToDeleteComment,

    // epics
    FailedToLoadEpics,
    InvalidEpic,
    FailedToUpdateEpic,
    UnableToDeleteEpic,

    // invitations
    FailedToLoadInvitations,
    InvalidInvitation,
    FailedToUpdateInvitation,
    UnableToDeleteInvitation,
    InvitationRevoked,
}

impl WsError {
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            WsError::InvalidLoginPair => "E-Mail and Login does not match",
            WsError::InvalidSignInToken => "Given token is not valid",
            WsError::NoIssueStatuses => {
                "Failed to fetch first issue status. Are you sure there is any?"
            }
            WsError::FailedToFetchIssueStatuses => "Failed to load issue statuses",
            WsError::FailedToDisableBindToken => "Failed to disable one use token",
            WsError::BindTokenNotExists => "Used single use bind token does not exists in database",
            WsError::NoBindToken => "Current user does not have any active tokens",
            WsError::FailedToCreateBindToken => {
                "Something went wrong when creating bind token. Please try later"
            }
            WsError::AccessTokenNotExists => "Token used for authentication does not exists",
            WsError::UserNotExists(_) => "User does not exists",
            WsError::NoMatchingPair(_, _) => "User for given pair does not exists",
            WsError::FailedToLoadProjectUsers => {
                "There was problem while loading project users. Please try later"
            }
            WsError::FailedToLoadAssignees => {
                "There was problem while loading issue assignees. Please try later"
            }
            WsError::InvalidPair(_, _) => "Given sign up pair is not valid.",
            WsError::TakenPair(_, _) => "Given sign up pair is already taken.",
            WsError::InvalidUserProject => "Unable to connect user to project",
            WsError::FailedToChangeAvatar => "Unable to change user avatar",
            WsError::FailedToLoadInvitedUsers => "Failed to load invited users. Please try later",

            // comments
            WsError::FailedToLoadComments => "Failed to load comments. Please try later",
            WsError::InvalidComment => "There is something wrong with given comment data",
            WsError::FailedToUpdateComment => {
                "There was problem when updating comment. Please try later"
            }
            WsError::UnableToDeleteComment => "Unable to delete comment",

            // epics
            WsError::FailedToLoadEpics => "Failed to load epics. Please try later",
            WsError::InvalidEpic => "There is something wrong with given epic data",
            WsError::FailedToUpdateEpic => {
                "There was problem when updating comment. Please try later"
            }
            WsError::UnableToDeleteEpic => "Unable to delete epic",

            // invitations
            WsError::InvalidInvitation => "Given invitation contains problems",
            WsError::FailedToLoadInvitations => "Failed to load invitations. Please try later",
            WsError::FailedToUpdateInvitation => {
                "There was problem when updating invitation. Please try later"
            }
            WsError::UnableToDeleteInvitation => "Unable to delete invitation",
            WsError::InvitationRevoked => "This invitation is no longer valid",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsg {
    Ping,
    Pong,
    Die,

    // auth
    AuthorizeLoad(Uuid),
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
    InvitationListLoad,
    InvitationListLoaded(Vec<Invitation>),
    //
    InvitedUsersLoad,
    InvitedUsersLoaded(Vec<User>),
    //
    InvitationSendRequest {
        name: UsernameString,
        email: EmailString,
        role: UserRole,
    },
    InvitationSendSuccess,
    InvitationSendFailure,
    //
    InvitationRevokeRequest(InvitationId),
    InvitationRevokeSuccess(InvitationId),
    //
    InvitationAcceptRequest(InvitationToken),
    InvitationAcceptSuccess(BindToken),
    InvitationAcceptFailure(InvitationToken),
    //
    InvitationRejectRequest(InvitationToken),
    InvitationRejectSuccess,
    InvitationRejectFailure(InvitationToken),
    //
    InvitedUserRemoveRequest(UserId),
    InvitedUserRemoveSuccess(UserId),

    // project page
    ProjectsLoad,
    ProjectsLoaded(Vec<Project>),

    ProjectIssuesLoad,
    ProjectIssuesLoaded(Vec<Issue>),
    ProjectUsersLoad,
    ProjectUsersLoaded(Vec<User>),
    ProjectUpdateLoad(UpdateProjectPayload),

    // issue
    IssueUpdate(IssueId, IssueFieldId, PayloadVariant),
    IssueUpdated(Issue),
    IssueDelete(IssueId),
    IssueDeleted(IssueId),
    IssueCreate(CreateIssuePayload),
    IssueCreated(Issue),

    // issue status
    IssueStatusesLoad,
    IssueStatusesLoaded(Vec<IssueStatus>),
    IssueStatusUpdate(IssueStatusId, TitleString, Position),
    IssueStatusUpdated(IssueStatus),
    IssueStatusCreate(TitleString, Position),
    IssueStatusCreated(IssueStatus),
    IssueStatusDelete(IssueStatusId),
    IssueStatusDeleted(IssueStatusId),

    // comments
    IssueCommentsLoad(IssueId),
    IssueCommentsLoaded(Vec<Comment>),
    CommentCreate(CreateCommentPayload),
    CommentCreated(Comment),
    CommentUpdate(UpdateCommentPayload),
    CommentUpdated(Comment),
    CommentDelete(CommentId),
    CommentDeleted(CommentId),

    // users
    AvatarUrlChanged(UserId, String),
    ProfileUpdate(EmailString, UsernameString),
    ProfileUpdated,

    // user projects
    UserProjectsLoad,
    UserProjectsLoaded(Vec<UserProject>),
    UserProjectSetCurrent(UserProjectId),
    UserProjectCurrentChanged(UserProject),

    // messages
    Message(Message),
    MessagesLoad,
    MessagesLoaded(Vec<Message>),
    MessageMarkSeen(MessageId),
    MessageMarkedSeen(MessageId),

    // epics
    EpicsLoad,
    EpicsLoaded(Vec<Epic>),
    EpicCreate(NameString),
    EpicCreated(Epic),
    EpicUpdate(EpicId, NameString),
    EpicUpdated(Epic),
    EpicDelete(EpicId),
    EpicDeleted(EpicId),

    // errors
    Error(WsError),
}
