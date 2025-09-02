use dioxus::prelude::*;
use crate::views::{Home,Blog};
use crate::extensions::{PostView, PageView, LoginPage, RegisterPage, EmailVerificationPage, AdminDashboard};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/post/:slug")]
    PostRoute { slug: String },
    #[route("/page/:slug")]
    PageRoute { slug: String },
    #[route("/login")]
    LoginRoute {},
    #[route("/register")]
    RegisterRoute {},
    #[route("/verify-email")]
    VerifyEmailRoute {},
    #[route("/admin")]
    AdminRoute {},
}

// Route components
#[component]
fn PostRoute(slug: String) -> Element {
    rsx! { PostView { slug } }
}

#[component]
fn PageRoute(slug: String) -> Element {
    rsx! { PageView { slug } }
}

#[component]
fn LoginRoute() -> Element {
    rsx! { LoginPage {} }
}

#[component]
fn RegisterRoute() -> Element {
    rsx! { RegisterPage {} }
}

#[component]
fn VerifyEmailRoute() -> Element {
    rsx! { EmailVerificationPage {} }
}

#[component]
fn AdminRoute() -> Element {
    rsx! { AdminDashboard {} }
}

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "main-nav",
            div {
                class: "nav-brand",
                Link {
                    to: Route::Home {},
                    "🍌 BananaBit CMS"
                }
            }
            div {
                class: "nav-links",
                Link {
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    to: Route::Blog { id: 0},
                    "Blog"
                }
                Link {
                    to: Route::PageRoute { slug: "about".to_string() },
                    "About"
                }
                Link {
                    to: Route::PageRoute { slug: "contact".to_string() },
                    "Contact"
                }
            }
            div {
                class: "nav-auth",
                Link {
                    to: Route::LoginRoute {},
                    "Login"
                }
                Link {
                    to: Route::RegisterRoute {},
                    "Register"
                }
                Link {
                    to: Route::AdminRoute {},
                    "Admin"
                }
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
        document::Link { rel: "stylesheet", href: "/assets/components.css" }

        Router::<Route> {}
    }
}
