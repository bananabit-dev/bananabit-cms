use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::path::Path;
use client::{Post, User, UserRole};

/// Database manager for the CMS
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    /// Initialize database connection and create tables
    pub async fn init(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create database file if it doesn't exist
        if let Some(parent) = Path::new(database_url.trim_start_matches("sqlite://")).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let pool = SqlitePool::connect(database_url).await?;
        
        let database = Self { pool };
        database.create_tables().await?;
        
        Ok(database)
    }
    
    /// Create necessary tables
    async fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Posts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                slug TEXT UNIQUE NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                author TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                published BOOLEAN NOT NULL DEFAULT 0,
                scheduled_at TEXT,
                meta_description TEXT,
                meta_keywords TEXT
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL,
                created_at TEXT NOT NULL,
                active BOOLEAN NOT NULL DEFAULT 1,
                email_verified BOOLEAN NOT NULL DEFAULT 0,
                verification_token TEXT
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Media table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS media (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                filename TEXT NOT NULL,
                original_name TEXT NOT NULL,
                mime_type TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                uploaded_at TEXT NOT NULL,
                uploaded_by INTEGER,
                alt_text TEXT,
                FOREIGN KEY (uploaded_by) REFERENCES users(id)
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Themes table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS themes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                description TEXT,
                css_content TEXT NOT NULL,
                active BOOLEAN NOT NULL DEFAULT 0
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Settings table for configuration
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                description TEXT
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Migrate existing users table if needed
        self.migrate_users_table().await?;
        
        Ok(())
    }
    
    /// Migrate users table to add email verification fields
    async fn migrate_users_table(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if email_verified column exists
        let columns: Vec<String> = sqlx::query("PRAGMA table_info(users)")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();
        
        if !columns.contains(&"email_verified".to_string()) {
            sqlx::query("ALTER TABLE users ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT 0")
                .execute(&self.pool)
                .await?;
        }
        
        if !columns.contains(&"verification_token".to_string()) {
            sqlx::query("ALTER TABLE users ADD COLUMN verification_token TEXT")
                .execute(&self.pool)
                .await?;
        }
        
        Ok(())
    }
    
    /// Get all published posts
    pub async fn get_published_posts(&self) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT id, slug, title, content, author, created_at, updated_at, published 
             FROM posts WHERE published = 1 ORDER BY id DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let posts = rows.into_iter().map(|row| Post {
            id: row.get::<i64, _>("id") as u32,
            slug: row.get("slug"),
            title: row.get("title"),
            content: row.get("content"),
            author: row.get("author"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            published: row.get("published"),
        }).collect();
        
        Ok(posts)
    }
    
    /// Get post by ID
    pub async fn get_post_by_id(&self, id: u32) -> Result<Option<Post>, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            "SELECT id, slug, title, content, author, created_at, updated_at, published 
             FROM posts WHERE id = ?"
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Some(Post {
                id: row.get::<i64, _>("id") as u32,
                slug: row.get("slug"),
                title: row.get("title"),
                content: row.get("content"),
                author: row.get("author"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                published: row.get("published"),
            })),
            None => Ok(None),
        }
    }
    
    /// Get post by slug
    pub async fn get_post_by_slug(&self, slug: &str) -> Result<Option<Post>, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            "SELECT id, slug, title, content, author, created_at, updated_at, published 
             FROM posts WHERE slug = ?"
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Some(Post {
                id: row.get::<i64, _>("id") as u32,
                slug: row.get("slug"),
                title: row.get("title"),
                content: row.get("content"),
                author: row.get("author"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                published: row.get("published"),
            })),
            None => Ok(None),
        }
    }
    
    /// Create or update a post
    pub async fn save_post(&self, post: &Post) -> Result<u32, Box<dyn std::error::Error>> {
        if post.id == 0 {
            // Insert new post
            let result = sqlx::query(
                "INSERT INTO posts (slug, title, content, author, created_at, updated_at, published)
                 VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&post.slug)
            .bind(&post.title)
            .bind(&post.content)
            .bind(&post.author)
            .bind(&post.created_at)
            .bind(&post.updated_at)
            .bind(post.published)
            .execute(&self.pool)
            .await?;
            
            Ok(result.last_insert_rowid() as u32)
        } else {
            // Update existing post
            sqlx::query(
                "UPDATE posts SET slug=?, title=?, content=?, author=?, updated_at=?, published=?
                 WHERE id=?"
            )
            .bind(&post.slug)
            .bind(&post.title)
            .bind(&post.content)
            .bind(&post.author)
            .bind(&post.updated_at)
            .bind(post.published)
            .bind(post.id as i64)
            .execute(&self.pool)
            .await?;
            
            Ok(post.id)
        }
    }
    
    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, role, created_at, active, email_verified, verification_token
             FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let role_str: String = row.get("role");
                let role = match role_str.as_str() {
                    "Admin" => UserRole::Admin,
                    "Editor" => UserRole::Editor,
                    "Author" => UserRole::Author,
                    "Subscriber" => UserRole::Subscriber,
                    _ => UserRole::Subscriber,
                };
                
                Ok(Some(User {
                    id: row.get::<i64, _>("id") as u32,
                    username: row.get("username"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role,
                    created_at: row.get("created_at"),
                    active: row.get("active"),
                    email_verified: row.get::<bool, _>("email_verified"),
                    verification_token: row.get("verification_token"),
                }))
            },
            None => Ok(None),
        }
    }
    
    /// Create a new user
    pub async fn create_user(&self, user: &User) -> Result<u32, Box<dyn std::error::Error>> {
        let role_str = match user.role {
            UserRole::Admin => "Admin",
            UserRole::Editor => "Editor",
            UserRole::Author => "Author",
            UserRole::Subscriber => "Subscriber",
        };
        
        let result = sqlx::query(
            "INSERT INTO users (username, email, password_hash, role, created_at, active, email_verified, verification_token)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(role_str)
        .bind(&user.created_at)
        .bind(user.active)
        .bind(user.email_verified)
        .bind(&user.verification_token)
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid() as u32)
    }
    
    /// Initialize with default data
    pub async fn init_default_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we already have data
        let post_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
            .fetch_one(&self.pool)
            .await?;
            
        if post_count == 0 {
            // Add default post
            let welcome_post = Post {
                id: 0, // Will be auto-assigned
                slug: "welcome-to-bananabit-cms".to_string(),
                title: "Welcome to BananaBit CMS".to_string(),
                content: r#"# Welcome to BananaBit CMS

This is a modern, extension-based content management system built with Rust and Dioxus.

## Features

- **Extension-Based Architecture**: Everything is a plugin
- **Performance**: Built with Rust for maximum performance
- **Modern UI**: Uses Dioxus for a reactive frontend
- **Database Persistence**: SQLite database for reliable storage
- **Media Management**: Built-in file upload and management
- **Theme System**: Customizable themes
- **SEO-Friendly**: Advanced SEO features built-in

Welcome to the future of content management!"#.to_string(),
                author: "Admin".to_string(),
                created_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                updated_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                published: true,
            };
            
            self.save_post(&welcome_post).await?;
        }
        
        // No default users created - first registered user will be admin
        
        Ok(())
    }
}