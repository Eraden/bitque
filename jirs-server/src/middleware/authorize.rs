use actix_service::{Service, Transform};
use actix_web::http::header::{self};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Either, FutureExt, LocalBoxFuture, Ready};
use serde::Serialize;
use std::task::{Context, Poll};

type Db = actix_web::web::Data<actix::Addr<crate::db::DbExecutor>>;

#[derive(Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

#[derive(Default)]
pub struct Authorize;

impl<S, B> Transform<S> for Authorize
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizeMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizeMiddleware { service })
    }
}

pub struct AuthorizeMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthorizeMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<
        Ready<Result<Self::Response, Error>>,
        LocalBoxFuture<'static, Result<Self::Response, Error>>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let header_value = req.headers().get(&header::AUTHORIZATION);
        let access_token =
            match header_value.and_then(|s| parse_bearer(s.to_str().unwrap_or_default())) {
                Some(token) => token,
                _ => {
                    println!("No access token found");
                    let res = HttpResponse::Unauthorized().body("").into_body();
                    return Either::Left(ok(req.into_response(res)));
                }
            };

        let db_executor: Db = match req.app_data() {
            Some(d) => d,
            _ => {
                let response = ErrorResponse {
                    errors: vec!["Database connection failed".to_string()],
                };
                let res = HttpResponse::InternalServerError()
                    .json(response)
                    .into_body();
                return Either::Left(ok(req.into_response(res)));
            }
        };

        let fut = self.service.call(req);

        Either::Right(
            async move {
                match check_token(access_token, db_executor).await {
                    Some(user) => (),
                    _ => (),
                };
                fut.await
            }
            .boxed_local(),
        )
    }
}

async fn check_token(access_token: uuid::Uuid, db_executor: Db) -> Option<crate::models::User> {
    use crate::db::authorize_user::AuthorizeUser;
    match db_executor.send(AuthorizeUser { access_token }).await {
        Ok(Ok(user)) => Some(user),
        _ => None,
    }
}

fn parse_bearer(header: &str) -> Option<uuid::Uuid> {
    if !header.starts_with("Bearer ") {
        return None;
    }
    let (_bearer, token) = header.split_at(7);
    match uuid::Uuid::parse_str(token) {
        Ok(u) => Some(u),
        _ => None,
    }
}
