//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use client::{Post, User, Session, UserRole};

#[cfg(not(target_arch = "wasm32"))]
use sqlx::Row;

#[cfg(not(target_arch = "wasm32"))]
mod database;
#[cfg(not(target_arch = "wasm32"))]
mod email;
#[cfg(not(target_arch = "wasm32"))]
use database::Database;
#[cfg(not(target_arch = "wasm32"))]
use email::EmailService;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

/// Get all published posts
#[server(GetPosts)]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    db.get_published_posts().await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// Get post by ID
#[server(GetPostById)]
pub async fn get_post_by_id(id: u32) -> Result<Option<Post>, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    db.get_post_by_id(id).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// Get post by slug
#[server(GetPostBySlug)]
pub async fn get_post_by_slug(slug: String) -> Result<Option<Post>, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    db.get_post_by_slug(&slug).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// Save a post
#[server(SavePost)]
pub async fn save_post(post: Post) -> Result<u32, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    db.save_post(&post).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// Authenticate user
#[server(AuthenticateUser)]
pub async fn authenticate_user(username: String, password: String) -> Result<Session, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    match db.get_user_by_username(&username).await {
        Ok(Some(user)) => {
            // In a real implementation, you'd use proper password hashing
            if user.password_hash == password || user.password_hash == format!("hash_{}", password) {
                Ok(Session {
                    user_id: Some(user.id),
                    username: Some(user.username),
                    role: Some(user.role),
                    authenticated: true,
                })
            } else {
                Err(ServerFnError::ServerError("Invalid credentials".to_string()))
            }
        },
        Ok(None) => Err(ServerFnError::ServerError("User not found".to_string())),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

/// Get user by username
#[server(GetUserByUsername)]
pub async fn get_user_by_username(username: String) -> Result<Option<User>, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    db.get_user_by_username(&username).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// Initialize database with default data
#[server(InitDatabase)]
pub async fn init_database() -> Result<(), ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    db.init_default_data().await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// Register a new user and send verification email
#[server(RegisterUser)]
pub async fn register_user(
    username: String, 
    email: String, 
    password: String, 
    captcha_answer: Option<String>
) -> Result<String, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    // Check if this is the first user registration
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    let is_first_user = user_count == 0;
    
    // Validate captcha for first user
    if is_first_user {
        let captcha = captcha_answer.ok_or_else(|| 
            ServerFnError::ServerError("Captcha answer required for first user".to_string()))?;
        if captcha.trim().to_lowercase() != "a cool dude" {
            return Err(ServerFnError::ServerError("Incorrect captcha answer".to_string()));
        }
    }
    
    // Check if user already exists
    if let Ok(Some(_)) = db.get_user_by_username(&username).await {
        return Err(ServerFnError::ServerError("Username already exists".to_string()));
    }
    
    // Check if email already exists
    let existing_email: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(&db.pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    if existing_email.is_some() {
        return Err(ServerFnError::ServerError("Email already exists".to_string()));
    }
    
    // Generate verification token
    let verification_token = format!("verify_{}_{}", username, uuid::Uuid::new_v4());
    
    // Determine user role
    let role = if is_first_user {
        UserRole::Admin
    } else {
        UserRole::Subscriber
    };
    
    // Create user
    let user = User {
        id: 0, // Will be auto-assigned
        username: username.clone(),
        email: email.clone(),
        password_hash: format!("hash_{}", password), // In production, use proper password hashing
        role,
        created_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        active: true,
        email_verified: false,
        verification_token: Some(verification_token.clone()),
    };
    
    // Save user to database
    let user_id = db.create_user(&user).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    // Send verification email
    let email_service = EmailService::new()
        .map_err(|e| ServerFnError::ServerError(format!("Failed to initialize email service: {}", e)))?;
    
    email_service.send_verification_email(&email, &username, &verification_token).await
        .map_err(|e| ServerFnError::ServerError(format!("Failed to send verification email: {}", e)))?;
    
    Ok(format!("User registered successfully! Please check your email to verify your account. User ID: {}", user_id))
}

/// Verify user email with token
#[server(VerifyEmail)]
pub async fn verify_email(token: String) -> Result<String, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    // Find user by verification token
    let user_row = sqlx::query(
        "SELECT id, username, email, verification_token FROM users WHERE verification_token = ? AND email_verified = 0"
    )
    .bind(&token)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    let user_row = user_row.ok_or_else(|| 
        ServerFnError::ServerError("Invalid or expired verification token".to_string()))?;
    
    let user_id: i64 = user_row.get("id");
    let username: String = user_row.get("username");
    let email: String = user_row.get("email");
    
    // Update user to mark email as verified
    sqlx::query("UPDATE users SET email_verified = 1, verification_token = NULL WHERE id = ?")
        .bind(user_id)
        .execute(&db.pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    // Send welcome email
    let email_service = EmailService::new()
        .map_err(|e| ServerFnError::ServerError(format!("Failed to initialize email service: {}", e)))?;
    
    if let Err(e) = email_service.send_welcome_email(&email, &username).await {
        log::warn!("Failed to send welcome email to {}: {}", email, e);
        // Don't fail the verification if welcome email fails
    }
    
    Ok("Email verified successfully! You can now log in to your account.".to_string())
}

/// Check if this would be the first user registration
#[server(IsFirstUser)]
pub async fn is_first_user() -> Result<bool, ServerFnError> {
    let db = Database::init("sqlite://cms.db").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    Ok(user_count == 0)
}
