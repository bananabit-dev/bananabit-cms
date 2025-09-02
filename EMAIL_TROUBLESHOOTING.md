# Email Troubleshooting Guide

This guide helps you troubleshoot common email issues with BananaBit CMS.

## 🔧 Quick Diagnostics

### Check if email service is configured
```bash
# Check environment variables
docker-compose exec app env | grep SMTP
docker-compose exec app env | grep FROM_EMAIL
```

### View application logs
```bash
# View real-time logs
docker-compose logs -f app

# View email-specific logs
docker-compose logs app | grep "📧\|email\|smtp"
```

### Test SMTP connection
```bash
# Test SMTP connectivity (replace with your SMTP host)
telnet smtp.gmail.com 587
```

## ❌ Common Issues

### 1. "Failed to send verification email"

**Symptoms:** User registration appears successful but no email is sent.

**Possible causes:**
- SMTP credentials are incorrect
- SMTP host/port is wrong
- Firewall blocking SMTP ports
- Email provider requires app-specific passwords

**Solutions:**
1. Verify SMTP settings in .env file
2. Check logs: `docker-compose logs app | grep email`
3. For Gmail: Use App Password instead of regular password
4. Ensure port 587 or 465 is open

### 2. "SSL certificate problem"

**Symptoms:** SSL/TLS connection errors in logs.

**Solutions:**
1. Use port 587 with STARTTLS (most common)
2. Try port 465 for SSL
3. Check if your email provider supports the configuration

### 3. "Authentication failed"

**Symptoms:** SMTP authentication errors.

**Solutions:**
1. Double-check SMTP_USERNAME and SMTP_PASSWORD
2. For Gmail: Enable 2FA and create an App Password
3. For SendGrid: Use "apikey" as username, API key as password
4. Check if account is active and has SMTP access

### 4. Emails going to spam

**Symptoms:** Emails are sent but end up in spam folders.

**Solutions:**
1. Set up SPF record: `v=spf1 include:_spf.google.com ~all`
2. Configure DKIM (get record from your email provider)
3. Add DMARC record: `v=DMARC1; p=quarantine; rua=mailto:admin@yourdomain.com`
4. Use a FROM_EMAIL that matches your domain
5. Use a reputable email service provider

### 5. "Connection timeout"

**Symptoms:** Email sending times out.

**Solutions:**
1. Check if firewall blocks SMTP ports (587, 465, 25)
2. Try different SMTP port (587 vs 465)
3. Check if your hosting provider blocks SMTP

### 6. Development: MailHog not receiving emails

**Symptoms:** Development setup doesn't show emails in MailHog.

**Solutions:**
1. Ensure MailHog is running: `docker-compose ps`
2. Check if app connects to MailHog: `docker-compose logs app`
3. Verify SMTP_HOST=mailhog in development
4. Access MailHog at http://localhost:8025

## 🛠️ Debug Commands

### View all environment variables
```bash
docker-compose exec app env
```

### Test email configuration
```bash
# Connect to app container
docker-compose exec app /bin/bash

# Check SMTP connectivity (if telnet is available)
telnet $SMTP_HOST $SMTP_PORT
```

### Check network connectivity
```bash
# Test if services can communicate
docker-compose exec app ping mailhog
```

### Reset email configuration
```bash
# Stop services
docker-compose down

# Edit environment
nano .env

# Restart services
docker-compose up -d
```

## 📧 Email Provider Specific Issues

### Gmail
- **Issue:** "Less secure app access"
- **Solution:** Use App Password with 2FA enabled

### SendGrid
- **Issue:** API key not working
- **Solution:** Use "apikey" as username, not your email

### Mailgun
- **Issue:** Domain verification required
- **Solution:** Verify your domain in Mailgun dashboard first

### AWS SES
- **Issue:** Account in sandbox mode
- **Solution:** Request production access from AWS

## 🆘 Getting Help

If you're still having issues:

1. Check the logs: `docker-compose logs app`
2. Verify your email provider's documentation
3. Test SMTP settings with a tool like `swaks` or online SMTP testers
4. Check if your hosting provider has SMTP restrictions

## 📝 Useful Log Examples

### Successful email sending:
```
📧 Verification email sent successfully to user@example.com
```

### Failed email sending:
```
❌ Failed to send verification email to user@example.com: SMTP error
```

### Connection issues:
```
Failed to initialize email service: Connection refused
```