#[cfg(feature = "aws-s3")]
use rusoto_signature::Region;

use {
    serde::{Deserialize, Serialize},
    std::fs::{read_to_string, write},
};

#[derive(Debug)]
pub enum Protocol {
    Http,
    Https,
}

#[cfg(feature = "aws-s3")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AmazonS3Storage {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region_name: String,
    #[serde(default = "AmazonS3Storage::default_active")]
    pub active: bool,
}

#[cfg(feature = "aws-s3")]
impl AmazonS3Storage {
    pub fn default_active() -> bool {
        true
    }

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
    #[cfg(feature = "aws-s3")]
    pub s3: AmazonS3Storage,
}

impl Default for Configuration {
    #[cfg(feature = "aws-s3")]
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,

            s3: AmazonS3Storage {
                access_key_id: "".to_string(),
                secret_access_key: "".to_string(),
                bucket: "".to_string(),
                region_name: Region::default().name().to_string(),
                active: true,
            },
        }
    }
    #[cfg(not(feature = "aws-s3"))]
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
        let _ = std::fs::create_dir_all("./config");
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
        let _ = std::fs::create_dir_all("./config");
        let s = toml::to_string(self).map_err(|e| e.to_string())?;
        write(Self::config_file(), s.as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(test))]
    pub fn config_file() -> &'static str {
        "./config/web.toml"
    }

    #[cfg(test)]
    pub fn config_file() -> &'static str {
        "./config/web.test.toml"
    }
}
