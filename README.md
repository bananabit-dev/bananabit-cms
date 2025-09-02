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

