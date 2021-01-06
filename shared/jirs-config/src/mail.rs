#[derive(serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub user: String,
    pub pass: String,
    pub host: String,
    pub from: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            concurrency: 2,
            user: "apikey".to_string(),
            pass: "YOUR-TOKEN".to_string(),
            host: "smtp.sendgrid.net".to_string(),
            from: "contact@jirs.pl".to_string(),
        }
    }
}

impl Configuration {
    crate::rw!("mail.toml");
}
crate::read!(Configuration);
