use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Protocol {
    Http,
    Https,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub port: String,
    pub bind: String,
    pub ssl: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            concurrency: 2,
            port: "5000".to_string(),
            bind: "0.0.0.0".to_string(),
            ssl: false,
        }
    }
}

impl Configuration {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }

    pub fn full_addr(&self) -> String {
        match self.protocol() {
            Protocol::Http => format!("http://{}", self.addr()),
            Protocol::Https => format!("https://{}", self.addr()),
        }
    }

    pub fn protocol(&self) -> Protocol {
        if self.bind.as_str() == "0.0.0.0"
            || self.bind.as_str().starts_with("127.")
            || self.bind.as_str() == "localhost"
            || self.bind.as_str().ends_with(".lvh.me")
        {
            Protocol::Http
        } else {
            Protocol::Https
        }
    }

    crate::rw!("web.toml");
}
crate::read!(Configuration);
