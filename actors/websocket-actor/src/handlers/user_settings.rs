use futures::executor::block_on;
use jirs_data::{TextEditorMode, UserId, UserSetting, WsMsg};

use crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

pub fn default_user_setting(user_id: UserId) -> UserSetting {
    UserSetting {
        id: 0,
        user_id,
        text_editor_mode: Default::default(),
    }
}

pub struct SetTextEditorMode {
    pub mode: TextEditorMode,
}

impl WsHandler<SetTextEditorMode> for WebSocketActor {
    fn handle_msg(&mut self, msg: SetTextEditorMode, _ctx: &mut Self::Context) -> WsResult {
        let user_id = self.require_user()?.id;
        let setting = db_or_debug_and_return!(
            self,
            database_actor::user_settings::UpdateUserSetting {
                user_id,
                mode: msg.mode
            }
        );
        Ok(Some(WsMsg::UserSettingUpdated(setting)))
    }
}
