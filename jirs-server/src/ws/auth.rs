use crate::db::tokens::CreateBindToken;
use crate::db::users::FindUser;
use crate::db::DbExecutor;
use crate::ws::WsResult;
use actix::Addr;
use actix_web::web::Data;
use jirs_data::WsMsg;

pub async fn authenticate(db: &Data<Addr<DbExecutor>>, name: String, email: String) -> WsResult {
    // TODO check attempt number, allow only 5 times per day
    let user = match db.send(FindUser { name, email }).await {
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
    let _token = match db.send(CreateBindToken { user_id: user.id }).await {
        Ok(Ok(token)) => token,
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
