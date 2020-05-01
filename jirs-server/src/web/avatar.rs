use actix_multipart::Multipart;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

#[post("/")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().json(""))
}

#[get("/{id}")]
async fn download(_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json("")
}
