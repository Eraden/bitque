use {
    actix_service::{Service, Transform},
    actix_web::{
        dev::{ServiceRequest, ServiceResponse},
        http::header::{self},
        http::HeaderMap,
        Error,
    },
    futures::future::{ok, FutureExt, LocalBoxFuture, Ready},
    jirs_data::User,
    std::task::{Context, Poll},
};

type Db = actix_web::web::Data<database_actor::DbPool>;

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
                    let res = crate::errors::ServiceError::DatabaseConnectionLost
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
) -> std::result::Result<uuid::Uuid, crate::errors::ServiceError> {
    headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| crate::errors::ServiceError::Unauthorized)
        .map(|h| h.to_str().unwrap_or_default())
        .and_then(|s| parse_bearer(s))
}

fn check_token(
    headers: &HeaderMap,
    pool: Db,
) -> std::result::Result<User, crate::errors::ServiceError> {
    token_from_headers(headers).and_then(|access_token| {
        use database_actor::authorize_user::AuthorizeUser;
        let conn = pool
            .get()
            .map_err(|_| crate::errors::ServiceError::DatabaseConnectionLost)?;
        AuthorizeUser { access_token }
            .execute(&conn)
            .map_err(|_| crate::errors::ServiceError::Unauthorized)
    })
}

fn parse_bearer(header: &str) -> Result<uuid::Uuid, crate::errors::ServiceError> {
    if !header.starts_with("Bearer ") {
        return Err(crate::errors::ServiceError::Unauthorized);
    }
    let (_bearer, token) = header.split_at(7);
    uuid::Uuid::parse_str(token).map_err(|_e| crate::errors::ServiceError::Unauthorized)
}
