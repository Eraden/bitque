use database_actor::issue_statuses;
use futures::executor::block_on;
use jirs_data::{IssueStatusId, Position, TitleString, WsMsg};

use crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

pub struct LoadIssueStatuses;

impl WsHandler<LoadIssueStatuses> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadIssueStatuses, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let v = db_or_debug_and_return!(self, issue_statuses::LoadIssueStatuses { project_id });
        Ok(Some(WsMsg::IssueStatusesLoaded(v)))
    }
}

pub struct CreateIssueStatus {
    pub position: i32,
    pub name: TitleString,
}

impl WsHandler<CreateIssueStatus> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateIssueStatus, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let CreateIssueStatus { position, name } = msg;
        let issue_status = db_or_debug_and_return!(
            self,
            issue_statuses::CreateIssueStatus {
                project_id,
                position,
                name,
            }
        );
        Ok(Some(WsMsg::IssueStatusCreated(issue_status)))
    }
}

pub struct DeleteIssueStatus {
    pub issue_status_id: IssueStatusId,
}

impl WsHandler<DeleteIssueStatus> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteIssueStatus, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user_project()?.project_id;

        let DeleteIssueStatus { issue_status_id } = msg;
        let n = db_or_debug_and_return!(
            self,
            issue_statuses::DeleteIssueStatus {
                issue_status_id,
                project_id,
            }
        );
        Ok(Some(WsMsg::IssueStatusDeleted(msg.issue_status_id, n)))
    }
}

pub struct UpdateIssueStatus {
    pub issue_status_id: IssueStatusId,
    pub position: Position,
    pub name: TitleString,
}

impl WsHandler<UpdateIssueStatus> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateIssueStatus, _ctx: &mut Self::Context) -> WsResult {
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
                position,
                name,
                project_id,
            }
        );
        let msg = Some(WsMsg::IssueStatusUpdated(issue_status));
        if let Some(ws_msg) = msg.as_ref() {
            self.broadcast(ws_msg)
        }
        Ok(msg)
    }
}
