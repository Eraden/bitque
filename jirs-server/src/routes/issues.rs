use actix_web::{delete, get, post, put, HttpResponse};

#[get("")]
pub async fn project_issues() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<!DOCTYPE html><html><head><title>Issues</title></head><body>Foo</body></html>")
}

#[get("/<id>")]
pub async fn issue_with_users_and_omments() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}

#[post("/")]
pub async fn create() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}

#[put("/<id>")]
pub async fn update() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}

#[delete("/<id>")]
pub async fn delete() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}
