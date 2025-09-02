use std::collections::HashMap;

pub mod posts;
pub mod comments;
pub mod auth;
pub mod pages;
pub mod media;
pub mod themes;
pub mod seo;
pub mod scheduling;
pub mod i18n;
pub mod analytics;

pub use posts::*;
pub use comments::*;
pub use auth::*;
pub use pages::*;
pub use media::*;
pub use themes::*;
pub use seo::*;
pub use scheduling::*;
pub use i18n::*;
pub use analytics::*;

// Re-export types from client
pub use client::{Post, User, UserRole, Session, Comment, MediaFile, Theme, SeoMetadata, AnalyticsEvent};

/// Core trait that all extensions must implement
pub trait Extension {
    /// Unique identifier for this extension
    fn id(&self) -> &'static str;
    
    /// Human-readable name
    fn name(&self) -> &'static str;
    
    /// Version of the extension
    fn version(&self) -> &'static str;
    
    /// Initialize the extension (called once on startup)
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get routes that this extension provides
    fn routes(&self) -> Vec<ExtensionRoute> {
        Vec::new()
    }
    
    /// Get components that this extension provides
    fn components(&self) -> Vec<ExtensionComponent> {
        Vec::new()
    }
    
    /// Get hooks for various system events
    fn hooks(&self) -> ExtensionHooks {
        ExtensionHooks::default()
    }
}

/// Route provided by an extension
#[derive(Debug, Clone)]
pub struct ExtensionRoute {
    pub path: String,
    pub requires_auth: bool,
    pub admin_only: bool,
}

/// Component provided by an extension
#[derive(Debug, Clone)]
pub struct ExtensionComponent {
    pub name: String,
    pub description: String,
}

/// Hooks for system events
#[derive(Debug, Default)]
pub struct ExtensionHooks {
    pub before_render: Option<fn()>,
    pub after_render: Option<fn()>,
    pub on_route_change: Option<fn(&str)>,
}

/// Extension manager that handles all registered extensions
pub struct ExtensionManager {
    extensions: HashMap<String, Box<dyn Extension>>,
    initialized: bool,
}

impl ExtensionManager {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            initialized: false,
        }
    }
    
    /// Register a new extension
    pub fn register<T: Extension + 'static>(&mut self, extension: T) {
        let id = extension.id().to_string();
        self.extensions.insert(id, Box::new(extension));
    }
    
    /// Initialize all extensions
    pub fn init_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            return Ok(());
        }
        
        for (id, extension) in &mut self.extensions {
            if let Err(e) = extension.init() {
                eprintln!("Failed to initialize extension {}: {}", id, e);
                return Err(e);
            }
        }
        
        self.initialized = true;
        Ok(())
    }
    
    /// Get all routes from all extensions
    pub fn get_all_routes(&self) -> Vec<ExtensionRoute> {
        self.extensions
            .values()
            .flat_map(|ext| ext.routes())
            .collect()
    }
    
    /// Get extension by ID
    pub fn get_extension(&self, id: &str) -> Option<&dyn Extension> {
        self.extensions.get(id).map(|e| e.as_ref())
    }
    
    /// List all registered extensions
    pub fn list_extensions(&self) -> Vec<(&str, &str, &str)> {
        self.extensions
            .values()
            .map(|ext| (ext.id(), ext.name(), ext.version()))
            .collect()
    }
}

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}