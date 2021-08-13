use jirs_data::msg::WsMsgUser;
use jirs_data::{TextEditorMode, UserId, UserSetting};

use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

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

#[async_trait::async_trait]
impl AsyncHandler<SetTextEditorMode> for WebSocketActor {
    async fn exec(&mut self, msg: SetTextEditorMode) -> WsResult {
        let user_id = self.require_user()?.id;
        let setting = db_or_debug_and_return!(
            self,
            database_actor::user_settings::UpdateUserSetting {
                user_id,
                mode: msg.mode
            }; async
        );
        Ok(Some(WsMsgUser::UserSettingUpdated(setting).into()))
    }
}
