# BananaBit CMS Docker Setup

This directory contains Docker configuration files for running BananaBit CMS with email support.

## Quick Start

### Development (with MailHog email testing)

1. Start the services:
   ```bash
   docker-compose up -d
   ```

2. Access the application at: http://localhost:8080
3. Access MailHog (email testing) at: http://localhost:8025
4. Register the first user (will become admin)
5. Check MailHog for verification emails

### Production

1. Copy environment template:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your configuration:
   - Set your domain in `BASE_URL`
   - Configure SMTP settings for your email provider
   - Set proper `FROM_EMAIL` and `FROM_NAME`

3. Start production services:
   ```bash
   docker-compose -f docker-compose.prod.yml up -d
   ```

## Email Providers

See CMS_README.md for detailed configuration examples for:
- Gmail
- SendGrid
- Mailgun
- AWS SES
- Other SMTP providers

## DNS Configuration

For production email delivery, configure these DNS records:
- SPF record for your domain
- DKIM record from your email provider
- DMARC policy record

See CMS_README.md for detailed DNS setup instructions.

## Volumes

- `./data` - Database and application data
- `./uploads` - User uploaded files (production)

## Ports

- `8080` - Main application
- `8025` - MailHog web interface (development only)
- `1025` - MailHog SMTP server (development only)

## Troubleshooting

1. **Emails not sending**: Check SMTP configuration in `.env`
2. **Database issues**: Ensure `./data` directory has proper permissions
3. **Port conflicts**: Change ports in docker-compose.yml if needed

For more details, see the main CMS_README.md file.