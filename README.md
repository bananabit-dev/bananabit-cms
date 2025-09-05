# BananaBit CMS

A modern, extension-based content management system built with Rust and Dioxus.

## 🚀 Quick Start

The easiest way to get started is with our interactive setup script:

```bash
./start.sh
```

This will guide you through either:
- **Development setup** with MailHog for email testing
- **Production setup** with real email configuration

## 📧 Email Features

BananaBit CMS includes a complete email system:
- ✅ User verification emails
- ✅ Welcome emails after verification  
- ✅ Professional HTML email templates
- ✅ Support for all major SMTP providers (Gmail, SendGrid, Mailgun, AWS SES)
- ✅ Development email testing with MailHog
- ✅ Production-ready with proper DNS configuration

## 🔧 Manual Setup

### Development (with email testing)
```bash
docker-compose up -d
```
- App: http://localhost:8080
- MailHog: http://localhost:8025

### Production
```bash
cp .env.example .env
# Edit .env with your settings
docker-compose -f docker-compose.prod.yml up -d
```

## 🚀 Deploy to Fly.io

BananaBit CMS is optimized for deployment on [Fly.io](https://fly.io) with full email functionality:

### Prerequisites
1. Install the [Fly CLI](https://fly.io/docs/hands-on/install-flyctl/)
2. Sign up for a [Fly.io account](https://fly.io/app/sign-up)
3. Configure an email service provider (see options below)

### Quick Deploy
```bash
# Login to Fly.io
fly auth login

# Create and deploy your app
fly launch

# Create a volume for database persistence
fly volumes create cms_data --region fra --size 1

# Set your domain (replace with your domain)
fly secrets set BASE_URL="https://your-app.fly.dev"

# Configure email settings (see provider examples below)
fly secrets set SMTP_HOST="smtp.gmail.com"
fly secrets set SMTP_PORT="587"
fly secrets set SMTP_USERNAME="your-email@gmail.com"
fly secrets set SMTP_PASSWORD="your-app-password"
fly secrets set FROM_EMAIL="noreply@your-domain.com"
fly secrets set FROM_NAME="Your Site Name"

# Deploy your app
fly deploy

# Verify deployment (optional)
./verify-css-deployment.sh
```

### Verify Deployment

After deployment, you can verify that CSS is loading correctly:

```bash
# Test CSS loading on your deployed site
./verify-css-deployment.sh https://your-app.fly.dev

# Or use the default URL
./verify-css-deployment.sh
```

This script will:
- Test accessibility of all CSS files
- Verify correct content types
- Check that the main application loads
- Provide troubleshooting guidance if issues are found

### Email Provider Configuration

Choose one of these popular email providers:

#### Gmail (Free tier available)
```bash
fly secrets set SMTP_HOST="smtp.gmail.com"
fly secrets set SMTP_PORT="587"
fly secrets set SMTP_USERNAME="your-email@gmail.com"
fly secrets set SMTP_PASSWORD="your-app-password"  # Use App Password, not regular password
fly secrets set FROM_EMAIL="your-email@gmail.com"
fly secrets set FROM_NAME="Your Site Name"
```

**Setup:** Enable 2FA in Gmail, then create an [App Password](https://support.google.com/accounts/answer/185833).

#### SendGrid (99 emails/day free)
```bash
fly secrets set SMTP_HOST="smtp.sendgrid.net"
fly secrets set SMTP_PORT="587"
fly secrets set SMTP_USERNAME="apikey"
fly secrets set SMTP_PASSWORD="your-sendgrid-api-key"
fly secrets set FROM_EMAIL="noreply@your-domain.com"
fly secrets set FROM_NAME="Your Site Name"
```

**Setup:** Sign up at [SendGrid](https://sendgrid.com), create an API key with mail send permissions.

#### Mailgun (100 emails/day free)
```bash
fly secrets set SMTP_HOST="smtp.mailgun.org"
fly secrets set SMTP_PORT="587"
fly secrets set SMTP_USERNAME="postmaster@mg.your-domain.com"
fly secrets set SMTP_PASSWORD="your-mailgun-smtp-password"
fly secrets set FROM_EMAIL="noreply@your-domain.com"
fly secrets set FROM_NAME="Your Site Name"
```

**Setup:** Sign up at [Mailgun](https://mailgun.com), verify your domain, get SMTP credentials.

#### AWS SES (62,000 emails/month free)
```bash
fly secrets set SMTP_HOST="email-smtp.us-east-1.amazonaws.com"
fly secrets set SMTP_PORT="587"
fly secrets set SMTP_USERNAME="your-ses-access-key"
fly secrets set SMTP_PASSWORD="your-ses-secret-key"
fly secrets set FROM_EMAIL="noreply@your-domain.com"
fly secrets set FROM_NAME="Your Site Name"
```

**Setup:** Set up [AWS SES](https://aws.amazon.com/ses/), verify your domain, create SMTP credentials.

### Custom Domain Setup

1. **Add your domain to Fly.io:**
   ```bash
   fly ips list
   fly certs create your-domain.com
   ```

2. **Update DNS records:** Point your domain's A record to the Fly.io IP addresses shown in `fly ips list`.

3. **Update your BASE_URL:**
   ```bash
   fly secrets set BASE_URL="https://your-domain.com"
   ```

### Email DNS Configuration (Recommended)

For better email deliverability, configure these DNS records:

#### SPF Record
```
TXT record: v=spf1 include:_spf.google.com ~all
```
(Replace with your email provider's SPF record)

#### DMARC Record
```
TXT record for _dmarc.your-domain.com: v=DMARC1; p=quarantine; rua=mailto:admin@your-domain.com
```

### Troubleshooting Fly.io Email Issues

#### Check application logs:
```bash
fly logs --app your-app-name
```

#### Verify secrets are set:
```bash
fly secrets list
```

#### Test email functionality:
```bash
# SSH into your running app
fly ssh console

# Check environment variables
env | grep SMTP
```

#### Common Issues:

1. **"Failed to send verification email"**
   - Verify SMTP credentials with `fly secrets list`
   - Check logs with `fly logs`
   - Ensure your email provider allows SMTP access

2. **"Connection timeout"**
   - Fly.io doesn't block SMTP ports (unlike some hosts)
   - Verify SMTP host/port combination
   - Try port 465 if 587 doesn't work

3. **"Authentication failed"**
   - Double-check credentials
   - For Gmail: Use App Password, not regular password
   - For SendGrid: Use "apikey" as username

4. **Emails going to spam**
   - Set up SPF/DKIM/DMARC records
   - Use a FROM_EMAIL that matches your domain
   - Consider using a dedicated IP (available on paid plans)

### Scaling Considerations

- **Volume**: The database volume is persistent across deploys
- **Email Rate Limits**: Monitor your email provider's rate limits
- **Memory**: Default 1GB is sufficient for small to medium sites
- **Regions**: Deploy closer to your users for better performance

### Monitoring

Monitor your application:
```bash
# View real-time logs
fly logs --app your-app-name -f

# Check app status
fly status

# Monitor email-specific logs
fly logs --app your-app-name | grep "📧\|email\|smtp"
```

## 📖 Documentation

- **[CMS_README.md](CMS_README.md)** - Complete setup and email configuration guide
- **[DOCKER_README.md](DOCKER_README.md)** - Docker-specific instructions  
- **[EMAIL_TROUBLESHOOTING.md](EMAIL_TROUBLESHOOTING.md)** - Email troubleshooting guide

## 🛠️ Development

### Manual Development Setup

Navigate to the platform crate of your choice:
```bash
cd web
```

and serve:
```bash
dx serve
```

## 🎯 Features

- **Extension-Based Architecture**: Everything is a plugin
- **Email System**: Complete email verification and notifications
- **Modern UI**: Built with Dioxus for reactive interfaces
- **Database Persistence**: SQLite with automatic migrations
- **Docker Ready**: Complete containerization with email services
- **Multi-Platform**: Web, desktop, and mobile support

