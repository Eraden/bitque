#[derive(serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Self { concurrency: 2 }
    }
}

impl Configuration {
    crate::rw!("websocket.toml");
}
crate::read!(Configuration);
