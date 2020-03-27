use crate::db::authorize_user::AuthorizeUser;
use crate::db::DbExecutor;
use crate::middleware::authorize::token_from_headers;
use actix::Addr;
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse};

#[get("")]
pub async fn current_user(req: HttpRequest, db: Data<Addr<DbExecutor>>) -> HttpResponse {
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        _ => crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    }
}
