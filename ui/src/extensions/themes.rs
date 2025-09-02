use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub css_content: String,
    pub active: bool,
}

/// Theme management extension
pub struct ThemeExtension {
    themes: HashMap<u32, Theme>,
    active_theme_id: Option<u32>,
    next_id: u32,
}

impl ThemeExtension {
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            active_theme_id: None,
            next_id: 1,
        }
    }
    
    pub fn get_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }
    
    pub fn get_active_theme(&self) -> Option<&Theme> {
        if let Some(id) = self.active_theme_id {
            self.themes.get(&id)
        } else {
            None
        }
    }
    
    pub fn add_theme(&mut self, mut theme: Theme) -> u32 {
        theme.id = self.next_id;
        self.themes.insert(self.next_id, theme);
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    pub fn activate_theme(&mut self, theme_id: u32) -> bool {
        if self.themes.contains_key(&theme_id) {
            // Deactivate current theme
            if let Some(current_id) = self.active_theme_id {
                if let Some(current_theme) = self.themes.get_mut(&current_id) {
                    current_theme.active = false;
                }
            }
            
            // Activate new theme
            if let Some(new_theme) = self.themes.get_mut(&theme_id) {
                new_theme.active = true;
                self.active_theme_id = Some(theme_id);
                return true;
            }
        }
        false
    }
    
    pub fn delete_theme(&mut self, theme_id: u32) -> Option<Theme> {
        if Some(theme_id) == self.active_theme_id {
            self.active_theme_id = None;
        }
        self.themes.remove(&theme_id)
    }
}

impl Extension for ThemeExtension {
    fn id(&self) -> &'static str {
        "core.themes"
    }
    
    fn name(&self) -> &'static str {
        "Theme System"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add default dark theme
        let dark_theme = Theme {
            id: 0,
            name: "Dark Professional".to_string(),
            description: "Professional dark theme with blue accents".to_string(),
            css_content: "/* Dark theme CSS variables would go here */".to_string(),
            active: true,
        };
        
        let theme_id = self.add_theme(dark_theme);
        self.activate_theme(theme_id);
        
        // Add light theme
        let light_theme = Theme {
            id: 0,
            name: "Light Professional".to_string(),
            description: "Clean light theme with subtle shadows".to_string(),
            css_content: "/* Light theme CSS variables would go here */".to_string(),
            active: false,
        };
        
        self.add_theme(light_theme);
        
        // Add colorful theme
        let colorful_theme = Theme {
            id: 0,
            name: "Vibrant Colors".to_string(),
            description: "Bright and colorful theme for creative sites".to_string(),
            css_content: "/* Vibrant theme CSS variables would go here */".to_string(),
            active: false,
        };
        
        self.add_theme(colorful_theme);
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            ExtensionRoute {
                path: "/admin/themes".to_string(),
                requires_auth: true,
                admin_only: true,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "ThemeManager".to_string(),
                description: "Manage and switch between themes".to_string(),
            },
            ExtensionComponent {
                name: "ThemeSelector".to_string(),
                description: "Quick theme switcher component".to_string(),
            },
        ]
    }
}

/// Theme manager component for admin
#[component]
pub fn ThemeManager() -> Element {
    let mut active_theme = use_signal(|| "Dark Professional".to_string());
    
    rsx! {
        div {
            h2 { "Theme Management" }
            p { "Customize the appearance of your CMS with different themes. Changes apply immediately." }
            
            div {
                h3 { "Current Theme" }
                div {
                    div {
                        h4 { "{active_theme}" }
                        span { "ACTIVE" }
                    }
                    div { "Professional dark theme with blue accents" }
                }
            }
            
            div {
                h3 { "Available Themes" }
                div {
                    div {
                        onclick: move |_| {
                            active_theme.set("Dark Professional".to_string());
                        },
                        div {
                            h4 { "Dark Professional" }
                            span { "ACTIVE" }
                        }
                        div { "Professional dark theme with blue accents" }
                        button { "Activate" }
                    }
                    
                    div {
                        onclick: move |_| {
                            active_theme.set("Light Professional".to_string());
                        },
                        div {
                            h4 { "Light Professional" }
                        }
                        div { "Clean light theme with subtle shadows" }
                        button { "Activate" }
                    }
                    
                    div {
                        onclick: move |_| {
                            active_theme.set("Vibrant Colors".to_string());
                        },
                        div {
                            h4 { "Vibrant Colors" }
                        }
                        div { "Bright and colorful theme for creative sites" }
                        button { "Activate" }
                    }
                }
            }
            
            div {
                h3 { "Theme Actions" }
                div {
                    button { "Create Custom Theme" }
                    button { "Import Theme" }
                    button { "Export Current Theme" }
                }
            }
        }
    }
}

/// Quick theme selector component
#[component]
pub fn ThemeSelector() -> Element {
    let mut current_theme = use_signal(|| "Dark Professional".to_string());
    let mut show_dropdown = use_signal(|| false);
    
    rsx! {
        div {
            button {
                onclick: move |_| {
                    show_dropdown.set(!show_dropdown());
                },
                span { "🎨 {current_theme}" }
                span { if show_dropdown() { "▲" } else { "▼" } }
            }
            
            if show_dropdown() {
                div {
                    div {
                        onclick: move |_| {
                            current_theme.set("Dark Professional".to_string());
                            show_dropdown.set(false);
                        },
                        "🌙 Dark Professional"
                    }
                    div {
                        onclick: move |_| {
                            current_theme.set("Light Professional".to_string());
                            show_dropdown.set(false);
                        },
                        "☀️ Light Professional"
                    }
                    div {
                        onclick: move |_| {
                            current_theme.set("Vibrant Colors".to_string());
                            show_dropdown.set(false);
                        },
                        "🌈 Vibrant Colors"
                    }
                }
            }
        }
    }
}