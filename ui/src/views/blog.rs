use dioxus::prelude::*;
use crate::Markdown;
use crate::Route;

/// Sample markdown content for demonstration
fn get_blog_content(id: i32) -> String {
    match id {
        1 => "/assets/blog/1.md".to_string(),
        _ => format!(r#"
# Blog {id}

This is blog post number {id}.

## Dynamic Content

- This is a dynamically generated blog post
- You can navigate to other posts using the links below
- Try posts 1-3 for special content examples

```rust
// This is a code block in blog post {id}
fn get_blog_id() -> i32 {{
    {id}
}}
```

[Go to Blog 1](/blog/1)
        "#)
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
                content: markdown_content,
                image_base_path: Some(image_base_path.to_string()),
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
