use seed::prelude::WebSocketMessage;

use jirs_data::{IssueId, IssueStatusId, WsMsg};

use crate::shared::styled_editor::Mode as TabMode;
use crate::FieldId;

#[derive(Clone, Debug, PartialEq)]
pub enum FieldChange {
    LinkCopied(FieldId, bool),
    TabChanged(FieldId, TabMode),
    ToggleCommentForm(FieldId, bool),
    EditComment(FieldId, i32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoardPageChange {
    // dragging
    IssueDragStarted(IssueId),
    IssueDragStopped(IssueId),
    DragLeave(IssueId),
    ExchangePosition(IssueId),
    IssueDragOverStatus(IssueStatusId),
    IssueDropZone(IssueStatusId),
}

#[derive(Clone, Debug, PartialEq)]
pub enum UsersPageChange {
    ResetForm,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectPageChange {
    ResetForm,
    SubmitProjectSettingsForm,
    // dragging
    ColumnDragStarted(IssueStatusId),
    ColumnDragStopped(IssueStatusId),
    ColumnDragLeave(IssueStatusId),
    ColumnExchangePosition(IssueStatusId),
    ColumnDragOverStatus(IssueStatusId),
    ColumnDropZone(IssueStatusId),
    // edit issue status name
    EditIssueStatusName(Option<IssueStatusId>),
    SubmitIssueStatusForm,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProfilePageChange {
    SubmitForm,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InvitationPageChange {
    SubmitForm,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReportsPageChange {
    DayHovered(Option<chrono::NaiveDate>),
    DaySelected(Option<chrono::NaiveDate>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PageChanged {
    Users(UsersPageChange),
    ProjectSettings(ProjectPageChange),
    Profile(ProfilePageChange),
    Board(BoardPageChange),
    Invitation(InvitationPageChange),
    Reports(ReportsPageChange),
}

#[derive(Debug)]
pub enum WebSocketChanged {
    WsMsg(WsMsg),
    WebSocketMessage(WebSocketMessage),
    WebSocketMessageLoaded(Vec<u8>),
    WebSocketOpened,
    WebSocketClosed,
    SendPing,
    Bounced(WsMsg),
}
