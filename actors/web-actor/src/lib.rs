#[macro_use]
extern crate log;

pub use errors::*;
use {
    crate::middleware::authorize::token_from_headers,
    actix::Addr,
    actix_web::{web::Data, HttpRequest, HttpResponse},
    database_actor::{authorize_user::AuthorizeUser, DbExecutor},
    jirs_data::User,
};

pub mod avatar;
pub mod errors;
pub mod handlers;
pub mod middleware;

pub async fn user_from_request(
    req: HttpRequest,
    db: &Data<Addr<DbExecutor>>,
) -> Result<User, HttpResponse> {
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return Err(ServiceError::Unauthorized.into_http_response()),
    };
    match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => Ok(user),
        Ok(Err(_e)) => Err(HttpResponse::InternalServerError().body("Critical database error")),
        _ => Err(ServiceError::Unauthorized.into_http_response()),
    }
}