use std::io::Write;

use actix::Addr;
use actix_multipart::{Field, Multipart};
use actix_web::http::header::ContentDisposition;
use actix_web::web::Data;
use actix_web::{post, web, Error, HttpResponse};
use common::*;
use database_actor::authorize_user::AuthorizeUser;
use database_actor::user_projects::CurrentUserProject;
use database_actor::users::UpdateAvatarUrl;
use database_actor::DbExecutor;
#[cfg(feature = "local-storage")]
use futures::executor::block_on;
use futures::{StreamExt, TryStreamExt};
use jirs_data::msg::{WsMsg, WsMsgUser};
use jirs_data::{User, UserId};
use websocket_actor::server::InnerMsg::BroadcastToChannel;
use websocket_actor::server::WsServer;

#[cfg(feature = "aws-s3")]
#[post("/")]
pub async fn upload(
    mut payload: Multipart,
    db: Data<Addr<DbExecutor>>,
    ws: Data<Addr<WsServer>>,
    fs: Data<Addr<filesystem_actor::FileSystemExecutor>>,
    amazon: Data<Addr<amazon_actor::AmazonExecutor>>,
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
                avatar_url = Some(
                    crate::handlers::upload_avatar_image::handle_image(
                        id,
                        field,
                        disposition,
                        fs.clone(),
                        amazon.clone(),
                    )
                    .await?,
                );
            }
            _ => continue,
        };
    }
    let user_id = match user_id {
        Some(id) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let project_id = match block_on(db.send(CurrentUserProject { user_id })) {
        Ok(Ok(user_project)) => user_project.project_id,
        _ => return Ok(HttpResponse::UnprocessableEntity().finish()),
    };

    match (user_id, avatar_url) {
        (user_id, Some(avatar_url)) => {
            let user = update_user_avatar(user_id, avatar_url.clone(), db).await?;
            ws.send(BroadcastToChannel(
                project_id,
                WsMsgUser::AvatarUrlChanged(user.id, avatar_url).into(),
            ))
            .await
            .map_err(|_| HttpResponse::UnprocessableEntity().finish())?;
            Ok(HttpResponse::NoContent().finish())
        }
        _ => Ok(HttpResponse::UnprocessableEntity().finish()),
    }
}

#[cfg(not(feature = "aws-s3"))]
#[post("/")]
pub async fn upload(
    mut payload: Multipart,
    db: Data<Addr<DbExecutor>>,
    ws: Data<Addr<WsServer>>,
    fs: Data<Addr<filesystem_actor::FileSystemExecutor>>,
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
                avatar_url = Some(
                    crate::handlers::upload_avatar_image::handle_image(
                        id,
                        field,
                        disposition,
                        fs.clone(),
                    )
                    .await?,
                );
            }
            _ => continue,
        };
    }
    let user_id = match user_id {
        Some(id) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let project_id = match block_on(db.send(CurrentUserProject { user_id })) {
        Ok(Ok(user_project)) => user_project.project_id,
        _ => return Ok(HttpResponse::UnprocessableEntity().finish()),
    };

    match (user_id, avatar_url) {
        (user_id, Some(avatar_url)) => {
            let user = update_user_avatar(user_id, avatar_url.clone(), db).await?;
            ws.send(BroadcastToChannel(
                project_id,
                WsMsg::User(WsMsgUser::AvatarUrlChanged(user.id, avatar_url)),
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
) -> Result<User, actix_web::Error> {
    match db
        .send(UpdateAvatarUrl {
            user_id,
            avatar_url: Some(new_url),
        })
        .await
    {
        Ok(Ok(user)) => Ok(user),

        Ok(Err(e)) => {
            common::log::error!("{:?}", e);
            Err(actix_web::Error::from(
                HttpResponse::Unauthorized().finish(),
            ))
        }
        Err(e) => {
            common::log::error!("{:?}", e);
            Err(actix_web::Error::from(
                HttpResponse::Unauthorized().finish(),
            ))
        }
    }
}

async fn handle_token(
    mut field: Field,
    db: Data<Addr<DbExecutor>>,
) -> Result<UserId, actix_web::Error> {
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
            common::log::error!("{:?}", e);
            Err(HttpResponse::Unauthorized().finish().into())
        }
        Err(e) => {
            common::log::error!("{:?}", e);
            Err(HttpResponse::Unauthorized().finish().into())
        }
    }
}
