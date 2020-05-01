use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/")]
async fn upload() -> impl Responder {
    HttpResponse::Ok().json("")
}

#[get("/{id}")]
async fn download(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json("")
}
