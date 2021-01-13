use {
    crate::{WebSocketActor, WsHandler, WsResult},
    futures::executor::block_on,
    jirs_data::{Code, Lang, WsMsg},
};

pub struct HighlightCode(pub Lang, pub Code);

impl WsHandler<HighlightCode> for WebSocketActor {
    fn handle_msg(&mut self, msg: HighlightCode, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?.id;
        match block_on(self.hi.send(highlight_actor::HighlightCode {
            code: msg.1,
            lang: msg.0,
        })) {
            Ok(Ok(res)) => Ok(Some(WsMsg::HighlightedCode(res))),
            Ok(Err(e)) => {
                error!("{:?}", e);
                Ok(None)
            }
            Err(e) => {
                error!("{}", e);
                Ok(None)
            }
        }
    }
}
