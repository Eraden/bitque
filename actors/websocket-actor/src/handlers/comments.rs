use jirs_data::msg::WsMsgComment;
use jirs_data::{CommentId, CreateCommentPayload, IssueId, UpdateCommentPayload, WsMsg};

use crate::{db_or_debug_and_return, AsyncHandler, WebSocketActor, WsResult};

#[async_trait::async_trait]
impl AsyncHandler<WsMsgComment> for WebSocketActor {
    async fn exec(&mut self, msg: WsMsgComment) -> WsResult {
        match msg {
            WsMsgComment::IssueCommentsLoad(issue_id) => {
                self.exec(LoadIssueComments { issue_id }).await
            }
            WsMsgComment::CommentCreate(payload) => self.exec(payload).await,
            WsMsgComment::CommentUpdate(payload) => self.exec(payload).await,
            WsMsgComment::CommentDelete(comment_id) => {
                self.exec(DeleteComment { comment_id }).await
            }
            WsMsgComment::IssueCommentsLoaded(_) => Ok(None),
            WsMsgComment::CommentCreated(_) => Ok(None),
            WsMsgComment::CommentUpdated(_) => Ok(None),
            WsMsgComment::CommentDeleted(_, _) => Ok(None),
        }
    }
}

pub struct LoadIssueComments {
    pub issue_id: IssueId,
}

#[async_trait::async_trait]
impl AsyncHandler<LoadIssueComments> for WebSocketActor {
    async fn exec(&mut self, msg: LoadIssueComments) -> WsResult {
        self.require_user()?;

        let comments = db_or_debug_and_return!(
            self,
            database_actor::comments::LoadIssueComments {
                issue_id: msg.issue_id,
            }; async
        );

        Ok(Some(WsMsg::Comment(WsMsgComment::IssueCommentsLoaded(
            comments,
        ))))
    }
}

#[async_trait::async_trait]
impl AsyncHandler<CreateCommentPayload> for WebSocketActor {
    async fn exec(&mut self, mut msg: CreateCommentPayload) -> WsResult {
        use database_actor::comments::CreateComment;

        let user_id = self.require_user()?.id;
        if msg.user_id.is_none() {
            msg.user_id = Some(user_id);
        }
        let issue_id = msg.issue_id;
        let _ = db_or_debug_and_return!(
            self,
            CreateComment {
                user_id,
                issue_id,
                body: msg.body,
            }; async
        );
        self.exec(LoadIssueComments { issue_id }).await
    }
}

#[async_trait::async_trait]
impl AsyncHandler<UpdateCommentPayload> for WebSocketActor {
    async fn exec(&mut self, msg: UpdateCommentPayload) -> WsResult {
        use database_actor::comments::UpdateComment;

        let user_id = self.require_user()?.id;

        let UpdateCommentPayload {
            id: comment_id,
            body,
        } = msg;

        let comment = db_or_debug_and_return!(
            self,
            UpdateComment {
                comment_id,
                user_id,
                body,
            }; async
        );
        self.broadcast(&WsMsg::Comment(WsMsgComment::CommentUpdated(comment)));
        Ok(None)
    }
}

pub struct DeleteComment {
    pub comment_id: CommentId,
}

#[async_trait::async_trait]
impl AsyncHandler<DeleteComment> for WebSocketActor {
    async fn exec(&mut self, msg: DeleteComment) -> WsResult {
        use database_actor::comments::DeleteComment;

        let user_id = self.require_user()?.id;

        let n = db_or_debug_and_return!(
            self,
            DeleteComment {
                comment_id: msg.comment_id,
                user_id,
            }; async
        );
        Ok(Some(WsMsg::Comment(WsMsgComment::CommentDeleted(
            msg.comment_id,
            n,
        ))))
    }
}
