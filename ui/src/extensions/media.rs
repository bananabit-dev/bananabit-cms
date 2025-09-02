use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent, MediaFile};
use std::collections::HashMap;
use client::time::now_iso8601;

/// Extended media file with computed fields for UI
#[derive(Debug, Clone)]
pub struct ExtendedMediaFile {
    pub media: MediaFile,
    pub url: String, // Computed field for serving
}

impl ExtendedMediaFile {
    pub fn from_media(media: MediaFile) -> Self {
        let url = format!("/uploads/{}", media.filename);
        Self { media, url }
    }
}

/// Media management extension
pub struct MediaExtension {
    media_files: HashMap<u32, ExtendedMediaFile>,
    next_id: u32,
    upload_dir: String,
}

impl MediaExtension {
    pub fn new() -> Self {
        Self {
            media_files: HashMap::new(),
            next_id: 1,
            upload_dir: "uploads".to_string(),
        }
    }
    
    pub fn get_media_files(&self) -> Vec<&ExtendedMediaFile> {
        self.media_files.values().collect()
    }
    
    pub fn get_media_by_id(&self, id: u32) -> Option<&ExtendedMediaFile> {
        self.media_files.get(&id)
    }
    
    pub fn add_media_file(&mut self, mut media: MediaFile) -> u32 {
        media.id = self.next_id;
        let extended = ExtendedMediaFile::from_media(media);
        self.media_files.insert(self.next_id, extended);
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    pub fn delete_media_file(&mut self, id: u32) -> Option<ExtendedMediaFile> {
        self.media_files.remove(&id)
    }
    
    pub fn update_alt_text(&mut self, id: u32, alt_text: String) -> bool {
        if let Some(media) = self.media_files.get_mut(&id) {
            media.media.alt_text = Some(alt_text);
            true
        } else {
            false
        }
    }
}

impl Extension for MediaExtension {
    fn id(&self) -> &'static str {
        "core.media"
    }
    
    fn name(&self) -> &'static str {
        "Media Management"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create upload directory if it doesn't exist
        std::fs::create_dir_all(&self.upload_dir)?;
        
        // Add some sample media files for demo
        let sample_image = MediaFile {
            id: 0,
            filename: "bananabit-logo.png".to_string(),
            original_name: "logo.png".to_string(),
            mime_type: "image/png".to_string(),
            file_size: 15432,
            uploaded_at: now_iso8601(),
            uploaded_by: Some(1), // Admin user
            alt_text: Some("BananaBit CMS Logo".to_string()),
        };
        
        self.add_media_file(sample_image);
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            ExtensionRoute {
                path: "/admin/media".to_string(),
                requires_auth: true,
                admin_only: false,
            },
            ExtensionRoute {
                path: "/uploads/*".to_string(),
                requires_auth: false,
                admin_only: false,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "MediaLibrary".to_string(),
                description: "Browse and manage uploaded media files".to_string(),
            },
            ExtensionComponent {
                name: "MediaUpload".to_string(),
                description: "Upload new media files".to_string(),
            },
            ExtensionComponent {
                name: "MediaPicker".to_string(),
                description: "Select media files for content".to_string(),
            },
        ]
    }
}

/// Media library component for browsing uploaded files
#[component]
pub fn MediaLibrary() -> Element {
    rsx! {
        div {
            h2 { "Media Library" }
            
            div {
                h3 { "Upload New Media" }
                input {
                    r#type: "file",
                    multiple: true,
                    accept: "image/*,video/*,audio/*,.pdf,.doc,.docx",
                    onchange: move |_event| {
                        // Handle file upload
                        println!("Files selected for upload");
                    }
                }
                p { "Drag and drop files here or click to browse. Supported formats: Images, Videos, Audio, PDF, Documents" }
            }
            
            div {
                // Demo media items
                div {
                    img {
                        src: "/uploads/bananabit-logo.png",
                        alt: "BananaBit CMS Logo",
                        width: "150",
                        height: "150"
                    }
                    div {
                        h4 { "bananabit-logo.png" }
                        p { "PNG Image • 15.4 KB" }
                        input {
                            r#type: "text",
                            placeholder: "Alt text...",
                            value: "BananaBit CMS Logo"
                        }
                        div {
                            button { "Edit" }
                            button { "Delete" }
                        }
                    }
                }
                
                div {
                    div {
                        span { "+" }
                        p { "Upload Media" }
                    }
                }
            }
        }
    }
}

/// Media picker component for selecting files in content
#[component]
pub fn MediaPicker(on_select: EventHandler<MediaFile>) -> Element {
    rsx! {
        div {
            h3 { "Select Media" }
            
            div {
                div {
                    onclick: move |_| {
                        let sample_media = MediaFile {
                            id: 1,
                            filename: "bananabit-logo.png".to_string(),
                            original_name: "logo.png".to_string(),
                            mime_type: "image/png".to_string(),
                            file_size: 15432,
                            uploaded_at: now_iso8601(),
                            uploaded_by: Some(1),
                            alt_text: Some("BananaBit CMS Logo".to_string()),
                        };
                        on_select.call(sample_media);
                    },
                    img {
                        src: "/uploads/bananabit-logo.png",
                        alt: "BananaBit CMS Logo",
                        width: "100",
                        height: "100"
                    }
                    p { "bananabit-logo.png" }
                }
            }
        }
    }
}