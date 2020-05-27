use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use actix::Handler;
use diesel::prelude::*;
use jirs_data::{Message, UserId};

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
