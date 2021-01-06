#[derive(serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub database_url: String,
}

impl Default for Configuration {
    fn default() -> Self {
        let database_url = if cfg!(test) {
            "postgres://postgres@localhost:5432/jirs_test".to_string()
        } else {
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres@localhost:5432/jirs".to_string())
        };
        Self {
            concurrency: 2,
            database_url,
        }
    }
}

impl Configuration {
    crate::rw!("db.toml");
}
crate::read!(Configuration);
