use actix::{Handler, Message};
use diesel::prelude::*;

use jirs_data::{msg::WsError, Comment};

use crate::{
    db::{DbExecutor, DbPooledConn},
    db_pool,
    errors::ServiceError,
    q,
};

pub struct LoadIssueComments {
    pub issue_id: i32,
}

impl LoadIssueComments {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Vec<Comment>, ServiceError> {
        use crate::schema::comments::dsl::*;

        q!(comments.distinct_on(id).filter(issue_id.eq(self.issue_id)))
            .load(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceError::Error(WsError::FailedToLoadComments)
            })
    }
}

impl Message for LoadIssueComments {
    type Result = Result<Vec<Comment>, ServiceError>;
}

impl Handler<LoadIssueComments> for DbExecutor {
    type Result = Result<Vec<Comment>, ServiceError>;

    fn handle(&mut self, msg: LoadIssueComments, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct CreateComment {
    pub user_id: i32,
    pub issue_id: i32,
    pub body: String,
}

impl CreateComment {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Comment, ServiceError> {
        use crate::schema::comments::dsl::*;
        q!(diesel::insert_into(comments).values((
            body.eq(self.body),
            user_id.eq(self.user_id),
            issue_id.eq(self.issue_id),
        )))
        .get_result::<Comment>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::InvalidComment)
        })
    }
}

impl Message for CreateComment {
    type Result = Result<Comment, ServiceError>;
}

impl Handler<CreateComment> for DbExecutor {
    type Result = Result<Comment, ServiceError>;

    fn handle(&mut self, msg: CreateComment, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct UpdateComment {
    pub comment_id: i32,
    pub user_id: i32,
    pub body: String,
}

impl UpdateComment {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Comment, ServiceError> {
        use crate::schema::comments::dsl::*;

        q!(diesel::update(
            comments
                .filter(user_id.eq(self.user_id))
                .find(self.comment_id),
        )
        .set(body.eq(self.body)))
        .get_result::<Comment>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::FailedToUpdateComment)
        })
    }
}

impl Message for UpdateComment {
    type Result = Result<Comment, ServiceError>;
}

impl Handler<UpdateComment> for DbExecutor {
    type Result = Result<Comment, ServiceError>;

    fn handle(&mut self, msg: UpdateComment, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)
    }
}

pub struct DeleteComment {
    pub comment_id: i32,
    pub user_id: i32,
}

impl DeleteComment {
    pub fn execute(self, conn: &DbPooledConn) -> Result<usize, ServiceError> {
        use crate::schema::comments::dsl::*;
        q!(diesel::delete(
            comments
                .filter(user_id.eq(self.user_id))
                .find(self.comment_id),
        ))
        .execute(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::UnableToDeleteComment)
        })
    }
}

impl Message for DeleteComment {
    type Result = Result<(), ServiceError>;
}

impl Handler<DeleteComment> for DbExecutor {
    type Result = Result<(), ServiceError>;

    fn handle(&mut self, msg: DeleteComment, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);
        msg.execute(conn)?;
        Ok(())
    }
}
