use diesel::prelude::*;
use jirs_data::{Comment, CommentId, IssueId, UserId};

use crate::{db_create, db_delete, db_load, db_update};

db_load! {
    LoadIssueComments,
    msg => comments => comments.distinct_on(id).filter(issue_id.eq(msg.issue_id)),
    Comment,
    issue_id => IssueId
}

db_create! {
    CreateComment,
    msg => comments => diesel::insert_into(comments).values((
        body.eq(msg.body),
        user_id.eq(msg.user_id),
        issue_id.eq(msg.issue_id),
    )),
    Comment,
    issue_id => IssueId,
    user_id => UserId,
    body => String
}

db_update! {
    UpdateComment,
    msg => comments => diesel::update(
            comments
                .filter(user_id.eq(msg.user_id))
                .find(msg.comment_id),
        )
        .set(body.eq(msg.body)),
    Comment,
    comment_id => CommentId,
    user_id => UserId,
    body => String
}

db_delete! {
    DeleteComment,
    msg => comments => diesel::delete(
            comments
                .filter(user_id.eq(msg.user_id))
                .find(msg.comment_id),
        ),
    Comment,
    comment_id => CommentId,
    user_id => UserId
}
