use crate::db::authorize_user::AuthorizeUser;
use crate::db::issues::LoadProjectIssues;
use crate::db::projects::LoadCurrentProject;
use crate::db::users::LoadProjectUsers;
use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::middleware::authorize::token_from_headers;
use actix::Addr;
use actix_web::web::Data;
use actix_web::{get, put, HttpRequest, HttpResponse};
use jirs_data::ResponseData;

#[get("")]
pub async fn project_with_users_and_issues(
    req: HttpRequest,
    db: Data<Addr<DbExecutor>>,
) -> HttpResponse {
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    let user = match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => user,
        _ => return crate::errors::ServiceErrors::Unauthorized.into_http_response(),
    };
    let issues_future = db.send(LoadProjectIssues {
        project_id: user.project_id,
    });
    let project_future = db.send(LoadCurrentProject {
        project_id: user.project_id,
    });
    let users_future = db.send(LoadProjectUsers {
        project_id: user.project_id,
    });
    let issues_result = issues_future.await;
    let issues = match issues_result {
        Ok(Ok(issues)) => issues,
        _ => return ServiceErrors::DatabaseConnectionLost.into_http_response(),
    };
    let project_result = project_future.await;
    let project = match project_result {
        Ok(Ok(project)) => project,
        _ => return ServiceErrors::DatabaseConnectionLost.into_http_response(),
    };
    let users_result = users_future.await;
    let users = match users_result {
        Ok(Ok(users)) => users,
        _ => return ServiceErrors::DatabaseConnectionLost.into_http_response(),
    };
    let res = jirs_data::FullProject {
        id: project.id,
        name: project.name,
        url: project.url,
        description: project.description,
        category: project.category,
        created_at: project.created_at,
        updated_at: project.updated_at,
        issues: issues
            .into_iter()
            .map(|i| {
                let mut issue: jirs_data::Issue = i.into();
                issue.user_ids = users.iter().map(|u| u.id).collect();
                issue
            })
            .collect(),
        users: users.into_iter().map(|u| u.into()).collect(),
    };
    HttpResponse::Ok().json(res.into_response())
}

#[put("")]
pub async fn update(_req: HttpRequest, _db: Data<Addr<DbExecutor>>) -> HttpResponse {
    HttpResponse::NotImplemented().body("")
}
