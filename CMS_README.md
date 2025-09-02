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