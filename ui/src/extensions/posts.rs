use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent, Post};
use crate::navbar::Route;
use crate::Markdown;
use std::collections::HashMap;

/// Posts extension - handles blog posts and pages
pub struct PostsExtension {
    posts: HashMap<u32, Post>,
    slug_to_id: HashMap<String, u32>,
}

impl PostsExtension {
    pub fn new() -> Self {
        Self {
            posts: HashMap::new(),
            slug_to_id: HashMap::new(),
        }
    }
    
    pub fn add_post(&mut self, post: Post) {
        self.slug_to_id.insert(post.slug.clone(), post.id);
        self.posts.insert(post.id, post);
    }
    
    pub fn get_post_by_id(&self, id: u32) -> Option<&Post> {
        self.posts.get(&id)
    }
    
    pub fn get_post_by_slug(&self, slug: &str) -> Option<&Post> {
        let id = self.slug_to_id.get(slug)?;
        self.posts.get(id)
    }
    
    pub fn list_published_posts(&self) -> Vec<&Post> {
        let mut posts: Vec<&Post> = self.posts
            .values()
            .filter(|post| post.published)
            .collect();
        posts.sort_by(|a, b| b.id.cmp(&a.id)); // Latest first
        posts
    }
}

impl Extension for PostsExtension {
    fn id(&self) -> &'static str {
        "core.posts"
    }
    
    fn name(&self) -> &'static str {
        "Posts & Pages"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add the first blog post (keeping blog/0 route)
        let first_post = Post {
            id: 0,
            slug: "welcome-to-bananabit-cms".to_string(),
            title: "Welcome to BananaBit CMS".to_string(),
            content: include_str!("../../../web/assets/blog/0.md").to_string(),
            author: "Admin".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            published: true,
        };
        
        self.add_post(first_post);
        
        // Add a second example post
        let second_post = Post {
            id: 1,
            slug: "extension-architecture".to_string(),
            title: "Understanding the Extension Architecture".to_string(),
            content: r#"# Extension Architecture

Our CMS is built around a powerful extension system that makes it highly modular and extensible.

## Core Concepts

- **Extensions** - Self-contained modules that provide functionality
- **Routes** - URL endpoints handled by extensions
- **Components** - Reusable UI elements
- **Hooks** - Event handlers for system events

## Built-in Extensions

1. **Posts Extension** - Handles blog posts and pages
2. **Comments Extension** - Manages user comments
3. **Auth Extension** - User authentication and authorization
4. **Admin Extension** - Administrative interface

This modular approach allows developers to easily add new features without modifying the core system."#.to_string(),
            author: "Admin".to_string(),
            created_at: "2024-01-02T00:00:00Z".to_string(),
            updated_at: "2024-01-02T00:00:00Z".to_string(),
            published: true,
        };
        
        self.add_post(second_post);
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            // Keep the original blog/0 route for backwards compatibility
            ExtensionRoute {
                path: "/post/:slug".to_string(),
                requires_auth: false,
                admin_only: false,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "PostView".to_string(),
                description: "Individual post view component".to_string(),
            },
            ExtensionComponent {
                name: "PostList".to_string(),
                description: "List of posts component".to_string(),
            },
        ]
    }
}

#[component]
pub fn PostView(slug: String) -> Element {
    // For now, we'll use a simple static approach
    // In a real implementation, this would fetch from the extension manager
    let slug_for_format = slug.clone();
    let content = use_resource(move || {
        let slug_copy = slug.clone();
        async move {
            // This is a simplified version - in reality we'd get this from the extension manager
            if slug_copy == "welcome-to-bananabit-cms" || slug_copy == "0" {
                gloo_net::http::Request::get("/assets/blog/0.md")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap_or_else(|_| "Error loading post".to_string())
            } else {
                format!("# Post: {}\n\nThis is a dynamically generated post for slug: {}", slug_copy, slug_copy)
            }
        }
    });

    let image_base_path = "/assets/images";

    rsx! {
        document::Link { rel: "stylesheet", href: "/assets/blog.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/markdown.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/syntax.css"}

        div {
            id: "post",
            class: "markdown-container",

            match content.read().as_ref() {
                Some(markdown) => rsx! {
                    Markdown {
                        content: Some(markdown.clone()),
                        image_base_path: Some(image_base_path.to_string()),
                        id: Some(format!("post-content-{}", slug_for_format))
                    }
                },
                None => rsx! { p { "Loading Post..." } }
            }

            div {
                class: "post-navigation",
                Link {
                    to: Route::Home {},
                    "← Back to Home"
                }
            }
        }
    }
}

#[component]
pub fn PostList() -> Element {
    rsx! {
        div {
            class: "post-list",
            h2 { "Recent Posts" }
            
            div {
                class: "post-item",
                h3 { 
                    Link {
                        to: Route::Blog { id: 0 },
                        "Welcome to BananaBit CMS"
                    }
                }
                p { "The first post in our new extension-based CMS" }
                span { class: "post-meta", "Published on 2024-01-01" }
            }
            
            div {
                class: "post-item",
                h3 { 
                    a { href: "/post/extension-architecture", "Understanding the Extension Architecture" }
                }
                p { "Learn about our powerful extension system" }
                span { class: "post-meta", "Published on 2024-01-02" }
            }
        }
    }
}