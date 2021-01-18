use {
    crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult},
    database_actor::messages,
    futures::executor::block_on,
    jirs_data::{MessageId, WsMsg},
};

pub struct LoadMessages;

impl WsHandler<LoadMessages> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadMessages, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        let v = db_or_debug_and_return!(self, messages::LoadMessages { user_id });
        Ok(Some(WsMsg::MessagesLoaded(v)))
    }
}

pub struct MarkMessageSeen {
    pub id: MessageId,
}

impl WsHandler<MarkMessageSeen> for WebSocketActor {
    fn handle_msg(&mut self, msg: MarkMessageSeen, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        let count = db_or_debug_and_return!(
            self,
            messages::MarkMessageSeen {
                message_id: msg.id,
                user_id,
            }
        );
        Ok(Some(WsMsg::MessageMarkedSeen(msg.id, count)))
    }
}
