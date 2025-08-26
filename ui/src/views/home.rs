use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
            div {
            h1 { "Welcome to the Home Page this is currently a work in progress check out the blog" }
        }
    }
}
