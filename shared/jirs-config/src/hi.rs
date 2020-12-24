use std::fs::{read_to_string, write};

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
        "./config/highlight.toml"
    }

    #[cfg(test)]
    pub fn config_file() -> &'static str {
        "./config/highlight.test.toml"
    }
}
