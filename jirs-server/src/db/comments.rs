use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::Comment;

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
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let rows: Vec<Comment> = comments
            .distinct_on(id)
            .filter(issue_id.eq(msg.issue_id))
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))?;
        Ok(rows)
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
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let row: Comment = diesel::insert_into(comments)
            .values(form)
            .get_result::<Comment>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))?;
        Ok(row)
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
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let row: Comment = diesel::update(
            comments
                .filter(user_id.eq(msg.user_id))
                .find(msg.comment_id),
        )
        .set(body.eq(msg.body))
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
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        diesel::delete(
            comments
                .filter(user_id.eq(msg.user_id))
                .find(msg.comment_id),
        )
        .execute(conn)
        .map_err(|_| ServiceErrors::RecordNotFound("issue comments".to_string()))?;
        Ok(())
    }
}
