use actix::Addr;
use actix_web::web::Data;

use jirs_data::WsMsg;

use crate::db::tokens::CreateBindToken;
use crate::db::users::FindUser;
use crate::db::DbExecutor;
use crate::mail::welcome::Welcome;
use crate::mail::MailExecutor;
use crate::ws::WsResult;

pub async fn authenticate(
    db: &Data<Addr<DbExecutor>>,
    mail: &Data<Addr<MailExecutor>>,
    name: String,
    email: String,
) -> WsResult {
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
    let token = match db.send(CreateBindToken { user_id: user.id }).await {
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
    if let Some(bind_token) = token.bind_token.as_ref().cloned() {
        match mail
            .send(Welcome {
                bind_token,
                email: user.email.clone(),
            })
            .await
        {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => {
                error!("{}", e);
                return Ok(None);
            }
            Err(e) => {
                error!("{}", e);
                return Ok(None);
            }
        }
    }
    Ok(Some(WsMsg::AuthenticateSuccess))
}
