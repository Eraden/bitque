use std::collections::HashMap;

use actix::Addr;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put, HttpRequest, HttpResponse};

use jirs_data::ResponseData;

use crate::db::authorize_user::AuthorizeUser;
use crate::db::comments::LoadIssueComments;
use crate::db::issues::{CreateIssue, DeleteIssue, LoadIssue, UpdateIssue};
use crate::db::users::{LoadIssueAssignees, LoadProjectUsers};
use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::middleware::authorize::token_from_headers;
use crate::routes::user_from_request;

#[get("")]
pub async fn project_issues() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<!DOCTYPE html><html><head><title>Issues</title></head><body>Foo</body></html>")
}

#[get("/{id}")]
pub async fn issue_with_users_and_comments(
    req: HttpRequest,
    path: Path<i32>,
    db: Data<Addr<DbExecutor>>,
) -> HttpResponse {
    let issue_id = path.into_inner();
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    let _user = match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => user,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };

    match load_issue(issue_id, db).await {
        Ok(full_issue) => HttpResponse::Ok().json(full_issue.into_response()),
        Err(e) => e.into_http_response(),
    }
}

#[post("")]
pub async fn create(
    req: HttpRequest,
    payload: Json<jirs_data::CreateIssuePayload>,
    db: Data<Addr<DbExecutor>>,
) -> HttpResponse {
    let user = match user_from_request(req, &db).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let msg = CreateIssue {
        title: payload.title.clone(),
        issue_type: payload.issue_type.clone(),
        status: payload.status.clone(),
        priority: payload.priority.clone(),
        description: payload.description.clone(),
        description_text: payload.description_text.clone(),
        estimate: payload.estimate.clone(),
        time_spent: payload.time_spent.clone(),
        time_remaining: payload.time_remaining.clone(),
        project_id: payload.project_id,
        reporter_id: user.id,
        user_ids: payload.user_ids.clone(),
    };
    match db.send(msg).await {
        Ok(Ok(issue)) => HttpResponse::Ok().json(issue),
        Ok(Err(e)) => e.into_http_response(),
        _ => ServiceErrors::DatabaseConnectionLost.into_http_response(),
    }
}

#[put("/{id}")]
pub async fn update(
    req: HttpRequest,
    payload: Json<jirs_data::UpdateIssuePayload>,
    path: Path<i32>,
    db: Data<Addr<DbExecutor>>,
) -> HttpResponse {
    let issue_id = path.into_inner();
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    let _user = match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => user,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    let signal = UpdateIssue {
        issue_id,
        title: payload.title.clone(),
        issue_type: payload.issue_type.clone(),
        status: payload.status.clone(),
        priority: payload.priority.clone(),
        list_position: payload.list_position.clone(),
        description: payload.description.clone(),
        description_text: payload.description_text.clone(),
        estimate: payload.estimate.clone(),
        time_spent: payload.time_spent.clone(),
        time_remaining: payload.time_remaining.clone(),
        project_id: payload.project_id.clone(),
        user_ids: payload.user_ids.clone(),
        users: payload.users.clone(),
    };
    match db.send(signal).await {
        Ok(Ok(_)) => (),
        Ok(Err(e)) => return e.into_http_response(),
        _ => return ServiceErrors::DatabaseConnectionLost.into_http_response(),
    };
    match load_issue(issue_id, db).await {
        Ok(full_issue) => HttpResponse::Ok().json(full_issue.into_response()),
        Err(e) => e.into_http_response(),
    }
}

#[delete("/{id}")]
pub async fn delete(req: HttpRequest, path: Path<i32>, db: Data<Addr<DbExecutor>>) -> HttpResponse {
    let _user = match user_from_request(req, &db).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let issue_id = path.into_inner();
    let msg = DeleteIssue { issue_id };
    match db.send(msg).await {
        Ok(Ok(_)) => HttpResponse::NoContent().body(""),
        Ok(Err(e)) => e.into_http_response(),
        _ => ServiceErrors::DatabaseConnectionLost.into_http_response(),
    }
}

async fn load_issue(
    issue_id: i32,
    db: Data<Addr<DbExecutor>>,
) -> Result<jirs_data::FullIssue, ServiceErrors> {
    let issue_future = db.send(LoadIssue { issue_id });
    let assignees_future = db.send(LoadIssueAssignees { issue_id });
    let comments_future = db.send(LoadIssueComments { issue_id });
    let issue_result = issue_future.await;
    let issue = match issue_result {
        Ok(Ok(issue)) => issue,
        _ => return Err(ServiceErrors::DatabaseConnectionLost),
    };
    let users = match db
        .send(LoadProjectUsers {
            project_id: issue.project_id,
        })
        .await
    {
        Ok(Ok(users)) => users,
        _ => return Err(ServiceErrors::DatabaseConnectionLost),
    };
    let mut full_issue: jirs_data::FullIssue = issue.into();
    let assignees_result = assignees_future.await;
    let assignees = match assignees_result {
        Ok(Ok(assignees)) => assignees,
        _ => return Err(ServiceErrors::DatabaseConnectionLost),
    };
    let mut user_map = HashMap::new();
    for user in users.into_iter() {
        user_map.insert(user.id, user);
    }
    let comments_result = comments_future.await;
    let comments = match comments_result {
        Ok(Ok(comments)) => comments,
        _ => return Err(ServiceErrors::DatabaseConnectionLost),
    };
    full_issue.user_ids = assignees.iter().map(|u| u.id).collect();
    full_issue.comments = comments
        .into_iter()
        .map(|c| {
            let mut comment: jirs_data::Comment = c.into();
            comment.user = user_map.get(&comment.user_id).map(|user| user.into());
            comment
        })
        .collect();
    Ok(full_issue)
}
