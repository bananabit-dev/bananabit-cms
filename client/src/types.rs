//! Shared data types for BananaBit CMS

use serde::{Deserialize, Serialize};

/// Post data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub published: bool,
}

/// User data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    pub user_id: Option<u32>,
    pub username: Option<String>,
    pub role: Option<UserRole>,
    pub authenticated: bool,
}

/// Comment data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub post_id: u32,
    pub author: String,
    pub content: String,
    pub created_at: String,
    pub approved: bool,
}

/// Media file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub id: u32,
    pub filename: String,
    pub original_name: String,
    pub mime_type: String,
    pub file_size: u64,
    pub uploaded_at: String,
    pub uploaded_by: Option<u32>,
    pub alt_text: Option<String>,
}

/// Theme information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub css_content: String,
    pub active: bool,
}

/// SEO metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoMetadata {
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
}

/// Analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: u32,
    pub event_type: String,
    pub path: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub timestamp: String,
}