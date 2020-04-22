use std::fs::*;

use actix::{Actor, SyncContext};
use lettre;
use serde::{Deserialize, Serialize};

pub mod invite;
pub mod welcome;

pub type MailTransport = lettre::SmtpTransport;

pub struct MailExecutor {
    pub transport: MailTransport,
    pub config: Configuration,
}

impl Actor for MailExecutor {
    type Context = SyncContext<Self>;
}

impl Default for MailExecutor {
    fn default() -> Self {
        let config = Configuration::read();
        Self {
            transport: mail_transport(&config),
            config,
        }
    }
}

fn mail_client(config: &Configuration) -> lettre::SmtpClient {
    let mail_user = config.user.as_str();
    let mail_pass = config.pass.as_str();
    let mail_host = config.host.as_str();

    lettre::SmtpClient::new_simple(mail_host)
        .expect("Failed to init SMTP client")
        .credentials(lettre::smtp::authentication::Credentials::new(
            mail_user.to_string(),
            mail_pass.to_string(),
        ))
        .connection_reuse(lettre::smtp::ConnectionReuseParameters::ReuseUnlimited)
        .smtp_utf8(true)
}

fn mail_transport(config: &Configuration) -> MailTransport {
    mail_client(config).transport()
}

#[derive(Serialize, Deserialize)]
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
        let s = toml::to_string(self).map_err(|e| e.to_string())?;
        write(Self::config_file(), s.as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(test))]
    fn config_file() -> &'static str {
        "mail.toml"
    }

    #[cfg(test)]
    fn config_file() -> &'static str {
        "mail.test.toml"
    }
}
