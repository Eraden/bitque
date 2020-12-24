use actix_web::HttpResponse;

use jirs_data::{msg::WsError, ErrorResponse};

const TOKEN_NOT_FOUND: &str = "Token not found";
const DATABASE_CONNECTION_FAILED: &str = "Database connection failed";

#[derive(Debug)]
pub enum HighlightError {
    UnknownLanguage,
    UnknownTheme,
    ResultUnserializable,
}

#[derive(Debug)]
pub enum ServiceError {
    Unauthorized,
    DatabaseConnectionLost,
    DatabaseQueryFailed(String),
    RecordNotFound(String),
    RegisterCollision,
    Error(WsError),
    Highlight(HighlightError),
}

impl ServiceError {
    pub fn into_http_response(self) -> HttpResponse {
        self.into()
    }
}

impl Into<HttpResponse> for ServiceError {
    fn into(self) -> HttpResponse {
        match self {
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json(ErrorResponse {
                errors: vec![TOKEN_NOT_FOUND.to_owned()],
            }),
            ServiceError::DatabaseConnectionLost => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    errors: vec![DATABASE_CONNECTION_FAILED.to_owned()],
                })
            }
            ServiceError::DatabaseQueryFailed(error) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    errors: vec![error],
                })
            }
            ServiceError::RecordNotFound(resource_name) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    errors: vec![format!("Resource not found {}", resource_name)],
                })
            }
            ServiceError::RegisterCollision => HttpResponse::Unauthorized().json(ErrorResponse {
                errors: vec!["Register collision".to_string()],
            }),
            ServiceError::Error(error) => HttpResponse::BadRequest().json(ErrorResponse {
                errors: vec![error.to_str().to_string()],
            }),
            ServiceError::Highlight(HighlightError::UnknownTheme) => HttpResponse::BadRequest()
                .json(ErrorResponse::single(
                    "Code highlight Failed. Unexpected theme",
                )),
            ServiceError::Highlight(HighlightError::UnknownLanguage) => HttpResponse::BadRequest()
                .json(ErrorResponse::single(
                    "Can't highlight in given language. It's unknown",
                )),
            ServiceError::Highlight(HighlightError::ResultUnserializable) => {
                HttpResponse::BadRequest().json(ErrorResponse::single(
                    "Highlight succeed but result can't be send",
                ))
            }
        }
    }
}
