use rusoto_signature::Region;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region_name: String,
    #[serde(default = "Configuration::default_active")]
    pub active: bool,
    pub concurrency: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            access_key_id: "".to_string(),
            secret_access_key: "".to_string(),
            bucket: "".to_string(),
            region_name: Region::default().name().to_string(),
            active: true,
            concurrency: 2,
        }
    }
}

impl Configuration {
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

    crate::rw!("amazon.toml");
}

crate::read!(Configuration);
