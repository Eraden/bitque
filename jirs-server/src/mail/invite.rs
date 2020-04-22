use actix::{Handler, Message};
// use lettre;
// use lettre_email;
use uuid::Uuid;

use crate::mail::MailExecutor;

#[derive(Debug)]
pub struct Invite {
    pub bind_token: Uuid,
    pub email: String,
    pub inviter_name: String,
}

impl Message for Invite {
    type Result = Result<(), String>;
}

impl Handler<Invite> for MailExecutor {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: Invite, _ctx: &mut Self::Context) -> Self::Result {
        use lettre::Transport;
        let transport = &mut self.transport;
        let from = self.config.from.as_str();
        let addr = crate::web::Configuration::read().full_addr();

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

        let email = lettre_email::Email::builder()
            .from(from)
            .to(msg.email.as_str())
            .html(html.as_str())
            .subject("Invitation to JIRS project")
            .build()
            .map_err(|_| "Email is not valid".to_string())?;

        transport
            .send(email.into())
            .and_then(|_| Ok(()))
            .map_err(|e| format!("Mailer: {}", e))
    }
}
