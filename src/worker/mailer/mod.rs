use serde::{Deserialize, Serialize};
use crate::worker::common::{Worker,WorkerOpts};
mod email_sender;
mod template;
pub const DEFAULT_FROM_SENDER: &str = "XXXX科技 <xxxx@163.com>";
use crate::config::APPCOFIG;
use crate::worker::AppWorker;
use crate::common::error::Result;
use async_trait::async_trait;
pub use email_sender::EmailSender;
use tokio::sync::OnceCell;

static EMAILSENDER: OnceCell<EmailSender> = OnceCell::const_new();
use self::template::Template;

async fn email_init() -> EmailSender {
    let config = APPCOFIG.mailer.clone().unwrap().smtp.unwrap();
    EmailSender::smtp(&config).unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Email {
    /// Mailbox to `From` header
    pub from: Option<String>,
    /// Mailbox to `To` header
    pub to: String,
    /// Mailbox to `ReplyTo` header
    pub reply_to: Option<String>,
    /// Subject header to message
    pub subject: String,
    /// Plain text message
    pub text: String,
    /// HTML template
    pub html: String,
}
#[derive(Clone)]
pub struct MailerWorker {}

impl AppWorker<Email> for MailerWorker {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Worker<Email> for MailerWorker {
    /// Returns options for the mailer worker, specifying the queue to process.
    fn opts() -> WorkerOpts<Email, Self> { 
        WorkerOpts::new().queue("mailer")
    }

    /// Performs the email sending operation using the provided [`AppContext`]
    /// and email details.
    async fn perform(&self, email: Email) ->  Result<()> {
        let mailer = EMAILSENDER.get_or_init(email_init).await;
        let _ = mailer.mail(&email).await;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub from: Option<String>,
    pub to: String,
    pub reply_to: Option<String>,
    pub locals: serde_json::Value,
}

pub async fn mail_template(dir: String, args: Args) -> Result<()> {
    let content = Template::new(dir).render(&args.locals)?;
    mail(&Email {
        from: args.from.clone(),
        to: args.to.clone(),
        reply_to: args.reply_to.clone(),
        subject: content.subject,
        text: content.text,
        html: content.html,
    })
    .await 
}

async fn mail(email: &Email) -> Result<()> {
    MailerWorker::enqueue_async(email.clone())
        .await
        .map_err(Box::from)?;
    Ok(())
}
