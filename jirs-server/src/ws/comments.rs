use futures::executor::block_on;

use jirs_data::{CommentId, CreateCommentPayload, IssueId, UpdateCommentPayload, WsMsg};

use crate::ws::{WebSocketActor, WsHandler, WsResult};

pub struct LoadIssueComments {
    pub issue_id: IssueId,
}

impl WsHandler<LoadIssueComments> for WebSocketActor {
    fn handle_msg(&mut self, msg: LoadIssueComments, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;

        let comments = match block_on(self.db.send(crate::db::comments::LoadIssueComments {
            issue_id: msg.issue_id,
        })) {
            Ok(Ok(comments)) => comments,
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        };

        Ok(Some(WsMsg::IssueCommentsLoaded(comments)))
    }
}

impl WsHandler<CreateCommentPayload> for WebSocketActor {
    fn handle_msg(&mut self, mut msg: CreateCommentPayload, ctx: &mut Self::Context) -> WsResult {
        use crate::db::comments::CreateComment;

        let user_id = self.require_user()?.id;
        if msg.user_id.is_none() {
            msg.user_id = Some(user_id);
        }
        let issue_id = msg.issue_id;
        match block_on(self.db.send(CreateComment {
            user_id,
            issue_id,
            body: msg.body,
        })) {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        };
        self.handle_msg(LoadIssueComments { issue_id }, ctx)
    }
}

impl WsHandler<UpdateCommentPayload> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateCommentPayload, _ctx: &mut Self::Context) -> WsResult {
        use crate::db::comments::UpdateComment;

        let user_id = self.require_user()?.id;

        let UpdateCommentPayload {
            id: comment_id,
            body,
        } = msg;

        let comment = match block_on(self.db.send(UpdateComment {
            comment_id,
            user_id,
            body,
        })) {
            Ok(Ok(comment)) => comment,
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        };
        self.broadcast(&WsMsg::CommentUpdated(comment));
        Ok(None)
    }
}

pub struct DeleteComment {
    pub comment_id: CommentId,
}

impl WsHandler<DeleteComment> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteComment, _ctx: &mut Self::Context) -> WsResult {
        use crate::db::comments::DeleteComment;

        let user_id = self.require_user()?.id;

        let m = DeleteComment {
            comment_id: msg.comment_id,
            user_id,
        };
        match block_on(self.db.send(m)) {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => {
                error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        };

        Ok(Some(WsMsg::CommentDeleted(msg.comment_id)))
    }
}
