//! API client for communicating with the CMS server

use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {}", self.message)
    }
}

impl std::error::Error for ApiError {}

pub type ApiResult<T> = Result<T, ApiError>;

/// Client for interacting with CMS API
pub struct CmsClient {
    base_url: String,
}

impl CmsClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn default() -> Self {
        Self::new("http://localhost:8080".to_string())
    }

    // Post operations
    pub async fn get_posts(&self) -> ApiResult<Vec<Post>> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_net::http::Request;
            let response = Request::get(&format!("{}/api/posts", self.base_url))
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = reqwest::get(&format!("{}/api/posts", self.base_url))
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })
        }
    }

    pub async fn get_post_by_id(&self, id: u32) -> ApiResult<Option<Post>> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_net::http::Request;
            let response = Request::get(&format!("{}/api/posts/{}", self.base_url, id))
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            if response.status() == 404 {
                return Ok(None);
            }
            
            let post = response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            Ok(Some(post))
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = reqwest::get(&format!("{}/api/posts/{}", self.base_url, id))
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            if response.status() == 404 {
                return Ok(None);
            }
            
            let post = response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            Ok(Some(post))
        }
    }

    pub async fn get_post_by_slug(&self, slug: &str) -> ApiResult<Option<Post>> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_net::http::Request;
            let response = Request::get(&format!("{}/api/posts/slug/{}", self.base_url, slug))
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            if response.status() == 404 {
                return Ok(None);
            }
            
            let post = response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            Ok(Some(post))
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = reqwest::get(&format!("{}/api/posts/slug/{}", self.base_url, slug))
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            if response.status() == 404 {
                return Ok(None);
            }
            
            let post = response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            Ok(Some(post))
        }
    }

    pub async fn create_post(&self, post: &Post) -> ApiResult<Post> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_net::http::Request;
            let response = Request::post(&format!("{}/api/posts", self.base_url))
                .json(post)
                .map_err(|e| ApiError { message: e.to_string() })?
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let client = reqwest::Client::new();
            let response = client.post(&format!("{}/api/posts", self.base_url))
                .json(post)
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })
        }
    }

    // User operations
    pub async fn authenticate(&self, username: &str, password: &str) -> ApiResult<Session> {
        let credentials = serde_json::json!({
            "username": username,
            "password": password
        });

        #[cfg(target_arch = "wasm32")]
        {
            use gloo_net::http::Request;
            let response = Request::post(&format!("{}/api/auth/login", self.base_url))
                .json(&credentials)
                .map_err(|e| ApiError { message: e.to_string() })?
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let client = reqwest::Client::new();
            let response = client.post(&format!("{}/api/auth/login", self.base_url))
                .json(&credentials)
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })
        }
    }

    pub async fn get_user_by_username(&self, username: &str) -> ApiResult<Option<User>> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_net::http::Request;
            let response = Request::get(&format!("{}/api/users/{}", self.base_url, username))
                .send()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            if response.status() == 404 {
                return Ok(None);
            }
            
            let user = response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            Ok(Some(user))
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = reqwest::get(&format!("{}/api/users/{}", self.base_url, username))
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            
            if response.status() == 404 {
                return Ok(None);
            }
            
            let user = response.json()
                .await
                .map_err(|e| ApiError { message: e.to_string() })?;
            Ok(Some(user))
        }
    }
}