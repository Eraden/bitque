use futures::executor::block_on;
use jirs_data::{Code, Lang, WsMsg};

use crate::{actor_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

pub struct HighlightCode(pub Lang, pub Code);

impl WsHandler<HighlightCode> for WebSocketActor {
    fn handle_msg(&mut self, msg: HighlightCode, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?.id;
        let res = actor_or_debug_and_return!(
            self,
            hi,
            highlight_actor::HighlightCode {
                code: msg.1,
                lang: msg.0,
            }
        );
        Ok(Some(WsMsg::HighlightedCode(res)))
    }
}
