use actix::{Handler, Message};
use lettre::message::Mailbox;
use uuid::Uuid;

use crate::{email_address, MailError, MailExecutor};

#[derive(Debug)]
pub struct Invite {
    pub bind_token: Uuid,
    pub email: String,
    pub inviter_name: String,
}

impl Message for Invite {
    type Result = Result<(), MailError>;
}

impl Handler<Invite> for MailExecutor {
    type Result = Result<(), MailError>;

    fn handle(&mut self, msg: Invite, _ctx: &mut Self::Context) -> Self::Result {
        use lettre::Transport;
        let transport = &mut self.transport;
        let addr = jirs_config::web::Configuration::read().full_addr();
        let from = email_address(self.config.from.as_str())?;
        let to = email_address(&msg.email)?;

        let html = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head><meta charset="UTF-8"></head>
            <body>
            <h1>You have been invited to project by {inviter_name}!</h1>
            <p>
            </p>
            <p>
                Please click this link: <a href="{addr}/invite?token={bind_token}">{addr}/invite?token={bind_token}</a>
            </p>
            </body>
            </html>
            "#,
            bind_token = msg.bind_token,
            inviter_name = msg.inviter_name,
            addr = addr,
        );

        let mail = lettre::Message::builder()
            .to(Mailbox::new(None, to))
            .from(Mailbox::new(None, from))
            .subject("Invitation to JIRS project")
            .body(html)
            .map_err(|e| {
                log::error!("{:?}", e);
                MailError::MailformedBody
            })?;

        transport.send(&mail).map(|_| ()).map_err(|e| {
            log::error!("Mailer: {}", e);
            MailError::FailedToSendEmail
        })
    }
}
