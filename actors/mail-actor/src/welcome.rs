use actix::{Handler, Message};
use uuid::Uuid;

use crate::MailExecutor;

#[derive(Debug)]
pub struct Welcome {
    pub bind_token: Uuid,
    pub email: String,
}

impl Message for Welcome {
    type Result = Result<(), String>;
}

impl Handler<Welcome> for MailExecutor {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: Welcome, _ctx: &mut Self::Context) -> Self::Result {
        use lettre::Transport;
        let transport = &mut self.transport;
        let from = self.config.from.as_str();

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

        let email = lettre_email::Email::builder()
            .from(from)
            .to(msg.email.as_str())
            .html(html.as_str())
            .subject("Welcome to JIRS")
            .build()
            .map_err(|_| "Email is not valid".to_string())?;

        transport
            .send(email.into())
            .map(|_| ())
            .map_err(|e| format!("Mailer: {}", e))
    }
}
