use actix::Handler;
use diesel::prelude::*;

use jirs_data::{Message, MessageId, UserId};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

pub struct LoadMessages {
    pub user_id: UserId,
}

impl actix::Message for LoadMessages {
    type Result = Result<Vec<Message>, ServiceErrors>;
}

impl Handler<LoadMessages> for DbExecutor {
    type Result = Result<Vec<Message>, ServiceErrors>;

    fn handle(&mut self, msg: LoadMessages, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::messages::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = messages.filter(receiver_id.eq(msg.user_id));
        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        query
            .load(conn)
            .map_err(|_| ServiceErrors::DatabaseQueryFailed("load user messages".to_string()))
    }
}

pub struct MarkMessageSeen {
    pub user_id: UserId,
    pub message_id: MessageId,
}

impl actix::Message for MarkMessageSeen {
    type Result = Result<MessageId, ServiceErrors>;
}

impl Handler<MarkMessageSeen> for DbExecutor {
    type Result = Result<MessageId, ServiceErrors>;

    fn handle(&mut self, msg: MarkMessageSeen, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::messages::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = diesel::delete(
            messages
                .find(msg.message_id)
                .filter(receiver_id.eq(msg.user_id)),
        );
        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        query
            .execute(conn)
            .map_err(|_| ServiceErrors::DatabaseQueryFailed("load user messages".to_string()))?;
        Ok(msg.message_id)
    }
}
