use actix::Addr;
use actix_web::web::Data;

use jirs_data::WsMsg;

use crate::db::users::{LoadProjectUsers, Register};
use crate::db::DbExecutor;
use crate::mail::MailExecutor;
use crate::ws::auth::authenticate;
use crate::ws::{current_user, WsResult};

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

pub async fn register(
    db: &Data<Addr<DbExecutor>>,
    mail: &Data<Addr<MailExecutor>>,
    name: String,
    email: String,
) -> WsResult {
    let msg = match db
        .send(Register {
            name: name.clone(),
            email: email.clone(),
        })
        .await
    {
        Ok(Ok(_)) => Some(WsMsg::SignUpSuccess),
        Ok(Err(_)) => Some(WsMsg::SignUpPairTaken),
        _ => None,
    };

    match authenticate(db, mail, name, email).await {
        Ok(_) => (),
        Err(e) => return Ok(Some(e)),
    };

    Ok(msg)
}
