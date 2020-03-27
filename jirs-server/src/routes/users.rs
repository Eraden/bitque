use actix_web::{get, HttpResponse};

#[get("/currentUser")]
pub async fn current_user() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}
