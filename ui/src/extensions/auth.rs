use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// User data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password_hash: String, // In real app, this would be properly hashed
    pub role: UserRole,
    pub created_at: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Editor,
    Author,
    Subscriber,
}

/// Current session information
#[derive(Debug, Clone, Default)]
pub struct Session {
    pub user_id: Option<u32>,
    pub username: Option<String>,
    pub role: Option<UserRole>,
    pub authenticated: bool,
}

/// Authentication extension - handles user auth and sessions
pub struct AuthExtension {
    users: HashMap<u32, User>,
    sessions: HashMap<String, Session>, // session_id -> session
    current_session: Session,
    next_user_id: u32,
}

impl AuthExtension {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
            current_session: Session::default(),
            next_user_id: 1,
        }
    }
    
    pub fn create_user(&mut self, username: String, email: String, password: String, role: UserRole) -> Result<u32, String> {
        // Check if user already exists
        if self.users.values().any(|u| u.username == username || u.email == email) {
            return Err("User already exists".to_string());
        }
        
        let user = User {
            id: self.next_user_id,
            username,
            email,
            password_hash: format!("hash_{}", password), // Simplified for demo
            role,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            active: true,
        };
        
        let user_id = user.id;
        self.users.insert(user_id, user);
        self.next_user_id += 1;
        
        Ok(user_id)
    }
    
    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<Session, String> {
        let user = self.users
            .values()
            .find(|u| u.username == username && u.active)
            .ok_or("Invalid credentials")?;
        
        // Simplified password check
        if user.password_hash != format!("hash_{}", password) {
            return Err("Invalid credentials".to_string());
        }
        
        let session = Session {
            user_id: Some(user.id),
            username: Some(user.username.clone()),
            role: Some(user.role.clone()),
            authenticated: true,
        };
        
        self.current_session = session.clone();
        Ok(session)
    }
    
    pub fn logout(&mut self) {
        self.current_session = Session::default();
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.current_session.authenticated
    }
    
    pub fn is_admin(&self) -> bool {
        matches!(self.current_session.role, Some(UserRole::Admin))
    }
    
    pub fn can_edit(&self) -> bool {
        matches!(self.current_session.role, Some(UserRole::Admin | UserRole::Editor | UserRole::Author))
    }
    
    pub fn current_user(&self) -> Option<&User> {
        let user_id = self.current_session.user_id?;
        self.users.get(&user_id)
    }
}

impl Extension for AuthExtension {
    fn id(&self) -> &'static str {
        "core.auth"
    }
    
    fn name(&self) -> &'static str {
        "Authentication System"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create default admin user
        self.create_user(
            "admin".to_string(),
            "admin@bananabit.cms".to_string(),
            "admin123".to_string(),
            UserRole::Admin,
        ).map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        
        // Create a sample editor
        self.create_user(
            "editor".to_string(),
            "editor@bananabit.cms".to_string(),
            "editor123".to_string(),
            UserRole::Editor,
        ).map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            ExtensionRoute {
                path: "/login".to_string(),
                requires_auth: false,
                admin_only: false,
            },
            ExtensionRoute {
                path: "/admin".to_string(),
                requires_auth: true,
                admin_only: true,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "LoginForm".to_string(),
                description: "User login form".to_string(),
            },
            ExtensionComponent {
                name: "UserInfo".to_string(),
                description: "Display current user information".to_string(),
            },
        ]
    }
}

#[component]
pub fn LoginPage() -> Element {
    rsx! {
        div {
            class: "login-page",
            div {
                class: "login-container",
                h1 { "Login to BananaBit CMS" }
                LoginForm {}
            }
        }
    }
}

#[component]
pub fn LoginForm() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut success = use_signal(|| false);
    
    let on_submit = move |_| {
        error.set(String::new());
        
        if username().is_empty() || password().is_empty() {
            error.set("Please fill in all fields".to_string());
            return;
        }
        
        // In a real implementation, this would call the auth extension
        if username() == "admin" && password() == "admin123" {
            success.set(true);
        } else {
            error.set("Invalid credentials".to_string());
        }
    };
    
    rsx! {
        div {
            class: "login-form",
            
            if success() {
                div {
                    class: "success-message",
                    p { "Login successful! Welcome back." }
                }
            } else {
                form {
                    onsubmit: on_submit,
                    prevent_default: "onsubmit",
                    
                    if !error().is_empty() {
                        div {
                            class: "error-message",
                            p { "{error}" }
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { r#for: "username", "Username:" }
                        input {
                            r#type: "text",
                            id: "username",
                            value: "{username}",
                            oninput: move |e| username.set(e.value().clone()),
                            placeholder: "Enter your username",
                            required: true
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { r#for: "password", "Password:" }
                        input {
                            r#type: "password",
                            id: "password",
                            value: "{password}",
                            oninput: move |e| password.set(e.value().clone()),
                            placeholder: "Enter your password",
                            required: true
                        }
                    }
                    
                    div {
                        class: "form-group",
                        button {
                            r#type: "submit",
                            class: "login-btn",
                            "Login"
                        }
                    }
                    
                    div {
                        class: "login-help",
                        p { 
                            small { 
                                "Demo credentials: admin / admin123" 
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn UserInfo() -> Element {
    // In a real implementation, this would get the current user from the auth extension
    let is_logged_in = true; // Simulated
    let username = "admin"; // Simulated
    
    rsx! {
        div {
            class: "user-info",
            if is_logged_in {
                div {
                    class: "user-status",
                    span { "Welcome, {username}!" }
                    button {
                        class: "logout-btn",
                        onclick: move |_| {
                            // In real app, this would call logout
                        },
                        "Logout"
                    }
                }
            } else {
                div {
                    class: "user-status",
                    a { href: "/login", "Login" }
                }
            }
        }
    }
}

#[component]
pub fn AdminDashboard() -> Element {
    rsx! {
        div {
            class: "admin-dashboard",
            h1 { "Admin Dashboard" }
            
            div {
                class: "admin-nav",
                ul {
                    li { a { href: "/admin/posts", "Manage Posts" } }
                    li { a { href: "/admin/comments", "Manage Comments" } }
                    li { a { href: "/admin/users", "Manage Users" } }
                    li { a { href: "/admin/extensions", "Extensions" } }
                }
            }
            
            div {
                class: "admin-content",
                h2 { "Welcome to the Admin Panel" }
                p { "From here you can manage all aspects of your CMS." }
                
                div {
                    class: "quick-stats",
                    div {
                        class: "stat-box",
                        h3 { "2" }
                        p { "Posts" }
                    }
                    div {
                        class: "stat-box", 
                        h3 { "2" }
                        p { "Comments" }
                    }
                    div {
                        class: "stat-box",
                        h3 { "2" }
                        p { "Users" }
                    }
                }
            }
        }
    }
}