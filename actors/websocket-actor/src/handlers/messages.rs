use database_actor::messages;
use jirs_data::{MessageId, WsMsg};

use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

pub struct LoadMessages;

#[async_trait::async_trait]
impl AsyncHandler<LoadMessages> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadMessages) -> WsResult {
        let user_id = self.require_user()?.id;
        let v = db_or_debug_and_return!(self, messages::LoadMessages { user_id }; async);
        Ok(Some(WsMsg::MessagesLoaded(v)))
    }
}

pub struct MarkMessageSeen {
    pub id: MessageId,
}

#[async_trait::async_trait]
impl AsyncHandler<MarkMessageSeen> for WebSocketActor {
    async fn exec(&mut self, msg: MarkMessageSeen) -> WsResult {
        let user_id = self.require_user()?.id;
        let count = db_or_debug_and_return!(
            self,
            messages::MarkMessageSeen {
                message_id: msg.id,
                user_id,
            }; async
        );
        Ok(Some(WsMsg::MessageMarkedSeen(msg.id, count)))
    }
}
