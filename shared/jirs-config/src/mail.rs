use std::fs::{read_to_string, write};

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
    fn config_file() -> &'static str {
        "./config/mail.toml"
    }

    #[cfg(test)]
    fn config_file() -> &'static str {
        "./config/mail.test.toml"
    }
}
