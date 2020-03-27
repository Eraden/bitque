use crate::models::ErrorResponse;
use actix_web::HttpResponse;

const TOKEN_NOT_FOUND: &str = "Token not found";
const DATABASE_CONNECTION_FAILED: &str = "Database connection failed";

pub enum ServiceErrors {
    Unauthorized,
    DatabaseConnectionLost,
    RecordNotFound(String),
}

impl ServiceErrors {
    pub fn into_http_response(self) -> HttpResponse {
        self.into()
    }
}

impl Into<HttpResponse> for ServiceErrors {
    fn into(self) -> HttpResponse {
        match self {
            ServiceErrors::Unauthorized => HttpResponse::Unauthorized().json(ErrorResponse {
                errors: vec![TOKEN_NOT_FOUND.to_owned()],
            }),
            ServiceErrors::DatabaseConnectionLost => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    errors: vec![DATABASE_CONNECTION_FAILED.to_owned()],
                })
            }
            ServiceErrors::RecordNotFound(resource_name) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    errors: vec![format!("Resource not found {}", resource_name)],
                })
            }
        }
    }
}
