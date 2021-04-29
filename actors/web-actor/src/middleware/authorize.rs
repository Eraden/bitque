use actix_web::http::header::{self};
use actix_web::http::HeaderMap;

pub fn token_from_headers(
    headers: &HeaderMap,
) -> std::result::Result<uuid::Uuid, crate::errors::ServiceError> {
    headers
        .get(header::AUTHORIZATION).ok_or(crate::errors::ServiceError::Unauthorized)
        .map(|h| h.to_str().unwrap_or_default())
        .and_then(|s| parse_bearer(s))
}

fn parse_bearer(header: &str) -> Result<uuid::Uuid, crate::errors::ServiceError> {
    if !header.starts_with("Bearer ") {
        return Err(crate::errors::ServiceError::Unauthorized);
    }
    let (_bearer, token) = header.split_at(7);
    uuid::Uuid::parse_str(token).map_err(|_e| crate::errors::ServiceError::Unauthorized)
}
