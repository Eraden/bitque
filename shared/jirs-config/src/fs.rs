use std::fs::{read_to_string, write};

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

    pub fn read() -> Self {
        let _ = std::fs::create_dir_all("./config");
        let contents: String = read_to_string(Self::config_file()).unwrap_or_default();
        let config = match toml::from_str(contents.as_str()) {
            Ok(config) => config,
            _ => {
                let config = Configuration::default();
                config.write().unwrap_or_else(|e| panic!(e));
                config
            }
        };
        let _ = std::fs::create_dir_all(config.tmp_path.as_str()).map_err(|e| e.to_string());
        let _ = std::fs::create_dir_all(config.store_path.as_str()).map_err(|e| e.to_string());
        config
    }

    pub fn write(&self) -> Result<(), String> {
        let _ = std::fs::create_dir_all("./config");
        let s = toml::to_string(self).map_err(|e| e.to_string())?;
        write(Self::config_file(), s.as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(test))]
    pub fn config_file() -> &'static str {
        "./config/fs.toml"
    }

    #[cfg(test)]
    pub fn config_file() -> &'static str {
        "./config/fs.test.toml"
    }
}
