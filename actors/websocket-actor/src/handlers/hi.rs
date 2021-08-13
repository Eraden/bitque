use jirs_data::{Code, Lang, WsMsg};

use crate::{actor_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

pub struct HighlightCode(pub Lang, pub Code);

#[async_trait::async_trait]
impl AsyncHandler<HighlightCode> for WebSocketActor {
    async fn exec(&mut self, msg: HighlightCode) -> WsResult {
        self.require_user()?;
        let res = actor_or_debug_and_return!(
            self,
            hi,
            highlight_actor::HighlightCode {
                code: msg.1,
                lang: msg.0,
            }; async
        );
        Ok(Some(WsMsg::HighlightedCode(res)))
    }
}
