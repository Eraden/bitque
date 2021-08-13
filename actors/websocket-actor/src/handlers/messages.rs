use database_actor::messages;
use jirs_data::msg::WsMsgMessage;
use jirs_data::MessageId;

use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgMessage> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgMessage) -> WsResult {
        match msg {
            // messages
            WsMsgMessage::MessagesLoad => self.exec(LoadMessages).await,
            WsMsgMessage::MessageMarkSeen(id) => self.exec(MarkMessageSeen { id }).await,
            WsMsgMessage::MessageUpdated(_) => Ok(None),
            WsMsgMessage::MessagesLoaded(_) => Ok(None),
            WsMsgMessage::MessageMarkedSeen(_, _) => Ok(None),
        }
    }
}

pub struct LoadMessages;

#[async_trait::async_trait]
impl AsyncHandler<LoadMessages> for WebSocketActor {
    async fn exec(&mut self, _msg: LoadMessages) -> WsResult {
        let user_id = self.require_user()?.id;
        let v = db_or_debug_and_return!(self, messages::LoadMessages { user_id }; async);
        Ok(Some(WsMsgMessage::MessagesLoaded(v).into()))
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
        Ok(Some(WsMsgMessage::MessageMarkedSeen(msg.id, count).into()))
    }
}
