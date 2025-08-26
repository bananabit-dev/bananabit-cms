use dioxus::prelude::*;
use crate::views::{Home,Blog};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 0},
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}

// TODO: Remove routes and app from here.

#[component]
pub fn App() -> Element {
    // Build cool things 

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: "/assets/favicon.ico" }
        document::Link { rel: "stylesheet", href: "/assets/main.css" }

        Router::<Route> {}
    }
}
