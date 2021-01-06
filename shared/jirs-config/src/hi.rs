use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Self { concurrency: 2 }
    }
}

impl Configuration {
    crate::rw!("highlight.toml");
}
crate::read!(Configuration);
