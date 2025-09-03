use dioxus::prelude::*;
use crate::navbar::Route;
use crate::Markdown;
use crate::extensions::CommentSection;

#[component]
pub fn Blog(id: i32) -> Element {
    let content = use_resource(move || async move {
        let path = match id {
            0 => "/assets/blog/0.md",
            _ => "/assets/blog/none.md",
        };

        // Use gloo-net for WASM instead of reqwest
        match gloo_net::http::Request::get(path).send().await {
            Ok(resp) => resp.text().await.unwrap_or_else(|_| "Error reading file".to_string()),
            Err(_) => "Error fetching blog".to_string(),
        }
    });

    let image_base_path = "/assets/images";

    rsx! {
        document::Link { rel: "stylesheet", href: "/assets/blog.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/markdown.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/syntax.css"}

        div {
            id: "blog",
            class: "blog-post",

            // Post content
            article {
                class: "markdown-container",
                match content.read().as_ref() {
                    Some(markdown) => rsx! {
                        Markdown {
                            content: Some(markdown.clone()),
                            image_base_path: Some(image_base_path.to_string()),
                            id: Some(format!("blog-content-{}", id))
                        }
                    },
                    None => rsx! { p { "Loading Blog..." } }
                }
            }

            // Comments section
            if id == 0 {
                CommentSection { post_id: id as u32 }
            }

            // Navigation
            div {
                class: "blog-navigation",
                Link {
                    to: Route::Blog { id: id - 1 },
                    class: if id <= 1 { "disabled-link" } else { "" },
                    "← Previous"
                }
                span { " | " }
                Link { to: Route::Home {}, "Home" }
                span { " | " }
                Link { to: Route::Blog { id: id + 1 }, "Next →" }
            }
        }
    }
}