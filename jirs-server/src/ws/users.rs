use actix::Addr;
use actix_web::web::Data;

use jirs_data::WsMsg;

use crate::db::users::{FindUser, LoadProjectUsers};
use crate::db::DbExecutor;
use crate::ws::{current_user, WsResult};

pub async fn authenticate(db: &Data<Addr<DbExecutor>>, name: String, email: String) -> WsResult {
    // TODO check attempt number, allow only 5 times per day
    let _user = match db.send(FindUser { name, email }).await {
        Ok(Ok(user)) => user,
        Ok(Err(e)) => {
            error!("{:?}", e);
            return Ok(None);
        }
        Err(e) => {
            error!("{:?}", e);
            return Ok(None);
        }
    };
    //  TODO send email somehow
    Ok(Some(WsMsg::AuthenticateSuccess))
}

pub async fn load_project_users(
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
