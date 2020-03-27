use crate::db::DbExecutor;
use actix::Addr;
use actix_web::web::Data;
use actix_web::{get, put, HttpRequest, HttpResponse};

#[get("")]
pub async fn project_with_users_and_issues(
    req: HttpRequest,
    db: Data<Addr<DbExecutor>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().body("")
}

#[put("")]
pub async fn update(req: HttpRequest, db: Data<Addr<DbExecutor>>) -> HttpResponse {
    HttpResponse::NotImplemented().body("")
}
