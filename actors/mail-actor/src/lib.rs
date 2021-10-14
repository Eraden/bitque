use std::convert::TryFrom;

use actix::{Actor, SyncContext};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::PoolConfig;

pub mod invite;
pub mod welcome;

pub type MailTransport = lettre::SmtpTransport;

#[derive(Debug)]
pub enum MailError {
    EmailWithoutUser,
    EmailWithoutDomain,
    InvalidEmailAddress,
    FailedToSendEmail,
    MailformedBody,
}

pub struct MailExecutor {
    pub transport: MailTransport,
    pub config: jirs_config::mail::Configuration,
}

impl Actor for MailExecutor {
    type Context = SyncContext<Self>;
}

impl Default for MailExecutor {
    fn default() -> Self {
        let config = jirs_config::mail::Configuration::read();
        Self {
            transport: mail_transport(&config),
            config,
        }
    }
}

fn mail_client(config: &jirs_config::mail::Configuration) -> lettre::SmtpTransport {
    let jirs_config::mail::Configuration {
        user: mail_user,
        pass: mail_pass,
        host: mail_host,
        ..
    } = &config;

    lettre::SmtpTransport::relay(mail_host)
        .expect("Failed to init SMTP client")
        .credentials(Credentials::new(mail_user.clone(), mail_pass.clone()))
        .pool_config(PoolConfig::default())
        .build()
}

fn mail_transport(config: &jirs_config::mail::Configuration) -> MailTransport {
    mail_client(config)
}

pub fn email_address(email: &str) -> Result<lettre::Address, MailError> {
    let (user, domain) = {
        let mut parts = email.split("@");
        (
            match parts.next() {
                Some(s) => s,
                None => return Err(MailError::EmailWithoutUser),
            },
            match parts.next() {
                Some(s) => s,
                None => return Err(MailError::EmailWithoutDomain),
            },
        )
    };
    lettre::Address::try_from((user, domain)).map_err(|_| MailError::InvalidEmailAddress)
}
