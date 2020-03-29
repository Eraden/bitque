use actix::Addr;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};

use crate::db::authorize_user::AuthorizeUser;
use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::middleware::authorize::token_from_headers;
use crate::models::User;

pub mod comments;
pub mod issues;
pub mod projects;
pub mod users;

pub async fn user_from_request(
    req: HttpRequest,
    db: &Data<Addr<DbExecutor>>,
) -> Result<User, HttpResponse> {
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return Err(ServiceErrors::Unauthorized.into_http_response()),
    };
    match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => Ok(user),
        Ok(Err(e)) => Err(e.into_http_response()),
        _ => Err(ServiceErrors::Unauthorized.into_http_response()),
    }
}
