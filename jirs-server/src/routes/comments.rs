use actix::Addr;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, post, put, HttpRequest, HttpResponse};

use crate::db::comments::{CreateComment, DeleteComment, UpdateComment};
use crate::db::DbExecutor;
use crate::routes::user_from_request;

#[post("")]
pub async fn create(
    req: HttpRequest,
    payload: Json<jirs_data::CreateCommentPayload>,
    db: Data<Addr<DbExecutor>>,
) -> HttpResponse {
    let user = match user_from_request(req, &db).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    let msg = CreateComment {
        body: payload.body.clone(),
        issue_id: payload.issue_id,
        user_id: user.id,
    };
    let comment = match db.send(msg).await {
        Ok(Ok(comment)) => comment,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };

    HttpResponse::Ok().json(comment)
}

#[put("/{id}")]
pub async fn update(
    req: HttpRequest,
    path: Path<i32>,
    db: Data<Addr<DbExecutor>>,
    payload: Json<jirs_data::UpdateCommentPayload>,
) -> HttpResponse {
    let user = match user_from_request(req, &db).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let comment_id = path.into_inner();
    let body = payload.body.clone();

    let msg = UpdateComment {
        comment_id,
        body,
        user_id: user.id,
    };
    let comment = match db.send(msg).await {
        Ok(Ok(comment)) => comment,
        Ok(Err(e)) => return e.into_http_response(),
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    HttpResponse::Ok().json(comment)
}

#[delete("/{id}")]
pub async fn delete(req: HttpRequest, path: Path<i32>, db: Data<Addr<DbExecutor>>) -> HttpResponse {
    let user = match user_from_request(req, &db).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let comment_id = path.into_inner();
    let msg = DeleteComment {
        user_id: user.id,
        comment_id,
    };
    match db.send(msg).await {
        Ok(Ok(_)) => (),
        Ok(Err(_)) => (),
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    HttpResponse::NoContent().body("")
}
