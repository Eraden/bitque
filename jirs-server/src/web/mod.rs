use std::fs::*;

use actix::Addr;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::authorize_user::AuthorizeUser;
use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::middleware::authorize::token_from_headers;
use crate::models::User;

pub async fn user_from_request(
    req: HttpRequest,
    db: &Data<Addr<DbExecutor>>,
) -> Result<User, HttpResponse> {
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return Err(ServiceErrors::Unauthorized.into_http_response()),
    };
    match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => Ok(user),
        Ok(Err(e)) => Err(e.into_http_response()),
        _ => Err(ServiceErrors::Unauthorized.into_http_response()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub port: String,
    pub bind: String,
    pub ssl: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,
        }
    }
}

impl Configuration {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }

    pub fn read() -> Self {
        let contents: String = read_to_string(Self::config_file()).unwrap_or_default();
        match toml::from_str(contents.as_str()) {
            Ok(config) => config,
            _ => {
                let config = Configuration::default();
                config.write().unwrap_or_else(|e| panic!(e));
                config
            }
        }
    }

    pub fn write(&self) -> Result<(), String> {
        let s = toml::to_string(self).map_err(|e| e.to_string())?;
        write(Self::config_file(), s.as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(test))]
    pub fn config_file() -> &'static str {
        "web.toml"
    }

    #[cfg(test)]
    pub fn config_file() -> &'static str {
        "web.test.toml"
    }
}
