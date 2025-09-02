use dioxus::prelude::*;
use crate::extensions::{PostList, PageList};
use crate::navbar::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "home-page",
            
            // Hero section
            section {
                class: "hero",
                div {
                    class: "hero-content",
                    h1 { "Welcome to BananaBit CMS" }
                    p { "A modern, extension-based content management system built with Rust and Dioxus" }
                    div {
                        class: "hero-actions",
                        Link {
                            to: Route::Blog { id: 0 },
                            class: "btn btn-primary",
                            "Read Our Blog"
                        }
                        Link {
                            to: Route::PageRoute { slug: "about".to_string() },
                            class: "btn btn-secondary",
                            "Learn More"
                        }
                    }
                }
            }
            
            // Features section
            section {
                class: "features",
                div {
                    class: "container",
                    h2 { "Key Features" }
                    div {
                        class: "features-grid",
                        div {
                            class: "feature-card",
                            h3 { "🧩 Extension-Based" }
                            p { "Everything is a plugin, making the system highly modular and customizable" }
                        }
                        div {
                            class: "feature-card",
                            h3 { "🚀 Performance" }
                            p { "Built with Rust for maximum performance and memory safety" }
                        }
                        div {
                            class: "feature-card",
                            h3 { "🔒 Security" }
                            p { "Type-safe code and secure defaults protect your content" }
                        }
                        div {
                            class: "feature-card",
                            h3 { "📝 Markdown" }
                            p { "Rich content editing with full markdown support and syntax highlighting" }
                        }
                    }
                }
            }
            
            // Recent posts section
            section {
                class: "recent-posts",
                div {
                    class: "container",
                    h2 { "Recent Posts" }
                    PostList {}
                }
            }
            
            // Pages section
            section {
                class: "pages-section",
                div {
                    class: "container",
                    h2 { "Pages" }
                    PageList {}
                }
            }
        }
    }
}
