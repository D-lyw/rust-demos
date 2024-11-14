use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

static FROM_MAIL: &str = "liuyuanwang4321@gmail.com";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mailer = build_mailer(std::env::var("EMAIL_APP_PASSWORD")?)?;

    let email = build_message(
        "1826789310@qq.com",
        "Test from mailer client by Rust",
        "hello from Rust",
    )?;

    if let Ok(_) = mailer.send(&email) {
        println!("send email successfully!")
    }

    Ok(())
}

/// build email message that will be sent
fn build_message(to: &str, subject: &str, body: &str) -> anyhow::Result<Message> {
    let message = Message::builder()
        .from(FROM_MAIL.parse::<Mailbox>()?)
        .to(to.parse::<Mailbox>()?)
        .subject(subject)
        .body(body.to_string())?;

    Ok(message)
}

/// build mailer client
fn build_mailer(app_password: String) -> anyhow::Result<SmtpTransport> {
    let smtp_server = "smtp.gmail.com";

    let creds = Credentials::new(FROM_MAIL.to_string(), app_password);

    let mailer = SmtpTransport::relay(&smtp_server)?
        .credentials(creds)
        .build();

    Ok(mailer)
}
