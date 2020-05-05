use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::Comment;

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

#[derive(Serialize, Deserialize)]
pub struct LoadIssueComments {
    pub issue_id: i32,
}

impl Message for LoadIssueComments {
    type Result = Result<Vec<Comment>, ServiceErrors>;
}

impl Handler<LoadIssueComments> for DbExecutor {
    type Result = Result<Vec<Comment>, ServiceErrors>;

    fn handle(&mut self, msg: LoadIssueComments, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let comments_query = comments.distinct_on(id).filter(issue_id.eq(msg.issue_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&comments_query));
        comments_query
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateComment {
    pub user_id: i32,
    pub issue_id: i32,
    pub body: String,
}

impl Message for CreateComment {
    type Result = Result<Comment, ServiceErrors>;
}

impl Handler<CreateComment> for DbExecutor {
    type Result = Result<Comment, ServiceErrors>;

    fn handle(&mut self, msg: CreateComment, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::CommentForm;
        use crate::schema::comments::dsl::*;

        let form = CommentForm {
            body: msg.body,
            user_id: msg.user_id,
            issue_id: msg.issue_id,
        };

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let comment_query = diesel::insert_into(comments).values(form);
        debug!("{}", diesel::debug_query::<Pg, _>(&comment_query));
        comment_query
            .get_result::<Comment>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateComment {
    pub comment_id: i32,
    pub user_id: i32,
    pub body: String,
}

impl Message for UpdateComment {
    type Result = Result<Comment, ServiceErrors>;
}

impl Handler<UpdateComment> for DbExecutor {
    type Result = Result<Comment, ServiceErrors>;

    fn handle(&mut self, msg: UpdateComment, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let query = diesel::update(
            comments
                .filter(user_id.eq(msg.user_id))
                .find(msg.comment_id),
        )
        .set(body.eq(msg.body));
        info!("{}", diesel::debug_query::<Pg, _>(&query));
        let row: Comment = query
            .get_result::<Comment>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))?;
        Ok(row)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteComment {
    pub comment_id: i32,
    pub user_id: i32,
}

impl Message for DeleteComment {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<DeleteComment> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: DeleteComment, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let comment_query = diesel::delete(
            comments
                .filter(user_id.eq(msg.user_id))
                .find(msg.comment_id),
        );
        debug!("{}", diesel::debug_query::<Pg, _>(&comment_query));
        comment_query
            .execute(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))?;
        Ok(())
    }
}
