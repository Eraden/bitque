use actix::Addr;
use actix_web::web::Data;

use jirs_data::WsMsg;

use crate::db::users::LoadProjectUsers;
use crate::db::DbExecutor;
use crate::ws::{current_user, WsResult};

pub async fn current_project(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
) -> WsResult {
    let project_id = current_user(user).map(|u| u.project_id)?;

    let m = match db.send(LoadProjectUsers { project_id }).await {
        Ok(Ok(v)) => Some(WsMsg::ProjectUsersLoaded(
            v.into_iter().map(|i| i.into()).collect(),
        )),
        _ => None,
    };
    Ok(m)
}
