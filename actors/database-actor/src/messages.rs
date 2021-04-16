use diesel::prelude::*;
use jirs_data::{BindToken, Message, MessageId, MessageType, User, UserId};

use crate::users::{FindUser, LookupUser};
use crate::{db_create, db_delete, db_load};

db_load! {
    LoadMessages,
    msg => messages => messages.filter(receiver_id.eq(msg.user_id)),
    Message,
    user_id => UserId
}

db_delete! {
    MarkMessageSeen,
    msg => messages => diesel::delete(
        messages.find(msg.message_id).filter(receiver_id.eq(msg.user_id))
    ),
    Message,
    user_id => UserId,
    message_id => MessageId
}

#[derive(Debug)]
pub enum CreateMessageReceiver {
    Reference(UserId),
    Lookup { name: String, email: String },
}

db_create! {
    CreateMessage,
    msg => conn => messages => {
        let user: User = match msg.receiver {
            CreateMessageReceiver::Lookup { name, email } => {
                LookupUser { name, email }.execute(conn)?
            }
            CreateMessageReceiver::Reference(user_id) => FindUser { user_id }.execute(conn)?,
        };

        diesel::insert_into(messages).values((
            receiver_id.eq(user.id),
            sender_id.eq(msg.sender_id),
            summary.eq(msg.summary),
            description.eq(msg.description),
            message_type.eq(msg.message_type),
            hyper_link.eq(msg.hyper_link),
        ))
    },
    Message,
    receiver => CreateMessageReceiver,
    sender_id => UserId,
    summary => String,
    description => String,
    message_type => MessageType,
    hyper_link => String
}

db_load! {
    LookupMessagesByToken,
    msg => messages => messages.filter(
        hyper_link.eq(format!("#{}", msg.token)).and(receiver_id.eq(msg.user_id)),
    ),
    Message,
    token => BindToken,
    user_id => UserId
}
