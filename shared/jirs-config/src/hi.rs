use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    #[serde(default = "Configuration::default_theme")]
    pub theme: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            concurrency: 2,
            theme: Self::default_theme(),
        }
    }
}

impl Configuration {
    crate::rw!("highlight.toml");

    fn default_theme() -> String {
        "Github".to_string()
    }
}
crate::read!(Configuration);
