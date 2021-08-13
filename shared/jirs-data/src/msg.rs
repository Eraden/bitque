use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AvatarUrl, BindToken, Code, Comment, CommentId, CreateCommentPayload, CreateIssuePayload,
    DescriptionString, EmailString, EndsAt, Epic, EpicId, HighlightedCode, Invitation,
    InvitationId, InvitationToken, Issue, IssueFieldId, IssueId, IssueStatus, IssueStatusId,
    IssueType, Lang, ListPosition, Message, MessageId, NameString, NumberOfDeleted, PayloadVariant,
    Position, Project, StartsAt, TextEditorMode, TitleString, UpdateCommentPayload,
    UpdateProjectPayload, User, UserId, UserProject, UserProjectId, UserRole, UserSetting,
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
pub enum WsMsgIssue {
    IssueUpdate(IssueId, IssueFieldId, PayloadVariant),
    IssueUpdated(Issue),
    IssueDelete(IssueId),
    IssueDeleted(IssueId, NumberOfDeleted),
    IssueCreate(CreateIssuePayload),
    IssueCreated(Issue),
    IssueSyncListPosition(Vec<(IssueId, ListPosition, IssueStatusId, Option<IssueId>)>),
}

impl From<WsMsgIssue> for WsMsg {
    fn from(msg: WsMsgIssue) -> Self {
        WsMsg::Issue(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgIssueStatus {
    IssueStatusesLoad,
    IssueStatusesLoaded(Vec<IssueStatus>),
    IssueStatusUpdate(IssueStatusId, TitleString, Position),
    IssueStatusUpdated(IssueStatus),
    IssueStatusCreate(TitleString, Position),
    IssueStatusCreated(IssueStatus),
    IssueStatusDelete(IssueStatusId),
    IssueStatusDeleted(IssueStatusId, NumberOfDeleted),
}

impl From<WsMsgIssueStatus> for WsMsg {
    fn from(msg: WsMsgIssueStatus) -> Self {
        WsMsg::IssueStatus(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgComment {
    IssueCommentsLoad(IssueId),
    IssueCommentsLoaded(Vec<Comment>),
    CommentCreate(CreateCommentPayload),
    CommentCreated(Comment),
    CommentUpdate(UpdateCommentPayload),
    CommentUpdated(Comment),
    CommentDelete(CommentId),
    CommentDeleted(CommentId, NumberOfDeleted),
}

impl From<WsMsgComment> for WsMsg {
    fn from(msg: WsMsgComment) -> Self {
        WsMsg::Comment(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgInvitation {
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
}

impl From<WsMsgInvitation> for WsMsg {
    fn from(msg: WsMsgInvitation) -> Self {
        WsMsg::Invitation(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgEpic {
    EpicsLoad,
    EpicsLoaded(Vec<Epic>),
    EpicCreate(
        NameString,
        Option<DescriptionString>,
        Option<DescriptionString>,
    ),
    EpicCreated(Epic),
    EpicUpdateName(EpicId, NameString),
    EpicUpdateStartsAt(EpicId, Option<StartsAt>),
    EpicUpdateEndsAt(EpicId, Option<EndsAt>),
    EpicUpdated(Epic),
    EpicDelete(EpicId),
    EpicDeleted(EpicId, NumberOfDeleted),
    EpicTransform(EpicId, IssueType),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgProject {
    ProjectsLoad,
    ProjectsLoaded(Vec<Project>),

    ProjectIssuesLoad,
    ProjectIssuesLoaded(Vec<Issue>),
    ProjectUsersLoad,
    ProjectUsersLoaded(Vec<User>),
    ProjectUpdateLoad(UpdateProjectPayload),
}

impl From<WsMsgProject> for WsMsg {
    fn from(msg: WsMsgProject) -> Self {
        WsMsg::Project(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgSession {
    // auth
    AuthorizeLoad(Uuid),
    AuthorizeLoaded(Result<(User, UserSetting), String>),
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
}

impl From<WsMsgSession> for WsMsg {
    fn from(msg: WsMsgSession) -> WsMsg {
        WsMsg::Session(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgUser {
    // users
    AvatarUrlChanged(UserId, AvatarUrl),
    ProfileUpdate(EmailString, UsernameString),
    ProfileUpdated,

    // user settings
    UserSettingUpdated(UserSetting),
    UserSettingSetEditorMode(TextEditorMode),
}

impl From<WsMsgUser> for WsMsg {
    fn from(msg: WsMsgUser) -> WsMsg {
        WsMsg::User(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsgMessage {
    // messages
    MessageUpdated(Message),
    MessagesLoad,
    MessagesLoaded(Vec<Message>),
    MessageMarkSeen(MessageId),
    MessageMarkedSeen(MessageId, NumberOfDeleted),
}

impl From<WsMsgMessage> for WsMsg {
    fn from(msg: WsMsgMessage) -> WsMsg {
        WsMsg::Message(msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WsMsg {
    Ping,
    Pong,
    Die,

    Session(WsMsgSession),
    Invitation(WsMsgInvitation),
    Project(WsMsgProject),
    Issue(WsMsgIssue),
    IssueStatus(WsMsgIssueStatus),
    Comment(WsMsgComment),
    User(WsMsgUser),
    Message(WsMsgMessage),
    Epic(WsMsgEpic),

    // user projects
    UserProjectsLoad,
    UserProjectsLoaded(Vec<UserProject>),
    UserProjectSetCurrent(UserProjectId),
    UserProjectCurrentChanged(UserProject),

    // highlight
    HighlightCode(Lang, Code),
    HighlightedCode(HighlightedCode),

    // errors
    Error(WsError),
}
