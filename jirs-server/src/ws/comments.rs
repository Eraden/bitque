use actix::Addr;
use actix_web::web::Data;

use jirs_data::{CommentId, CreateCommentPayload, IssueId, UpdateCommentPayload, WsMsg};

use crate::db::comments::LoadIssueComments;
use crate::db::DbExecutor;
use crate::ws::{current_user, WsResult};

pub async fn load_issues(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    issue_id: IssueId,
) -> WsResult {
    current_user(user)?;
    let comments = match db.send(LoadIssueComments { issue_id }).await {
        Ok(Ok(comments)) => comments.into_iter().map(|c| c.into()).collect(),
        _ => return Ok(None),
    };

    Ok(Some(WsMsg::IssueCommentsLoaded(comments)))
}

pub async fn create_comment(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    mut payload: CreateCommentPayload,
) -> WsResult {
    use crate::db::comments::CreateComment;

    let user_id = current_user(user)?.id;
    if payload.user_id.is_none() {
        payload.user_id = Some(user_id);
    }
    let issue_id = payload.issue_id;
    match db
        .send(CreateComment {
            user_id,
            issue_id,
            body: payload.body,
        })
        .await
    {
        Ok(Ok(_)) => (),
        _ => return Ok(None),
    };
    load_issues(db, user, issue_id).await
}

pub async fn update_comment(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    payload: UpdateCommentPayload,
) -> WsResult {
    use crate::db::comments::UpdateComment;

    info!("{:?}", payload);
    let user_id = current_user(user)?.id;

    let UpdateCommentPayload {
        id: comment_id,
        body,
    } = payload;

    let issue_id = match db
        .send(UpdateComment {
            comment_id,
            user_id,
            body,
        })
        .await
    {
        Ok(Ok(comment)) => comment.issue_id,
        _ => return Ok(None),
    };
    load_issues(db, user, issue_id).await
}

pub async fn delete_comment(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    comment_id: CommentId,
) -> WsResult {
    use crate::db::comments::DeleteComment;

    let user_id = current_user(user)?.id;

    let msg = DeleteComment {
        comment_id,
        user_id,
    };
    match db.send(msg).await {
        Ok(Ok(_)) => (),
        _ => return Ok(None),
    };

    Ok(Some(WsMsg::CommentDeleted(comment_id)))
}
