# BananaBit CMS - Extension-Based Content Management System

BananaBit CMS is a modern, extension-based content management system built with Rust and Dioxus.

## 🌟 Features

- **Extension-Based Architecture**: Everything is a plugin, making the system highly modular
- **Performance**: Built with Rust for maximum performance and memory safety
- **Modern UI**: Uses Dioxus for a reactive, component-based frontend
- **Markdown Support**: Rich content editing with syntax highlighting
- **Authentication**: Secure user authentication and authorization
- **Comments System**: Built-in commenting for posts
- **SEO-Friendly URLs**: Professional routing with slug support
- **Professional Design**: Modern, responsive UI

## 🏗️ Architecture

The CMS is built around a powerful extension system:

### Core Extensions

1. **Posts Extension** (`core.posts`)
   - Handles blog posts and articles
   - Supports both numeric IDs and SEO-friendly slugs
   - Markdown content with syntax highlighting

2. **Pages Extension** (`core.pages`)
   - Static page management
   - Custom templates support

3. **Comments Extension** (`core.comments`)
   - Threaded comments
   - Moderation system
   - User-friendly forms

4. **Auth Extension** (`core.auth`)
   - User authentication
   - Role-based access control
   - Admin dashboard

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Modern web browser

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/bananabit-dev/bananabit-cms
   cd bananabit-cms
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the web application:
   ```bash
   cd web
   cargo run --features web
   ```

Note: Since this is a WASM-based frontend, you'll need to serve it properly. In a production environment, you would use a tool like `dioxus-cli`:

```bash
cargo install dioxus-cli
cd web
dx serve
```

## 📍 Routes

The CMS supports the following routes:

- `/` - Home page with feature overview
- `/blog/0` - First blog post (legacy route)
- `/post/:slug` - SEO-friendly post URLs
- `/page/:slug` - Static pages (about, contact, etc.)
- `/login` - User authentication
- `/admin` - Admin dashboard (requires authentication)

## 🎨 Customization

### Creating Extensions

Extensions implement the `Extension` trait:

```rust
impl Extension for MyExtension {
    fn id(&self) -> &'static str { "my.extension" }
    fn name(&self) -> &'static str { "My Extension" }
    fn version(&self) -> &'static str { "1.0.0" }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize your extension
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        // Define custom routes
        vec![]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        // Define reusable components
        vec![]
    }
}
```

### User Registration

The first user to register will automatically be granted admin privileges. All subsequent users will be registered as subscribers.

For the first user registration, a captcha question must be answered: "Who's bananabit?" (Answer: "a cool dude")

All users must verify their email address before they can log in.

## 📧 Email Configuration

BananaBit CMS includes a secure email system for user verification and notifications. The system supports multiple SMTP providers and includes development tools for testing.

### Quick Start with Docker Compose

For development with MailHog (email testing):
```bash
docker-compose up -d
```

For production:
```bash
# Copy and configure environment file
cp .env.example .env
# Edit .env with your SMTP settings
docker-compose -f docker-compose.prod.yml up -d
```

### Email Service Providers

The CMS supports any SMTP-compatible email service. Here are configurations for popular providers:

#### Gmail
```env
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
FROM_EMAIL=your-email@gmail.com
FROM_NAME=Your Site Name
```

**Note:** For Gmail, you need to use an App Password, not your regular password. Enable 2FA and generate an App Password in your Google Account settings.

#### SendGrid
```env
SMTP_HOST=smtp.sendgrid.net
SMTP_PORT=587
SMTP_USERNAME=apikey
SMTP_PASSWORD=your-sendgrid-api-key
FROM_EMAIL=noreply@your-domain.com
FROM_NAME=Your Site Name
```

#### Mailgun
```env
SMTP_HOST=smtp.mailgun.org
SMTP_PORT=587
SMTP_USERNAME=your-mailgun-username
SMTP_PASSWORD=your-mailgun-password
FROM_EMAIL=noreply@your-domain.com
FROM_NAME=Your Site Name
```

