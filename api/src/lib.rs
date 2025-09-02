//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use client::{Post, User, Session};

mod database;
use database::Database;

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
