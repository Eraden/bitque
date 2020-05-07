use futures::executor::block_on;

use jirs_data::{IssueStatusId, Position, TitleString, WsMsg};

use crate::db::issue_statuses;
use crate::ws::{WebSocketActor, WsHandler, WsResult};

pub struct LoadIssueStatuses;

impl WsHandler<LoadIssueStatuses> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadIssueStatuses, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user()?.project_id;

        let msg = match block_on(
            self.db
                .send(issue_statuses::LoadIssueStatuses { project_id }),
        ) {
            Ok(Ok(v)) => Some(WsMsg::IssueStatusesResponse(v)),
            _ => None,
        };
        Ok(msg)
    }
}

pub struct CreateIssueStatus {
    pub position: i32,
    pub name: TitleString,
}

impl WsHandler<CreateIssueStatus> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateIssueStatus, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user()?.project_id;

        let CreateIssueStatus { position, name } = msg;
        let msg = match block_on(self.db.send(issue_statuses::CreateIssueStatus {
            project_id,
            position,
            name,
        })) {
            Ok(Ok(is)) => Some(WsMsg::IssueStatusCreated(is)),
            _ => None,
        };
        Ok(msg)
    }
}

pub struct DeleteIssueStatus {
    pub issue_status_id: IssueStatusId,
}

impl WsHandler<DeleteIssueStatus> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteIssueStatus, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user()?.project_id;

        let DeleteIssueStatus { issue_status_id } = msg;
        let msg = match block_on(self.db.send(issue_statuses::DeleteIssueStatus {
            issue_status_id,
            project_id,
        })) {
            Ok(Ok(is)) => Some(WsMsg::IssueStatusDeleted(is)),
            _ => None,
        };
        Ok(msg)
    }
}

pub struct UpdateIssueStatus {
    pub issue_status_id: IssueStatusId,
    pub position: Position,
    pub name: TitleString,
}

impl WsHandler<UpdateIssueStatus> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateIssueStatus, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user()?.project_id;

        let UpdateIssueStatus {
            issue_status_id,
            position,
            name,
        } = msg;
        let msg = match block_on(self.db.send(issue_statuses::UpdateIssueStatus {
            issue_status_id,
            position,
            name,
            project_id,
        })) {
            Ok(Ok(is)) => Some(WsMsg::IssueStatusUpdated(is)),
            _ => None,
        };
        Ok(msg)
    }
}
