use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub store_path: String,
    pub client_path: String,
    pub tmp_path: String,
    pub concurrency: usize,
    #[serde(default)]
    pub active: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            store_path: "./uploads".to_string(),
            client_path: "/uploads".to_string(),
            tmp_path: "/tmp".to_string(),
            concurrency: 2,
            active: true,
        }
    }
}

impl Configuration {
    pub fn is_empty(&self) -> bool {
        self.store_path.is_empty()
    }

    crate::rw!("fs.toml");
}
crate::read!(Configuration);
