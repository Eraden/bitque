use actix::Addr;
use actix_web::web::Data;

use jirs_data::{IssueId, WsMsg};

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
