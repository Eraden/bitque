#[cfg(feature = "aws-s3")]
use rusoto_core::Region;

use {
    crate::{
        db::{authorize_user::AuthorizeUser, DbExecutor},
        errors::ServiceError,
        middleware::authorize::token_from_headers,
    },
    actix::Addr,
    actix_web::{web::Data, HttpRequest, HttpResponse},
    jirs_data::User,
    serde::{Deserialize, Serialize},
    std::fs::*,
};

pub mod avatar;

pub async fn user_from_request(
    req: HttpRequest,
    db: &Data<Addr<DbExecutor>>,
) -> Result<User, HttpResponse> {
    let token = match token_from_headers(req.headers()) {
        Ok(uuid) => uuid,
        _ => return Err(ServiceError::Unauthorized.into_http_response()),
    };
    match db
        .send(AuthorizeUser {
            access_token: token,
        })
        .await
    {
        Ok(Ok(user)) => Ok(user),
        Ok(Err(e)) => Err(e.into_http_response()),
        _ => Err(ServiceError::Unauthorized.into_http_response()),
    }
}

#[derive(Debug)]
pub enum Protocol {
    Http,
    Https,
}

#[cfg(feature = "local-storage")]
#[derive(Serialize, Deserialize)]
pub struct FileSystemStorage {
    pub store_path: String,
    pub client_path: String,
}

#[cfg(feature = "local-storage")]
impl FileSystemStorage {
    pub fn is_empty(&self) -> bool {
        self.store_path.is_empty()
    }
}

#[cfg(feature = "aws-s3")]
#[derive(Serialize, Deserialize)]
pub struct AmazonS3Storage {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region_name: String,
}

#[cfg(feature = "aws-s3")]
impl AmazonS3Storage {
    pub fn is_empty(&self) -> bool {
        self.access_key_id.is_empty() || self.secret_access_key.is_empty() || self.bucket.is_empty()
    }

    pub fn region(&self) -> Region {
        self.region_name.parse::<Region>().unwrap_or_default()
    }

    pub fn set_variables(&self) {
        std::env::set_var("AWS_ACCESS_KEY_ID", self.access_key_id.as_str());
        std::env::set_var("AWS_SECRET_ACCESS_KEY", self.secret_access_key.as_str());
        std::env::set_var("AWS_S3_BUCKET_NAME", self.region_name.as_str());
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub port: String,
    pub bind: String,
    pub ssl: bool,
    pub tmp_dir: String,
    #[cfg(feature = "aws-s3")]
    pub s3: AmazonS3Storage,
    #[cfg(feature = "local-storage")]
    pub filesystem: FileSystemStorage,
}

impl Default for Configuration {
    #[cfg(all(feature = "local-storage", feature = "aws-s3"))]
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,

            tmp_dir: "./tmp".to_string(),
            filesystem: FileSystemStorage {
                store_path: "".to_string(),
                client_path: "/img".to_string(),
            },
            s3: AmazonS3Storage {
                access_key_id: "".to_string(),
                secret_access_key: "".to_string(),
                bucket: "".to_string(),
                region_name: Region::default().name().to_string(),
            },
        }
    }
    #[cfg(all(feature = "local-storage", not(feature = "aws-s3")))]
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,

            tmp_dir: "./tmp".to_string(),
            filesystem: FileSystemStorage {
                store_path: "./img".to_string(),
                client_path: "/img".to_string(),
            },
        }
    }

    #[cfg(all(feature = "aws-s3", not(feature = "local-storage")))]
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,

            tmp_dir: "./tmp".to_string(),
            s3: AmazonS3Storage {
                access_key_id: "".to_string(),
                secret_access_key: "".to_string(),
                bucket: "".to_string(),
                region_name: Region::default().name().to_string(),
            },
        }
    }

    #[cfg(all(not(feature = "aws-s3"), not(feature = "local-storage")))]
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,
            tmp_dir: "./tmp".to_string(),
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
