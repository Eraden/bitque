use crate::FieldId;
use jirs_data::{IssueId, IssueStatusId};

use crate::shared::styled_editor::Mode as TabMode;
use seed::prelude::WebSocketMessage;

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
pub enum PageChanged {
    Users(UsersPageChange),
    ProjectSettings(ProjectPageChange),
    Profile(ProfilePageChange),
    Board(BoardPageChange),
}

#[derive(Debug)]
pub enum WebSocketChanged {
    WsMsg(jirs_data::WsMsg),
    WebSocketMessage(WebSocketMessage),
    WebSocketMessageLoaded(Vec<u8>),
    WebSocketOpened,
    WebSocketClosed,
    SendPing,
}
