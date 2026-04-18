use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use crate::config::Config;
use crate::utils::errors::{AppError, AppResult};

pub struct EmailClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

impl EmailClient {
    pub fn new(config: &Config) -> AppResult<Self> {
        let creds = Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );

        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("SMTP config error: {}", e)))?
            .port(config.smtp_port)
            .credentials(creds)
            .build();

        let from: Mailbox = format!("{} <{}>", config.email_from_name, config.email_from)
            .parse()
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid from address: {}", e)))?;

        Ok(Self { transport, from })
    }

    pub async fn send_html(&self, to: &str, subject: &str, html_body: &str) -> AppResult<()> {
        let to_mailbox: Mailbox = to.parse()
            .map_err(|_| AppError::BadRequest(format!("Invalid email: {}", to)))?;

        let email = Message::builder()
            .from(self.from.clone())
            .to(to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Email build error: {}", e)))?;

        self.transport
            .send(email)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Email send error: {}", e)))?;

        Ok(())
    }

    pub async fn send_verification_email(&self, to: &str, username: &str, token: &str, platform_url: &str) -> AppResult<()> {
        let link = format!("{}/verify-email?token={}", platform_url, token);
        let html = format!(
            r#"<h2>Welcome, {}!</h2>
               <p>Please verify your email address by clicking the link below:</p>
               <p><a href="{}">Verify Email</a></p>
               <p>This link expires in 24 hours.</p>
               <p>If you did not create an account, ignore this email.</p>"#,
            username, link
        );
        self.send_html(to, "Verify your email address", &html).await
    }

    pub async fn send_password_reset(&self, to: &str, token: &str, platform_url: &str) -> AppResult<()> {
        let link = format!("{}/reset-password?token={}", platform_url, token);
        let html = format!(
            r#"<h2>Password Reset Request</h2>
               <p>Click the link below to reset your password:</p>
               <p><a href="{}">Reset Password</a></p>
               <p>This link expires in 2 hours.</p>
               <p>If you did not request this, ignore this email.</p>"#,
            link
        );
        self.send_html(to, "Reset your password", &html).await
    }
}
