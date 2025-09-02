use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;

/// Email service for sending verification and notification emails
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
    base_url: String,
}

impl EmailService {
    /// Initialize email service with SMTP configuration
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "1025".to_string())
            .parse::<u16>()?;
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string());
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string());
        let from_email = env::var("FROM_EMAIL").unwrap_or_else(|_| "noreply@bananabit.dev".to_string());
        let from_name = env::var("FROM_NAME").unwrap_or_else(|_| "BananaBit CMS".to_string());
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

        // Build SMTP transport
        let mut transport = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host)
            .port(smtp_port);

        // Add authentication if credentials are provided
        if !smtp_username.is_empty() && !smtp_password.is_empty() {
            transport = transport.credentials(Credentials::new(smtp_username, smtp_password));
        }

        let mailer = transport.build();

        Ok(Self {
            mailer,
            from_email,
            from_name,
            base_url,
        })
    }

    /// Send email verification message
    pub async fn send_verification_email(
        &self,
        to_email: &str,
        to_name: &str,
        verification_token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let verification_url = format!("{}/verify-email?token={}", self.base_url, verification_token);

        let html_body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify Your Email - BananaBit CMS</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9f9f9; padding: 30px; border-radius: 0 0 8px 8px; }}
        .button {{ display: inline-block; background: #667eea; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; margin: 20px 0; }}
        .footer {{ text-align: center; margin-top: 20px; color: #666; font-size: 14px; }}
        .token {{ background: #e9ecef; padding: 10px; border-radius: 4px; font-family: monospace; word-break: break-all; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🍌 BananaBit CMS</h1>
        <p>Welcome to the future of content management!</p>
    </div>
    <div class="content">
        <h2>Hi {}!</h2>
        <p>Thank you for registering with BananaBit CMS. To complete your registration and start using your account, please verify your email address.</p>
        
        <p><strong>Click the button below to verify your email:</strong></p>
        <p><a href="{}" class="button">Verify Email Address</a></p>
        
        <p>Or copy and paste this link into your browser:</p>
        <p><a href="{}">{}</a></p>
        
        <p><strong>Or use this verification token manually:</strong></p>
        <div class="token">{}</div>
        
        <p>This verification link will expire in 24 hours for security reasons.</p>
        
        <p>If you didn't create an account with BananaBit CMS, you can safely ignore this email.</p>
        
        <p>Best regards,<br>The BananaBit CMS Team</p>
    </div>
    <div class="footer">
        <p>This is an automated message from BananaBit CMS. Please do not reply to this email.</p>
    </div>
</body>
</html>
            "#,
            to_name, verification_url, verification_url, verification_url, verification_token
        );

        let text_body = format!(
            r#"
Hi {}!

Thank you for registering with BananaBit CMS. To complete your registration and start using your account, please verify your email address.

Please visit the following link to verify your email:
{}

Or use this verification token manually: {}

This verification link will expire in 24 hours for security reasons.

If you didn't create an account with BananaBit CMS, you can safely ignore this email.

Best regards,
The BananaBit CMS Team

---
This is an automated message from BananaBit CMS. Please do not reply to this email.
            "#,
            to_name, verification_url, verification_token
        );

        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse()?)
            .to(format!("{} <{}>", to_name, to_email).parse()?)
            .subject("Verify Your Email - BananaBit CMS")
            .multipart(MultiPart::alternative()
                .singlepart(SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(text_body))
                .singlepart(SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html_body)))?;

        match self.mailer.send(email).await {
            Ok(_) => {
                log::info!("📧 Verification email sent successfully to {}", to_email);
                Ok(())
            }
            Err(e) => {
                log::error!("❌ Failed to send verification email to {}: {}", to_email, e);
                Err(Box::new(e))
            }
        }
    }

    /// Send password reset email
    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        to_name: &str,
        reset_token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let reset_url = format!("{}/reset-password?token={}", self.base_url, reset_token);

        let html_body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Reset Your Password - BananaBit CMS</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9f9f9; padding: 30px; border-radius: 0 0 8px 8px; }}
        .button {{ display: inline-block; background: #dc3545; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; margin: 20px 0; }}
        .footer {{ text-align: center; margin-top: 20px; color: #666; font-size: 14px; }}
        .warning {{ background: #fff3cd; border: 1px solid #ffeaa7; padding: 15px; border-radius: 4px; margin: 15px 0; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🍌 BananaBit CMS</h1>
        <p>Password Reset Request</p>
    </div>
    <div class="content">
        <h2>Hi {}!</h2>
        <p>We received a request to reset your password for your BananaBit CMS account.</p>
        
        <div class="warning">
            <strong>⚠️ Security Notice:</strong> If you didn't request this password reset, please ignore this email. Your account is still secure.
        </div>
        
        <p><strong>Click the button below to reset your password:</strong></p>
        <p><a href="{}" class="button">Reset Password</a></p>
        
        <p>Or copy and paste this link into your browser:</p>
        <p><a href="{}">{}</a></p>
        
        <p>This password reset link will expire in 1 hour for security reasons.</p>
        
        <p>Best regards,<br>The BananaBit CMS Team</p>
    </div>
    <div class="footer">
        <p>This is an automated message from BananaBit CMS. Please do not reply to this email.</p>
    </div>
</body>
</html>
            "#,
            to_name, reset_url, reset_url, reset_url
        );

        let text_body = format!(
            r#"
Hi {}!

We received a request to reset your password for your BananaBit CMS account.

SECURITY NOTICE: If you didn't request this password reset, please ignore this email. Your account is still secure.

Please visit the following link to reset your password:
{}

This password reset link will expire in 1 hour for security reasons.

Best regards,
The BananaBit CMS Team

---
This is an automated message from BananaBit CMS. Please do not reply to this email.
            "#,
            to_name, reset_url
        );

        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse()?)
            .to(format!("{} <{}>", to_name, to_email).parse()?)
            .subject("Reset Your Password - BananaBit CMS")
            .multipart(MultiPart::alternative()
                .singlepart(SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(text_body))
                .singlepart(SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html_body)))?;

        match self.mailer.send(email).await {
            Ok(_) => {
                log::info!("📧 Password reset email sent successfully to {}", to_email);
                Ok(())
            }
            Err(e) => {
                log::error!("❌ Failed to send password reset email to {}: {}", to_email, e);
                Err(Box::new(e))
            }
        }
    }

    /// Send welcome email after successful verification
    pub async fn send_welcome_email(
        &self,
        to_email: &str,
        to_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dashboard_url = format!("{}/admin", self.base_url);

        let html_body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Welcome to BananaBit CMS!</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9f9f9; padding: 30px; border-radius: 0 0 8px 8px; }}
        .button {{ display: inline-block; background: #28a745; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; margin: 20px 0; }}
        .footer {{ text-align: center; margin-top: 20px; color: #666; font-size: 14px; }}
        .features {{ background: white; padding: 20px; border-radius: 4px; margin: 20px 0; }}
        .feature {{ margin: 10px 0; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🎉 Welcome to BananaBit CMS!</h1>
        <p>Your account has been successfully verified</p>
    </div>
    <div class="content">
        <h2>Hi {}!</h2>
        <p>Congratulations! Your email has been verified and your BananaBit CMS account is now active.</p>
        
        <p><strong>Ready to get started?</strong></p>
        <p><a href="{}" class="button">Go to Dashboard</a></p>
        
        <div class="features">
            <h3>🌟 What you can do now:</h3>
            <div class="feature">✍️ <strong>Create Posts:</strong> Write and publish your content with our Markdown editor</div>
            <div class="feature">🎨 <strong>Customize Themes:</strong> Make your site look exactly how you want</div>
            <div class="feature">📊 <strong>View Analytics:</strong> Track your site's performance and engagement</div>
            <div class="feature">🔧 <strong>Manage Extensions:</strong> Add new functionality with our extension system</div>
            <div class="feature">💬 <strong>Moderate Comments:</strong> Engage with your audience</div>
        </div>
        
        <p>Need help getting started? Check out our documentation or join our community for support.</p>
        
        <p>Happy content creating!<br>The BananaBit CMS Team</p>
    </div>
    <div class="footer">
        <p>You're receiving this because you created an account with BananaBit CMS.</p>
    </div>
</body>
</html>
            "#,
            to_name, dashboard_url
        );

        let text_body = format!(
            r#"
🎉 Welcome to BananaBit CMS!

Hi {}!

Congratulations! Your email has been verified and your BananaBit CMS account is now active.

You can now access your dashboard at: {}

What you can do now:
✍️ Create Posts: Write and publish your content with our Markdown editor
🎨 Customize Themes: Make your site look exactly how you want  
📊 View Analytics: Track your site's performance and engagement
🔧 Manage Extensions: Add new functionality with our extension system
💬 Moderate Comments: Engage with your audience

Need help getting started? Check out our documentation or join our community for support.

Happy content creating!
The BananaBit CMS Team

---
You're receiving this because you created an account with BananaBit CMS.
            "#,
            to_name, dashboard_url
        );

        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse()?)
            .to(format!("{} <{}>", to_name, to_email).parse()?)
            .subject("🎉 Welcome to BananaBit CMS - Account Verified!")
            .multipart(MultiPart::alternative()
                .singlepart(SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(text_body))
                .singlepart(SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html_body)))?;

        match self.mailer.send(email).await {
            Ok(_) => {
                log::info!("📧 Welcome email sent successfully to {}", to_email);
                Ok(())
            }
            Err(e) => {
                log::error!("❌ Failed to send welcome email to {}: {}", to_email, e);
                Err(Box::new(e))
            }
        }
    }
}