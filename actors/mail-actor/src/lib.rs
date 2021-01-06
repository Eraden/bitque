use actix::{Actor, SyncContext};

pub mod invite;
pub mod welcome;

pub type MailTransport = lettre::SmtpTransport;

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

fn mail_client(config: &jirs_config::mail::Configuration) -> lettre::SmtpClient {
    let jirs_config::mail::Configuration {
        user: mail_user,
        pass: mail_pass,
        host: mail_host,
        ..
    } = &config;

    lettre::SmtpClient::new_simple(mail_host)
        .expect("Failed to init SMTP client")
        .credentials(lettre::smtp::authentication::Credentials::new(
            mail_user.clone(),
            mail_pass.clone(),
        ))
        .connection_reuse(lettre::smtp::ConnectionReuseParameters::ReuseUnlimited)
        .smtp_utf8(true)
}

fn mail_transport(config: &jirs_config::mail::Configuration) -> MailTransport {
    mail_client(config).transport()
}