#### AWS SES
```env
SMTP_HOST=email-smtp.us-east-1.amazonaws.com
SMTP_PORT=587
SMTP_USERNAME=your-ses-username
SMTP_PASSWORD=your-ses-password
FROM_EMAIL=noreply@your-domain.com
FROM_NAME=Your Site Name
```

### DNS Records Setup

To ensure reliable email delivery and avoid spam filters, configure these DNS records for your domain:

#### SPF Record
Add a TXT record for your domain:
```
Name: @
Type: TXT
Value: v=spf1 include:_spf.google.com ~all
```
(Replace `_spf.google.com` with your email provider's SPF record)

#### DKIM Record
Your email provider will give you a DKIM record. Add it as a TXT record:
```
Name: selector._domainkey (provided by your email service)
Type: TXT
Value: (provided by your email service)
```

#### DMARC Record
Add a DMARC policy:
```
Name: _dmarc
Type: TXT
Value: v=DMARC1; p=quarantine; rua=mailto:admin@your-domain.com
```

### Development Setup

For local development, the docker-compose.yml includes MailHog for email testing:

1. Start the services:
   ```bash
   docker-compose up -d
   ```

2. Access MailHog Web UI at: http://localhost:8025
3. Your app will send emails to MailHog instead of real addresses
4. View all sent emails in the MailHog interface

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `SMTP_HOST` | SMTP server hostname | Yes | localhost |
| `SMTP_PORT` | SMTP server port | No | 1025 |
| `SMTP_USERNAME` | SMTP username | No | "" |
| `SMTP_PASSWORD` | SMTP password | No | "" |
| `FROM_EMAIL` | Sender email address | Yes | noreply@bananabit.dev |
| `FROM_NAME` | Sender display name | No | BananaBit CMS |
| `BASE_URL` | Your site's base URL | Yes | http://localhost:8080 |

### Email Features

The email system includes:

- **Verification Emails**: Sent during user registration
- **Welcome Emails**: Sent after email verification
- **Password Reset**: (Ready for implementation)
- **HTML & Text**: Multi-part emails with both HTML and plain text
- **Professional Templates**: Beautiful, responsive email templates
- **Security**: Verification tokens with expiration
- **Logging**: Comprehensive email sending logs

### Troubleshooting

#### Emails not sending
1. Check SMTP credentials in your environment variables
2. Verify SMTP host and port settings
3. Check application logs for error messages
4. Test SMTP connection with your provider's tools

#### Emails going to spam
1. Configure SPF, DKIM, and DMARC DNS records
2. Use a reputable email service provider
3. Ensure FROM_EMAIL matches your domain
4. Avoid spam trigger words in email content

#### Development email testing
1. Use MailHog with docker-compose for local testing
2. Access MailHog at http://localhost:8025
3. All emails will be captured instead of sent to real addresses

## 🎨 Styling

The CMS features a modern, dark theme with:

- Professional typography
- Responsive design
- Smooth animations
- Accessible color schemes
- Custom component styling

## 🧩 Extension System

The extension system allows for:

- **Modular Architecture**: Add/remove features as needed
- **Route Management**: Extensions can register new routes
- **Component System**: Reusable UI components
- **Event Hooks**: React to system events
- **Data Management**: Each extension manages its own data

## 📁 Project Structure

```
bananabit-cms/
├── ui/                     # Shared UI components and extensions
│   ├── src/
│   │   ├── extensions/     # Core extensions
│   │   │   ├── posts.rs    # Posts management
│   │   │   ├── comments.rs # Comments system
│   │   │   ├── auth.rs     # Authentication
│   │   │   └── pages.rs    # Static pages
│   │   ├── views/          # Page components
│   │   └── navbar.rs       # Navigation and routing
├── web/                    # Web frontend
├── api/                    # API definitions
├── ba-server/              # Backend server
├── desktop/                # Desktop app (future)
└── mobile/                 # Mobile app (future)
```

## 🔮 Future Plans

- [ ] Database integration for persistence
- [ ] Media management system
- [ ] Theme system
- [ ] Plugin marketplace
- [ ] Advanced SEO features
- [ ] Performance analytics
- [ ] Content scheduling
- [ ] Multi-language support

## 🤝 Contributing

We welcome contributions! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.

---

Built with ❤️ using Rust and Dioxus