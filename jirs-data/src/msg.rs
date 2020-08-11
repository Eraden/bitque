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
}
