use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent, User, UserRole, Session};
use std::collections::HashMap;

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
        
        // Generate verification token
        let verification_token = format!("verify_{}_{}_{}", username, self.next_user_id, "random_token");
        
        let user = User {
            id: self.next_user_id,
            username,
            email: email.clone(),
            password_hash: format!("hash_{}", password), // Simplified for demo
            role,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            active: true,
            email_verified: false,
            verification_token: Some(verification_token.clone()),
        };
        
        let user_id = user.id;
        self.users.insert(user_id, user);
        self.next_user_id += 1;
        
        // Send verification email (for now, just log it)
        println!("📧 Verification email sent to {}: Please verify your account using token: {}", email, verification_token);
        
        Ok(user_id)
    }
    
    pub fn register_user(&mut self, username: String, email: String, password: String, captcha_answer: Option<String>) -> Result<u32, String> {
        // Check if this is the first user
        let is_first_user = self.users.is_empty();
        
        // If first user, require captcha
        if is_first_user {
            let captcha_answer = captcha_answer.ok_or("Captcha answer required for first user")?;
            if captcha_answer.trim().to_lowercase() != "a cool dude" {
                return Err("Incorrect captcha answer".to_string());
            }
        }
        
        // First user becomes admin, others become subscribers
        let role = if is_first_user {
            UserRole::Admin
        } else {
            UserRole::Subscriber
        };
        
        self.create_user(username, email, password, role)
    }
    
    pub fn verify_email(&mut self, token: &str) -> Result<(), String> {
        let user = self.users.values_mut()
            .find(|u| u.verification_token.as_ref() == Some(&token.to_string()))
            .ok_or("Invalid verification token")?;
        
        user.email_verified = true;
        user.verification_token = None;
        
        Ok(())
    }
    
    pub fn is_first_user_registration(&self) -> bool {
        self.users.is_empty()
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
        // No default users created - first registered user will be admin
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
                path: "/register".to_string(),
                requires_auth: false,
                admin_only: false,
            },
            ExtensionRoute {
                path: "/verify-email".to_string(),
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
                div {
                    class: "auth-links",
                    p { 
                        "Don't have an account? "
                        a { href: "/register", "Register here" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RegisterPage() -> Element {
    rsx! {
        div {
            class: "register-page",
            div {
                class: "register-container",
                h1 { "Register for BananaBit CMS" }
                RegisterForm {}
                div {
                    class: "auth-links",
                    p { 
                        "Already have an account? "
                        a { href: "/login", "Login here" }
                    }
                }
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
                }
            }
        }
    }
}

#[component]
pub fn RegisterForm() -> Element {
    let mut username = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let mut captcha_answer = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut success = use_signal(|| false);
    let mut show_captcha = use_signal(|| true); // In real app, this would check if first user
    
    let on_submit = move |_| {
        error.set(String::new());
        
        if username().is_empty() || email().is_empty() || password().is_empty() || confirm_password().is_empty() {
            error.set("Please fill in all fields".to_string());
            return;
        }
        
        if password() != confirm_password() {
            error.set("Passwords do not match".to_string());
            return;
        }
        
        if show_captcha() && captcha_answer().trim().to_lowercase() != "a cool dude" {
            error.set("Incorrect captcha answer".to_string());
            return;
        }
        
        // In a real implementation, this would call the auth extension register_user method
        success.set(true);
    };
    
    rsx! {
        div {
            class: "register-form",
            
            if success() {
                div {
                    class: "success-message",
                    p { "Registration successful! Please check your email to verify your account." }
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
                            placeholder: "Choose a username",
                            required: true
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { r#for: "email", "Email:" }
                        input {
                            r#type: "email",
                            id: "email",
                            value: "{email}",
                            oninput: move |e| email.set(e.value().clone()),
                            placeholder: "Enter your email",
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
                            placeholder: "Create a password",
                            required: true
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { r#for: "confirm_password", "Confirm Password:" }
                        input {
                            r#type: "password",
                            id: "confirm_password",
                            value: "{confirm_password}",
                            oninput: move |e| confirm_password.set(e.value().clone()),
                            placeholder: "Confirm your password",
                            required: true
                        }
                    }
                    
                    if show_captcha() {
                        div {
                            class: "form-group captcha-group",
                            label { r#for: "captcha", "Security Question: Who's bananabit?" }
                            input {
                                r#type: "text",
                                id: "captcha",
                                value: "{captcha_answer}",
                                oninput: move |e| captcha_answer.set(e.value().clone()),
                                placeholder: "Answer the question",
                                required: true
                            }
                            small { 
                                class: "captcha-hint",
                                "Hint: The answer is two words describing a person" 
                            }
                        }
                    }
                    
                    div {
                        class: "form-group",
                        button {
                            r#type: "submit",
                            class: "register-btn",
                            "Register"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EmailVerificationPage() -> Element {
    let mut verification_token = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut success = use_signal(|| false);
    
    let on_submit = move |_| {
        error.set(String::new());
        
        if verification_token().is_empty() {
            error.set("Please enter your verification token".to_string());
            return;
        }
        
        // In a real implementation, this would call the auth extension verify_email method
        success.set(true);
    };
    
    rsx! {
        div {
            class: "verification-page",
            div {
                class: "verification-container",
                h1 { "Verify Your Email" }
                
                if success() {
                    div {
                        class: "success-message",
                        p { "Email verified successfully! You can now log in." }
                        a { href: "/login", "Go to Login" }
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
                        
                        p { "Please enter the verification token sent to your email:" }
                        
                        div {
                            class: "form-group",
                            label { r#for: "token", "Verification Token:" }
                            input {
                                r#type: "text",
                                id: "token",
                                value: "{verification_token}",
                                oninput: move |e| verification_token.set(e.value().clone()),
                                placeholder: "Enter your verification token",
                                required: true
                            }
                        }
                        
                        div {
                            class: "form-group",
                            button {
                                r#type: "submit",
                                class: "verify-btn",
                                "Verify Email"
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