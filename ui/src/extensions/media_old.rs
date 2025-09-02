use client::time::now_iso8601;
use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Media file data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub id: u32,
    pub filename: String,
    pub original_name: String,
    pub mime_type: String,
    pub file_size: u64,
    pub uploaded_at: String,
    pub uploaded_by: u32,
    pub alt_text: Option<String>,
    pub url: String, // Computed field for serving
}

/// Media management extension
pub struct MediaExtension {
    media_files: HashMap<u32, MediaFile>,
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
    
    pub fn get_media_files(&self) -> Vec<&MediaFile> {
        self.media_files.values().collect()
    }
    
    pub fn get_media_by_id(&self, id: u32) -> Option<&MediaFile> {
        self.media_files.get(&id)
    }
    
    pub fn add_media_file(&mut self, mut media: MediaFile) -> u32 {
        media.id = self.next_id;
        media.url = format!("/uploads/{}", media.filename);
        self.media_files.insert(self.next_id, media);
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    pub fn delete_media_file(&mut self, id: u32) -> Option<MediaFile> {
        self.media_files.remove(&id)
    }
    
    pub fn update_alt_text(&mut self, id: u32, alt_text: String) -> bool {
        if let Some(media) = self.media_files.get_mut(&id) {
            media.alt_text = Some(alt_text);
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
            uploaded_by: 1, // Admin user
            alt_text: Some("BananaBit CMS Logo".to_string()),
            url: "/uploads/bananabit-logo.png".to_string(),
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
        div { class: "media-library",
            style: "padding: 20px; max-width: 1200px; margin: 0 auto;",
            h2 { "Media Library" }
            
            div { 
                class: "media-upload-area",
                style: "border: 2px dashed #4a5568; border-radius: 8px; padding: 30px; text-align: center; margin-bottom: 30px; background: #2d3748; transition: border-color 0.3s ease;",
                h3 { "Upload New Media" }
                input {
                    r#type: "file",
                    multiple: true,
                    accept: "image/*,video/*,audio/*,.pdf,.doc,.docx",
                    onchange: move |event| {
                        // Handle file upload
                        println!("Files selected for upload");
                    }
                }
                p { 
                    style: "margin-top: 10px; color: #a0aec0; font-size: 14px;",
                    "Drag and drop files here or click to browse. Supported formats: Images, Videos, Audio, PDF, Documents"
                }
            }
            
            div { 
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 20px;",
                // Demo media items
                div { 
                    style: "background: #2d3748; border-radius: 8px; padding: 15px; border: 1px solid #4a5568; transition: transform 0.2s ease;",
                    img {
                        src: "/uploads/bananabit-logo.png",
                        alt: "BananaBit CMS Logo",
                        width: "150",
                        height: "150",
                        style: "object-fit: cover; border-radius: 8px; width: 100%;"
                    }
                    div { 
                        h4 { 
                            style: "margin: 10px 0 5px 0; font-size: 14px; color: #e2e8f0; word-break: break-all;",
                            "bananabit-logo.png" 
                        }
                        p { 
                            style: "margin: 5px 0; font-size: 12px; color: #a0aec0;",
                            "PNG Image • 15.4 KB" 
                        }
                        input {
                            r#type: "text",
                            placeholder: "Alt text...",
                            value: "BananaBit CMS Logo",
                            style: "width: 100%; padding: 5px; margin: 10px 0; border: 1px solid #4a5568; border-radius: 4px; background: #1a202c; color: #e2e8f0; font-size: 12px;"
                        }
                        div { 
                            style: "display: flex; gap: 5px; margin-top: 10px;",
                            button { 
                                style: "padding: 5px 10px; border: 1px solid #4a5568; border-radius: 4px; background: #4a5568; color: #e2e8f0; cursor: pointer; font-size: 12px;",
                                "Edit" 
                            }
                            button { 
                                style: "padding: 5px 10px; border: 1px solid #e53e3e; border-radius: 4px; background: #e53e3e; color: #e2e8f0; cursor: pointer; font-size: 12px;",
                                "Delete" 
                            }
                        }
                    }
                }
                
                div { 
                    style: "border: 2px dashed #4a5568; border-radius: 8px; display: flex; align-items: center; justify-content: center; min-height: 200px; cursor: pointer;",
                    div { 
                        style: "text-align: center; color: #a0aec0;",
                        span { 
                            style: "font-size: 48px; display: block; margin-bottom: 10px;",
                            "+" 
                        }
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
            style: "background: #2d3748; border: 1px solid #4a5568; border-radius: 8px; padding: 20px; max-height: 400px; overflow-y: auto;",
            h3 { "Select Media" }
            
            div { 
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 15px; margin-top: 15px;",
                div { 
                    style: "text-align: center; padding: 10px; border: 1px solid #4a5568; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| {
                        let sample_media = MediaFile {
                            id: 1,
                            filename: "bananabit-logo.png".to_string(),
                            original_name: "logo.png".to_string(),
                            mime_type: "image/png".to_string(),
                            file_size: 15432,
                            uploaded_at: now_iso8601(),
                            uploaded_by: 1,
                            alt_text: Some("BananaBit CMS Logo".to_string()),
                            url: "/uploads/bananabit-logo.png".to_string(),
                        };
                        on_select.call(sample_media);
                    },
                    img {
                        src: "/uploads/bananabit-logo.png",
                        alt: "BananaBit CMS Logo",
                        width: "100",
                        height: "100",
                        style: "object-fit: cover; border-radius: 4px;"
                    }
                    p { 
                        style: "margin: 10px 0 0 0; font-size: 12px; color: #a0aec0; word-break: break-all;",
                        "bananabit-logo.png" 
                    }
                }
            }
        }
    }
}