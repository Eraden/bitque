use std::io::Write;

#[cfg(feature = "local-storage")]
use filesystem_actor;
use {
    actix::Addr,
    actix_multipart::{Field, Multipart},
    actix_web::{http::header::ContentDisposition, post, web, web::Data, Error, HttpResponse},
    database_actor::{
        authorize_user::AuthorizeUser, user_projects::CurrentUserProject, users::UpdateAvatarUrl,
        DbExecutor,
    },
    futures::{executor::block_on, StreamExt, TryStreamExt},
    jirs_data::{User, UserId, WsMsg},
    websocket_actor::server::{InnerMsg::BroadcastToChannel, WsServer},
};

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
