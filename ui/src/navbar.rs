use dioxus::prelude::*;
use crate::views::{Home,Blog};
use crate::extensions::{PostView, PageView, LoginPage, RegisterPage, EmailVerificationPage};

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
    // Check if user is authenticated and has admin privileges
    let auth_state = use_signal(|| None::<client::Session>);
    
    use_effect(move || {
        spawn(async move {
            // TODO: Check current session/authentication state
            // For now, we'll just check if there's any user in the database
            match api::is_first_user().await {
                Ok(true) => {
                    // No users exist yet, redirect to register
                    dioxus::router::navigator().push("/register");
                },
                Ok(false) => {
                    // Users exist, but we need to check authentication
                    // For now, redirect to login since we don't have session management
                    dioxus::router::navigator().push("/login");
                },
                Err(_) => {
                    // Error checking users, redirect to login
                    dioxus::router::navigator().push("/login");
                }
            }
        });
    });
    
    rsx! { 
        div {
            class: "admin-check",
            p { "Checking authentication..." }
        }
    }
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
