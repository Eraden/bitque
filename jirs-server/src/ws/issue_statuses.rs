use futures::executor::block_on;

use jirs_data::WsMsg;

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
