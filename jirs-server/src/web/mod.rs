use std::fs::*;

use actix::Addr;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use jirs_data::User;

use crate::db::authorize_user::AuthorizeUser;
use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::middleware::authorize::token_from_headers;

pub mod avatar;

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

#[derive(Debug)]
pub enum Protocol {
    Http,
    Https,
}

#[derive(Serialize, Deserialize)]
pub struct FileSystemStorage {
    pub store_path: String,
    pub client_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct AmazonS3Storage {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
}

#[derive(Serialize, Deserialize)]
pub enum Storage {
    FileSystem,
    AmazonS3,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub port: String,
    pub bind: String,
    pub ssl: bool,
    pub storage: Storage,
    pub filesystem: Option<FileSystemStorage>,
    pub s3: Option<AmazonS3Storage>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,
            storage: Storage::FileSystem,
            filesystem: Some(FileSystemStorage {
                store_path: "./tmp".to_string(),
                client_path: "/img".to_string(),
            }),
            s3: None,
        }
    }
}

impl Configuration {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }

    pub fn full_addr(&self) -> String {
        match self.protocol() {
            Protocol::Http => format!("http://{}", self.addr()),
            Protocol::Https => format!("https://{}", self.addr()),
        }
    }

    pub fn protocol(&self) -> Protocol {
        if self.bind.as_str() == "0.0.0.0"
            || self.bind.as_str().starts_with("127.")
            || self.bind.as_str() == "localhost"
            || self.bind.as_str().ends_with(".lvh.me")
        {
            Protocol::Http
        } else {
            Protocol::Https
        }
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
