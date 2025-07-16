 use super::{Email, DEFAULT_FROM_SENDER};
use crate::common::error::{Error, Result};
use lettre::{
    message::MultiPart, transport::smtp::authentication::Credentials, AsyncTransport, Message,
    Tokio1Executor, Transport,
}; 
use tracing::info;

#[derive(Clone)]
pub enum EmailTransport {
    /// SMTP (Simple Mail Transfer Protocol) transport.
    Smtp(lettre::AsyncSmtpTransport<lettre::Tokio1Executor>),
    /// Test/stub transport for testing purposes.
    Test(lettre::transport::stub::StubTransport),
}

#[derive(Clone)]
pub struct EmailSender {
    pub transport: EmailTransport,
}

impl EmailSender {
    pub fn smtp(config: &crate:: config::appconfig::SmtpMailer) -> Result<Self> {
        let mut email_builder = if config.secure {
            lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
                .map_err(|error| {
                    tracing::error!(err.msg = %error, err.detail = ?error, "smtp_init_error");
                    Error::Message("error initialize smtp mailer".to_string())
                })?
                .port(config.port)
        } else {
            lettre::AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
                .port(config.port)
        };

        if let Some(auth) = config.auth.as_ref() {
            email_builder = email_builder
                .credentials(Credentials::new(auth.user.clone(), auth.password.clone()));
        }

        Ok(Self {
            transport: EmailTransport::Smtp(email_builder.build()),
        })
    }

    pub async fn mail(&self, email: &Email) -> Result<String> {
        let content = MultiPart::alternative_plain_html(email.text.clone(), email.html.clone());
        let mut builder = Message::builder()
            .from(
                email
                    .from
                    .clone()
                    .unwrap_or_else(|| DEFAULT_FROM_SENDER.to_string())
                    .parse()?,
            )
            .to(email.to.parse()?);

        if let Some(reply_to) = &email.reply_to {
            builder = builder.reply_to(reply_to.parse()?);
        }

        let msg = builder
            .subject(email.subject.clone())
            .multipart(content)
            .map_err(|error| {
                tracing::error!(err.msg = %error, err.detail = ?error, "email_building_error");
                Error::Message("error building email message".to_owned())
            })
            ?;

        match &self.transport {
            EmailTransport::Smtp(xp) => {                
               // xp.send(msg).await?;
                match xp.send(msg).await {
                    Ok(_) => info!("Email sent successfully!"),
                    Err(e) => info!("Could not send email: {e:?}"),
                }
            }
            EmailTransport::Test(xp) => {
                xp.send(&msg)
                    .map_err(|_| Error::Message("sending email error".into()))
                   ?;
            }
        };
        Ok("sc".to_owned())
    }
}
 