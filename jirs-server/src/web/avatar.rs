use std::io::Write;

use actix::Addr;
use actix_multipart::{Field, Multipart};
use actix_web::http::header::ContentDisposition;
use actix_web::web::Data;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};

use crate::db::DbExecutor;

#[post("/")]
pub async fn upload(
    mut payload: Multipart,
    db: Data<Addr<DbExecutor>>,
) -> Result<HttpResponse, Error> {
    while let Ok(Some(field)) = payload.try_next().await {
        let disposition: ContentDisposition = match field.content_disposition() {
            Some(d) => d,
            _ => continue,
        };
        if !disposition.is_form_data() {
            return Ok(HttpResponse::BadRequest().finish());
        }
        let _name = disposition.get_name().as_ref().cloned().unwrap_or_default();
        match disposition.get_name() {
            Some("token") => handle_token(field, disposition, db.clone()).await?,
            Some("avatar") => handle_image(field, disposition, db.clone()).await?,
            _ => continue,
        };
    }
    Ok(HttpResponse::Ok().json(""))
}

async fn handle_token(
    mut field: Field,
    _disposition: ContentDisposition,
    _db: Data<Addr<DbExecutor>>,
) -> Result<(), Error> {
    Ok(())
}

async fn handle_image(
    mut field: Field,
    disposition: ContentDisposition,
    _db: Data<Addr<DbExecutor>>,
) -> Result<(), Error> {
    let web_config = crate::web::Configuration::read();

    let filename = disposition.get_filename().unwrap();
    let filepath = format!("./tmp/{}", filename);
    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| std::fs::File::create(filepath))
        .await
        .unwrap();
    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // filesystem operations are blocking, we have to use thread pool
        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
    Ok(())
}

#[get("/{id}")]
async fn download(_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json("")
}
