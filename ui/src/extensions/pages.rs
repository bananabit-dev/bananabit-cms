use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Page data structure for static pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: u32,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub published: bool,
    pub template: String, // Template to use for rendering
}

/// Pages extension - handles static pages
pub struct PagesExtension {
    pages: HashMap<u32, Page>,
    slug_to_id: HashMap<String, u32>,
}

impl PagesExtension {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            slug_to_id: HashMap::new(),
        }
    }
    
    pub fn add_page(&mut self, page: Page) {
        self.slug_to_id.insert(page.slug.clone(), page.id);
        self.pages.insert(page.id, page);
    }
    
    pub fn get_page_by_slug(&self, slug: &str) -> Option<&Page> {
        let id = self.slug_to_id.get(slug)?;
        self.pages.get(id)
    }
    
    pub fn list_published_pages(&self) -> Vec<&Page> {
        self.pages
            .values()
            .filter(|page| page.published)
            .collect()
    }
}

impl Extension for PagesExtension {
    fn id(&self) -> &'static str {
        "core.pages"
    }
    
    fn name(&self) -> &'static str {
        "Static Pages"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create an About page
        let about_page = Page {
            id: 1,
            slug: "about".to_string(),
            title: "About BananaBit CMS".to_string(),
            content: r#"# About BananaBit CMS

BananaBit CMS is a modern, extension-based content management system built with Rust and Dioxus.

## Mission

Our mission is to provide a fast, secure, and highly customizable CMS that developers love to work with.

## Features

- **Extension-based Architecture**: Everything is a plugin, making the system highly modular
- **Performance**: Built with Rust for maximum performance and safety
- **Modern UI**: Uses Dioxus for a reactive, component-based frontend
- **Security**: Type-safe code and secure defaults
- **Flexibility**: Easy to extend and customize

## Technology Stack

- **Backend**: Rust with modern web frameworks
- **Frontend**: Dioxus (React-like for Rust)
- **Database**: Pluggable storage backends
- **Authentication**: Built-in secure auth system

## Getting Started

Visit our documentation to learn how to set up and customize your BananaBit CMS installation.

---

*Built with ❤️ by the BananaBit team*"#.to_string(),
            author: "Admin".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            published: true,
            template: "default".to_string(),
        };
        
        self.add_page(about_page);
        
        // Create a Contact page
        let contact_page = Page {
            id: 2,
            slug: "contact".to_string(),
            title: "Contact Us".to_string(),
            content: r#"# Contact Us

Get in touch with the BananaBit CMS team.

## Ways to Reach Us

### Email
- General inquiries: hello@bananabit.cms
- Support: support@bananabit.cms
- Security issues: security@bananabit.cms

### Social Media
- Twitter: [@BananaBitCMS](https://twitter.com/bananabitcms)
- GitHub: [bananabit-dev](https://github.com/bananabit-dev)

### Community
- Discord: Join our community server
- Forums: Community discussion boards

## Business Hours

We're available Monday through Friday, 9 AM to 5 PM UTC.

---

We'd love to hear from you!"#.to_string(),
            author: "Admin".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            published: true,
            template: "default".to_string(),
        };
        
        self.add_page(contact_page);
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            ExtensionRoute {
                path: "/page/:slug".to_string(),
                requires_auth: false,
                admin_only: false,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "PageView".to_string(),
                description: "Static page view component".to_string(),
            },
            ExtensionComponent {
                name: "PageList".to_string(),
                description: "List of pages component".to_string(),
            },
        ]
    }
}

#[component]
pub fn PageView(slug: String) -> Element {
    // In a real implementation, this would fetch from the pages extension
    let content = use_signal(|| match slug.as_str() {
        "about" => r#"# About BananaBit CMS

BananaBit CMS is a modern, extension-based content management system built with Rust and Dioxus.

## Mission

Our mission is to provide a fast, secure, and highly customizable CMS that developers love to work with.

## Features

- **Extension-based Architecture**: Everything is a plugin, making the system highly modular
- **Performance**: Built with Rust for maximum performance and safety
- **Modern UI**: Uses Dioxus for a reactive, component-based frontend
- **Security**: Type-safe code and secure defaults
- **Flexibility**: Easy to extend and customize

## Technology Stack

- **Backend**: Rust with modern web frameworks
- **Frontend**: Dioxus (React-like for Rust)
- **Database**: Pluggable storage backends
- **Authentication**: Built-in secure auth system

## Getting Started

Visit our documentation to learn how to set up and customize your BananaBit CMS installation.

---

*Built with ❤️ by the BananaBit team*"#.to_string(),
        "contact" => r#"# Contact Us

Get in touch with the BananaBit CMS team.

## Ways to Reach Us

### Email
- General inquiries: hello@bananabit.cms
- Support: support@bananabit.cms
- Security issues: security@bananabit.cms

### Social Media
- Twitter: [@BananaBitCMS](https://twitter.com/bananabitcms)
- GitHub: [bananabit-dev](https://github.com/bananabit-dev)

### Community
- Discord: Join our community server
- Forums: Community discussion boards

## Business Hours

We're available Monday through Friday, 9 AM to 5 PM UTC.

---

We'd love to hear from you!"#.to_string(),
        _ => format!("# Page Not Found\n\nThe page '{}' could not be found.", slug),
    });

    let image_base_path = "/assets/images";

    rsx! {
        document::Link { rel: "stylesheet", href: "/assets/blog.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/markdown.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/syntax.css"}

        div {
            id: "page",
            class: "markdown-container page-content",

            crate::Markdown {
                content: Some(content()),
                image_base_path: Some(image_base_path.to_string()),
                id: Some(format!("page-content-{}", slug))
            }

            div {
                class: "page-navigation",
                Link {
                    to: crate::navbar::Route::Home {},
                    "← Back to Home"
                }
            }
        }
    }
}

#[component]
pub fn PageList() -> Element {
    rsx! {
        div {
            class: "page-list",
            h2 { "Pages" }
            
            ul {
                li { 
                    a { href: "/page/about", "About" }
                    p { "Learn more about BananaBit CMS" }
                }
                li { 
                    a { href: "/page/contact", "Contact" }
                    p { "Get in touch with us" }
                }
            }
        }
    }
}