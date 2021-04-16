use futures::executor::block_on;
use jirs_data::{CommentId, CreateCommentPayload, IssueId, UpdateCommentPayload, WsMsg};

use crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

pub struct LoadIssueComments {
    pub issue_id: IssueId,
}

impl WsHandler<LoadIssueComments> for WebSocketActor {
    fn handle_msg(&mut self, msg: LoadIssueComments, _ctx: &mut Self::Context) -> WsResult {
        self.require_user()?;

        let comments = db_or_debug_and_return!(
            self,
            database_actor::comments::LoadIssueComments {
                issue_id: msg.issue_id,
            }
        );

        Ok(Some(WsMsg::IssueCommentsLoaded(comments)))
    }
}

impl WsHandler<CreateCommentPayload> for WebSocketActor {
    fn handle_msg(&mut self, mut msg: CreateCommentPayload, ctx: &mut Self::Context) -> WsResult {
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
            }
        );
        self.handle_msg(LoadIssueComments { issue_id }, ctx)
    }
}

impl WsHandler<UpdateCommentPayload> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateCommentPayload, _ctx: &mut Self::Context) -> WsResult {
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
            }
        );
        self.broadcast(&WsMsg::CommentUpdated(comment));
        Ok(None)
    }
}

pub struct DeleteComment {
    pub comment_id: CommentId,
}

impl WsHandler<DeleteComment> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteComment, _ctx: &mut Self::Context) -> WsResult {
        use database_actor::comments::DeleteComment;

        let user_id = self.require_user()?.id;

        let n = db_or_debug_and_return!(
            self,
            DeleteComment {
                comment_id: msg.comment_id,
                user_id,
            }
        );
        Ok(Some(WsMsg::CommentDeleted(msg.comment_id, n)))
    }
}
