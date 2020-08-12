use actix::Handler;
use diesel::prelude::*;

use jirs_data::{BindToken, Message, MessageId, MessageType, User, UserId};

use crate::{
    db::{
        users::{FindUser, LookupUser},
        DbExecutor,
    },
    errors::ServiceErrors,
};

#[derive(Debug)]
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

#[derive(Debug)]
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
        debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
        let size = query
            .execute(conn)
            .map_err(|_| ServiceErrors::DatabaseQueryFailed("load user messages".to_string()))?;

        if size > 0 {
            Ok(msg.message_id)
        } else {
            Err(ServiceErrors::DatabaseQueryFailed(format!(
                "failed to delete message for {:?}",
                msg
            )))
        }
    }
}

#[derive(Debug)]
pub enum CreateMessageReceiver {
    Reference(UserId),
    Lookup { name: String, email: String },
}

#[derive(Debug)]
pub struct CreateMessage {
    pub receiver: CreateMessageReceiver,
    pub sender_id: UserId,
    pub summary: String,
    pub description: String,
    pub message_type: MessageType,
    pub hyper_link: String,
}

impl actix::Message for CreateMessage {
    type Result = Result<Message, ServiceErrors>;
}

impl Handler<CreateMessage> for DbExecutor {
    type Result = Result<Message, ServiceErrors>;

    fn handle(&mut self, msg: CreateMessage, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::messages::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let user: User = match {
            match msg.receiver {
                CreateMessageReceiver::Lookup { name, email } => {
                    self.handle(LookupUser { name, email }, ctx)
                }
                CreateMessageReceiver::Reference(user_id) => self.handle(FindUser { user_id }, ctx),
            }
        } {
            Ok(user) => user,
            _ => {
                return Err(ServiceErrors::RecordNotFound(
                    "No matching user found".to_string(),
                ));
            }
        };

        let query = diesel::insert_into(messages).values((
            receiver_id.eq(user.id),
            sender_id.eq(msg.sender_id),
            summary.eq(msg.summary),
            description.eq(msg.description),
            message_type.eq(msg.message_type),
            hyper_link.eq(msg.hyper_link),
        ));
        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        query
            .get_result(conn)
            .map_err(|_| ServiceErrors::DatabaseQueryFailed("create message failed".to_string()))
    }
}

#[derive(Debug)]
pub struct LookupMessagesByToken {
    pub token: BindToken,
    pub user_id: UserId,
}

impl actix::Message for LookupMessagesByToken {
    type Result = Result<Vec<Message>, ServiceErrors>;
}

impl Handler<LookupMessagesByToken> for DbExecutor {
    type Result = Result<Vec<Message>, ServiceErrors>;

    fn handle(&mut self, msg: LookupMessagesByToken, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::messages::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = messages.filter(
            hyper_link
                .eq(format!("#{}", msg.token))
                .and(receiver_id.eq(msg.user_id)),
        );
        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        query
            .load(conn)
            .map_err(|_| ServiceErrors::DatabaseQueryFailed("create message failed".to_string()))
    }
}
