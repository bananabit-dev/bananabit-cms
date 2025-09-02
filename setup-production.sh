#!/bin/bash

# Production setup script for BananaBit CMS
echo "🍌 BananaBit CMS Production Setup"
echo "================================="

# Check if .env already exists
if [ -f .env ]; then
    echo "⚠️  .env file already exists. Backup will be created as .env.backup"
    cp .env .env.backup
fi

# Copy template
cp .env.example .env

echo "✅ Created .env file from template"
echo ""
echo "📝 Please edit the .env file and configure the following:"
echo ""
echo "Required settings:"
echo "  - BASE_URL: Your domain (https://your-domain.com)"
echo "  - SMTP_HOST: Your email provider's SMTP server"
echo "  - SMTP_PORT: Usually 587 for TLS or 465 for SSL"
echo "  - SMTP_USERNAME: Your SMTP username"
echo "  - SMTP_PASSWORD: Your SMTP password"
echo "  - FROM_EMAIL: Email address for sending notifications"
echo "  - FROM_NAME: Display name for emails"
echo ""
echo "🔧 Common email provider settings:"
echo ""
echo "Gmail:"
echo "  SMTP_HOST=smtp.gmail.com"
echo "  SMTP_PORT=587"
echo "  Use App Password, not regular password"
echo ""
echo "SendGrid:"
echo "  SMTP_HOST=smtp.sendgrid.net"
echo "  SMTP_PORT=587"
echo "  SMTP_USERNAME=apikey"
echo ""
echo "Mailgun:"
echo "  SMTP_HOST=smtp.mailgun.org"
echo "  SMTP_PORT=587"
echo ""
echo "AWS SES:"
echo "  SMTP_HOST=email-smtp.us-east-1.amazonaws.com"
echo "  SMTP_PORT=587"
echo ""
echo "📋 DNS Records to set up for email delivery:"
echo ""
echo "1. SPF Record (TXT record for @):"
echo "   v=spf1 include:_spf.[your-provider].com ~all"
echo ""
echo "2. DKIM Record (get from your email provider)"
echo ""
echo "3. DMARC Record (TXT record for _dmarc):"
echo "   v=DMARC1; p=quarantine; rua=mailto:admin@your-domain.com"
echo ""
echo "🚀 After configuring .env, start with:"
echo "   docker-compose -f docker-compose.prod.yml up -d"
echo ""
echo "📖 For detailed setup instructions, see CMS_README.md"