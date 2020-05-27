use crate::ws::{WebSocketActor, WsHandler, WsResult};
use futures::executor::block_on;

use crate::db::messages;
use jirs_data::WsMsg;

pub struct LoadMessages;

impl WsHandler<LoadMessages> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadMessages, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        match block_on(self.db.send(messages::LoadMessages { user_id })) {
            Ok(Ok(v)) => Ok(Some(WsMsg::MessagesResponse(v))),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        }
    }
}
