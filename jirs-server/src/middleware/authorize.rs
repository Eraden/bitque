use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::http::header::{self};
use actix_web::http::HeaderMap;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureExt, LocalBoxFuture, Ready};

use jirs_data::User;

use crate::db::SyncQuery;

type Db = actix_web::web::Data<crate::db::DbPool>;

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
    type Transform = AuthorizeMiddleware<S>;
    type InitError = ();
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
    type Future = LocalBoxFuture<'static, Result<Self::Response, Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let pool: &Db = match req.app_data::<Db>() {
            Some(d) => d,
            _ => {
                return async move {
                    let res = crate::errors::ServiceErrors::DatabaseConnectionLost
                        .into_http_response()
                        .into_body();
                    Ok(req.into_response(res))
                }
                .boxed_local();
            }
        };

        match check_token(req.headers(), pool.clone()) {
            std::result::Result::Err(e) => {
                return async move {
                    let res = e.into_http_response().into_body();
                    Ok(req.into_response(res))
                }
                .boxed_local();
            }
            Ok(_user) => {}
        };

        let fut = self.service.call(req);
        async move { fut.await }.boxed_local()
    }
}

pub fn token_from_headers(
    headers: &HeaderMap,
) -> std::result::Result<uuid::Uuid, crate::errors::ServiceErrors> {
    headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| crate::errors::ServiceErrors::Unauthorized)
        .map(|h| h.to_str().unwrap_or_default())
        .and_then(|s| parse_bearer(s))
}

fn check_token(
    headers: &HeaderMap,
    pool: Db,
) -> std::result::Result<User, crate::errors::ServiceErrors> {
    token_from_headers(headers).and_then(|access_token| {
        use crate::db::authorize_user::AuthorizeUser;
        AuthorizeUser { access_token }.handle(&pool)
    })
}

fn parse_bearer(header: &str) -> Result<uuid::Uuid, crate::errors::ServiceErrors> {
    if !header.starts_with("Bearer ") {
        return Err(crate::errors::ServiceErrors::Unauthorized);
    }
    let (_bearer, token) = header.split_at(7);
    uuid::Uuid::parse_str(token).map_err(|_e| crate::errors::ServiceErrors::Unauthorized)
}
