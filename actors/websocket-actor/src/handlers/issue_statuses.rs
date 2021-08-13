use database_actor::issue_statuses;
use jirs_data::msg::WsMsgIssueStatus;
use jirs_data::{IssueStatusId, Position, TitleString, WsMsg};

use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgIssueStatus> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgIssueStatus) -> WsResult {
        match msg {
            WsMsgIssueStatus::IssueStatusesLoad => self.exec(LoadIssueStatuses).await,
            WsMsgIssueStatus::IssueStatusDelete(issue_status_id) => {
                self.exec(DeleteIssueStatus { issue_status_id }).await
            }
            WsMsgIssueStatus::IssueStatusUpdate(issue_status_id, name, position) => {
                self.exec(UpdateIssueStatus {
                    issue_status_id,
                    position,
                    name,
                })
                .await
            }
            WsMsgIssueStatus::IssueStatusCreate(name, position) => {
                self.exec(CreateIssueStatus { position, name }).await
            }
            WsMsgIssueStatus::IssueStatusesLoaded(_) => Ok(None),
            WsMsgIssueStatus::IssueStatusUpdated(_) => Ok(None),
            WsMsgIssueStatus::IssueStatusCreated(_) => Ok(None),
            WsMsgIssueStatus::IssueStatusDeleted(_, _) => Ok(None),
        }
    }
}

pub struct LoadIssueStatuses;

#[async_trait::async_trait]
impl AsyncHandler<LoadIssueStatuses> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadIssueStatuses) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let v =
            db_or_debug_and_return!(self, issue_statuses::LoadIssueStatuses { project_id }; async);
        Ok(Some(WsMsg::IssueStatus(
            WsMsgIssueStatus::IssueStatusesLoaded(v),
        )))
    }
}

pub struct CreateIssueStatus {
    pub position: i32,
    pub name: TitleString,
}

#[async_trait::async_trait]
impl AsyncHandler<CreateIssueStatus> for WebSocketActor {
    async fn exec(&mut self, msg: CreateIssueStatus) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let CreateIssueStatus { position, name } = msg;
        let issue_status = db_or_debug_and_return!(
            self,
            issue_statuses::CreateIssueStatus {
                project_id,
                position,
                name,
            }; async
        );
        Ok(Some(WsMsg::IssueStatus(
            WsMsgIssueStatus::IssueStatusCreated(issue_status),
        )))
    }
}

pub struct DeleteIssueStatus {
    pub issue_status_id: IssueStatusId,
}

#[async_trait::async_trait]
impl AsyncHandler<DeleteIssueStatus> for WebSocketActor {
    async fn exec(&mut self, msg: DeleteIssueStatus) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let DeleteIssueStatus { issue_status_id } = msg;
        let n = db_or_debug_and_return!(
            self,
            issue_statuses::DeleteIssueStatus {
                project_id,
                issue_status_id
            }; async
        );
        Ok(Some(WsMsg::IssueStatus(
            WsMsgIssueStatus::IssueStatusDeleted(msg.issue_status_id, n),
        )))
    }
}

pub struct UpdateIssueStatus {
    pub issue_status_id: IssueStatusId,
    pub position: Position,
    pub name: TitleString,
}

#[async_trait::async_trait]
impl AsyncHandler<UpdateIssueStatus> for WebSocketActor {
    async fn exec(&mut self, msg: UpdateIssueStatus) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let UpdateIssueStatus {
            issue_status_id,
            position,
            name,
        } = msg;
        let issue_status = db_or_debug_and_return!(
            self,
            issue_statuses::UpdateIssueStatus {
                issue_status_id,
                project_id,
                position,
                name
            }; async
        );
        let msg = Some(WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusUpdated(
            issue_status,
        )));
        if let Some(ws_msg) = msg.as_ref() {
            self.broadcast(ws_msg)
        }
        Ok(msg)
    }
}
