use dioxus::prelude::*;
use crate::Markdown;
use crate::Route;

/// Sample markdown content for demonstration
fn get_blog_content(id: i32) -> String {
    match id {
        0 => "/assets/blog/0.md".to_string(),
        _ => "/assets/blog/none.md".to_string(),
    }
}

#[component]
pub fn Blog(id: i32) -> Element {
    // Get the blog content for this ID
    let markdown_content = get_blog_content(id);
    
    // Define the base path for images (if you have local images)
    let image_base_path = "/assets/images";
    
    rsx! {
        document::Link { rel: "stylesheet", href: "/assets/blog.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/markdown.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/syntax.css"}

        div {
            id: "blog",
            class: "markdown-container",

            // Render the markdown content using our enhanced component
            Markdown {
                image_base_path: Some(image_base_path.to_string()),
                file_path: Some(markdown_content),
                id: Some(format!("blog-content-{}", id))
            }

            // Navigation links
            div {
                class: "blog-navigation",
                Link {
                    to: Route::Blog { id: id - 1 },
                    class: if id <= 1 { "disabled-link" } else { "" },
                    "Previous"
                }
                span { " | " }
                Link {
                    to: Route::Home {},
                    "Home"
                }
                span { " | " }
                Link {
                    to: Route::Blog { id: id + 1 },
                    "Next"
                }
            }
        }
    }
}
