#[cfg(feature = "aws-s3")]
use std::fs::File;
#[cfg(feature = "aws-s3")]
use std::io::Read;
use std::io::Write;

use actix::Addr;
use actix_multipart::{Field, Multipart};
use actix_web::http::header::ContentDisposition;
use actix_web::web::Data;
use actix_web::{post, web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
#[cfg(feature = "aws-s3")]
use rusoto_s3::{PutObjectRequest, S3Client, S3};

use jirs_data::{User, UserId, WsMsg};

use crate::db::authorize_user::AuthorizeUser;
use crate::db::users::UpdateAvatarUrl;
use crate::db::DbExecutor;
#[cfg(feature = "aws-s3")]
use crate::web::AmazonS3Storage;
use crate::ws::InnerMsg::BroadcastToChannel;
use crate::ws::WsServer;

#[post("/")]
pub async fn upload(
    mut payload: Multipart,
    db: Data<Addr<DbExecutor>>,
    ws: Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    let mut user_id: Option<UserId> = None;
    let mut avatar_url: Option<String> = None;

    while let Ok(Some(field)) = payload.try_next().await {
        let disposition: ContentDisposition = match field.content_disposition() {
            Some(d) => d,
            _ => continue,
        };
        if !disposition.is_form_data() {
            return Ok(HttpResponse::BadRequest().finish());
        }
        match disposition.get_name() {
            Some("token") => {
                user_id = Some(handle_token(field, db.clone()).await?);
            }
            Some("avatar") => {
                let id = user_id.ok_or_else(|| HttpResponse::Unauthorized().finish())?;
                avatar_url = Some(handle_image(id, field, disposition, db.clone()).await?);
            }
            _ => continue,
        };
    }
    match (user_id, avatar_url) {
        (Some(user_id), Some(avatar_url)) => {
            let user = update_user_avatar(user_id, avatar_url.clone(), db).await?;
            ws.send(BroadcastToChannel(
                user.project_id,
                WsMsg::AvatarUrlChanged(user.id, avatar_url),
            ))
            .await
            .map_err(|_| HttpResponse::UnprocessableEntity().finish())?;
            Ok(HttpResponse::NoContent().finish())
        }
        _ => Ok(HttpResponse::UnprocessableEntity().finish()),
    }
}

async fn update_user_avatar(
    user_id: UserId,
    new_url: String,
    db: Data<Addr<DbExecutor>>,
) -> Result<User, Error> {
    match db
        .send(UpdateAvatarUrl {
            user_id,
            avatar_url: Some(new_url),
        })
        .await
    {
        Ok(Ok(user)) => Ok(user),

        Ok(Err(e)) => {
            error!("{:?}", e);
            Err(HttpResponse::Unauthorized().finish().into())
        }
        Err(e) => {
            error!("{:?}", e);
            Err(HttpResponse::Unauthorized().finish().into())
        }
    }
}

async fn handle_token(mut field: Field, db: Data<Addr<DbExecutor>>) -> Result<UserId, Error> {
    let mut f: Vec<u8> = vec![];
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
    let access_token = String::from_utf8(f)
        .unwrap_or_default()
        .parse::<uuid::Uuid>()
        .map_err(|_| HttpResponse::Unauthorized().finish())?;
    match db.send(AuthorizeUser { access_token }).await {
        Ok(Ok(user)) => Ok(user.id),

        Ok(Err(e)) => {
            error!("{:?}", e);
            Err(HttpResponse::Unauthorized().finish().into())
        }
        Err(e) => {
            error!("{:?}", e);
            Err(HttpResponse::Unauthorized().finish().into())
        }
    }
}

async fn handle_image(
    user_id: UserId,
    mut field: Field,
    disposition: ContentDisposition,
    _db: Data<Addr<DbExecutor>>,
) -> Result<String, Error> {
    let web_config = crate::web::Configuration::read();

    let mut new_link = None;
    let filename = disposition.get_filename().unwrap();
    let tmp_file_path = format!("{}/{}-{}", web_config.tmp_dir, user_id, filename);
    let mut f = web::block(move || std::fs::File::create(tmp_file_path))
        .await
        .unwrap();

    // Write temp file
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    // Write public visible file
    #[cfg(feature = "local-storage")]
    if !web_config.filesystem.is_empty() {
        let filesystem = &web_config.filesystem;
        std::fs::copy(
            format!("{}/{}-{}", web_config.tmp_dir, user_id, filename),
            format!("{}/{}-{}", filesystem.store_path, user_id, filename),
        )
        .map_err(|_| HttpResponse::InsufficientStorage().finish())?;

        new_link = Some(format!(
            "{proto}://{bind}{port}{client_path}/{user_id}-{filename}",
            proto = if web_config.ssl { "https" } else { "http" },
            bind = web_config.bind,
            port = match web_config.port.as_str() {
                "80" | "443" => "".to_string(),
                p @ _ => format!(":{}", p),
            },
            client_path = filesystem.client_path,
            user_id = user_id,
            filename = filename
        ));
    }

    // Upload to AWS S3
    #[cfg(feature = "aws-s3")]
    if !web_config.s3.is_empty() {
        let s3 = &web_config.s3;
        s3.set_variables();
        let key = format!("{}-{}", user_id, filename);
        let mut tmp_file = File::open(format!("{}/{}-{}", web_config.tmp_dir, user_id, filename))
            .map_err(|_| HttpResponse::InternalServerError())?;
        let mut buffer: Vec<u8> = vec![];
        tmp_file
            .read_to_end(&mut buffer)
            .map_err(|_| HttpResponse::InternalServerError())?;

        let client = S3Client::new(s3.region());
        let put_object = PutObjectRequest {
            bucket: s3.bucket.clone(),
            key: key.clone(),
            body: Some(buffer.into()),
            ..Default::default()
        };
        let _id = client
            .put_object(put_object)
            .await
            .map_err(|_| HttpResponse::InternalServerError())?;
        new_link = Some(aws_s3_url(key.as_str(), s3));
    }
    std::fs::remove_file(format!("{}/{}-{}", web_config.tmp_dir, user_id, filename).as_str())
        .unwrap_or_default();
    Ok(new_link.unwrap_or_default())
}

#[cfg(feature = "aws-s3")]
fn aws_s3_url(key: &str, config: &AmazonS3Storage) -> String {
    format!(
        "https://{bucket}.s3.{region}.amazonaws.com/{key}",
        bucket = config.bucket,
        region = config.region_name,
        key = key
    )
}
