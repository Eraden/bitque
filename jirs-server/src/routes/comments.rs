use actix_web::{delete, post, put, HttpResponse};

#[post("/")]
pub async fn create() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<!DOCTYPE html><html><head><title>Issues</title></head><body>Foo</body></html>")
}

#[put("/<id>")]
pub async fn update() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}

#[delete("/<id>")]
pub async fn delete() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}
