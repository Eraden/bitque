use actix::{Handler, Message};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use uuid::Uuid;

use crate::{email_address, MailError, MailExecutor};

#[derive(Debug)]
pub struct Welcome {
    pub bind_token: Uuid,
    pub email: String,
}

impl Message for Welcome {
    type Result = Result<(), MailError>;
}

impl Handler<Welcome> for MailExecutor {
    type Result = Result<(), MailError>;

    fn handle(&mut self, msg: Welcome, _ctx: &mut Self::Context) -> Self::Result {
        use lettre::Transport;
        let transport = &mut self.transport;
        let from = email_address(self.config.from.as_str())?;
        let to = email_address(&msg.email)?;

        let html = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head><meta charset="UTF-8"></head>
            <body>
            <h1>Welcome in JIRS!</h1>
            <p>
            </p>
            <p>
                Please copy this code to sign-in single use token field: <pre><code>{bind_token}</code</pre>
            </p>
            <p>
                Notice: This token is single use and will be removed from system once you use it.
            </p>
            </body>
            </html>
            "#,
            bind_token = msg.bind_token,
        );
        if cfg!(debug_assetrions) {
            log::info!("Sending email:\n{}", html);
        }

        let mail = lettre::Message::builder()
            .to(Mailbox::new(None, to))
            .from(Mailbox::new(None, from))
            .subject("Welcome to JIRS")
            .header(ContentType::TEXT_HTML)
            .body(html)
            .map_err(|e| {
                log::error!("{:?}", e);
                MailError::MailformedBody
            })?;

        transport.send(&mail).map(|_| ()).map_err(|e| {
            log::error!("{:?}", e);
            MailError::FailedToSendEmail
        })
    }
}
