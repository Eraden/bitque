use std::fs::{read_to_string, write};

use serde::export::PhantomData;
use serde::{de::DeserializeOwned, Serialize};

pub struct Reader<T: DeserializeOwned + Default + Serialize> {
    __phantom: PhantomData<T>,
}

impl<T: DeserializeOwned + Default + Serialize> Reader<T> {
    pub fn read(file_name: &str) -> T {
        let _ = std::fs::create_dir_all("./config");

        let contents: String =
            match read_to_string(std::path::PathBuf::new().join("./config").join(file_name)) {
                Ok(s) => s,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        let config = T::default();
                        let _ = Writer::write(&config, file_name);
                        return config;
                    }
                    panic!("Failed to read ./config/{}. Reason: {}", file_name, e);
                }
            };

        match toml::from_str(contents.as_str()) {
            Ok(config) => config,
            _ => {
                let config = T::default();
                let _ = Writer::write(&config, file_name);
                config
            }
        }
    }
}

pub struct Writer<T: Serialize> {
    __phantom: PhantomData<T>,
}

impl<T: Serialize> Writer<T> {
    pub fn write(config: &T, file_name: &str) -> Result<(), String> {
        let _ = std::fs::create_dir_all("./config");
        let s = toml::to_string(config).map_err(|e| e.to_string())?;
        write(
            std::path::PathBuf::new().join("./config").join(file_name),
            s.as_str(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[macro_export]
macro_rules! rw {
    ($path: expr) => {
        pub fn read() -> Self {
            $crate::utils::Reader::read(Self::config_file())
        }

        pub fn write(&self) -> Result<(), String> {
            $crate::utils::Writer::write(self, Self::config_file())
        }

        #[cfg(not(test))]
        pub fn config_file() -> &'static str {
            $path
        }

        #[cfg(test)]
        pub fn config_file() -> &'static str {
            std::concat!("test.", $path)
        }
    };
}

#[macro_export]
macro_rules! read {
    ($c: ident) => {
        static mut CONFIG: Option<Box<$c>> = None;

        pub fn config() -> &'static $c {
            if unsafe { CONFIG.is_none() } {
                unsafe {
                    CONFIG = Some(Box::new($c::read()));
                };
            }
            unsafe { CONFIG.as_ref().unwrap() }
        }
    };
}
