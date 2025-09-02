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
            css_content: r#"
                :root {
                    --bg-primary: #1a202c;
                    --bg-secondary: #2d3748;
                    --bg-tertiary: #4a5568;
                    --text-primary: #e2e8f0;
                    --text-secondary: #a0aec0;
                    --text-muted: #718096;
                    --accent-primary: #63b3ed;
                    --accent-secondary: #4299e1;
                    --border-color: #4a5568;
                    --success-color: #38a169;
                    --warning-color: #d69e2e;
                    --error-color: #e53e3e;
                }
                
                body {
                    background-color: var(--bg-primary);
                    color: var(--text-primary);
                    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
                }
                
                .navbar {
                    background-color: var(--bg-secondary);
                    border-bottom: 1px solid var(--border-color);
                }
                
                .btn-primary {
                    background-color: var(--accent-primary);
                    color: #ffffff;
                    border: 1px solid var(--accent-primary);
                }
                
                .btn-primary:hover {
                    background-color: var(--accent-secondary);
                    border-color: var(--accent-secondary);
                }
            "#.to_string(),
            active: true,
        };
        
        let theme_id = self.add_theme(dark_theme);
        self.activate_theme(theme_id);
        
        // Add light theme
        let light_theme = Theme {
            id: 0,
            name: "Light Professional".to_string(),
            description: "Clean light theme with subtle shadows".to_string(),
            css_content: r#"
                :root {
                    --bg-primary: #ffffff;
                    --bg-secondary: #f7fafc;
                    --bg-tertiary: #edf2f7;
                    --text-primary: #2d3748;
                    --text-secondary: #4a5568;
                    --text-muted: #718096;
                    --accent-primary: #3182ce;
                    --accent-secondary: #2c5282;
                    --border-color: #e2e8f0;
                    --success-color: #38a169;
                    --warning-color: #d69e2e;
                    --error-color: #e53e3e;
                }
                
                body {
                    background-color: var(--bg-primary);
                    color: var(--text-primary);
                    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
                }
                
                .navbar {
                    background-color: var(--bg-secondary);
                    border-bottom: 1px solid var(--border-color);
                    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
                }
                
                .btn-primary {
                    background-color: var(--accent-primary);
                    color: #ffffff;
                    border: 1px solid var(--accent-primary);
                }
                
                .btn-primary:hover {
                    background-color: var(--accent-secondary);
                    border-color: var(--accent-secondary);
                }
            "#.to_string(),
            active: false,
        };
        
        self.add_theme(light_theme);
        
        // Add colorful theme
        let colorful_theme = Theme {
            id: 0,
            name: "Vibrant Colors".to_string(),
            description: "Bright and colorful theme for creative sites".to_string(),
            css_content: r#"
                :root {
                    --bg-primary: #0f0f23;
                    --bg-secondary: #1a1a3a;
                    --bg-tertiary: #2d2d5f;
                    --text-primary: #ccccff;
                    --text-secondary: #9999cc;
                    --text-muted: #666699;
                    --accent-primary: #ff6b9d;
                    --accent-secondary: #ff8cc8;
                    --border-color: #444466;
                    --success-color: #51cf66;
                    --warning-color: #ffd43b;
                    --error-color: #ff6b6b;
                }
                
                body {
                    background: linear-gradient(135deg, var(--bg-primary) 0%, #1a1a3a 100%);
                    color: var(--text-primary);
                    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
                }
                
                .navbar {
                    background: linear-gradient(90deg, var(--bg-secondary) 0%, #2d2d5f 100%);
                    border-bottom: 1px solid var(--border-color);
                }
                
                .btn-primary {
                    background: linear-gradient(45deg, var(--accent-primary) 0%, #c084fc 100%);
                    color: #ffffff;
                    border: 1px solid var(--accent-primary);
                }
                
                .btn-primary:hover {
                    background: linear-gradient(45deg, var(--accent-secondary) 0%, #d8b4fe 100%);
                }
            "#.to_string(),
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
    let active_theme = use_signal(|| "Dark Professional".to_string());
    
    rsx! {
        div { class: "theme-manager",
            h2 { "Theme Management" }
            p { class: "description",
                "Customize the appearance of your CMS with different themes. Changes apply immediately."
            }
            
            div { class: "current-theme",
                h3 { "Current Theme" }
                div { class: "theme-preview active",
                    div { class: "preview-header",
                        h4 { "{active_theme}" }
                        span { class: "badge active", "ACTIVE" }
                    }
                    div { class: "preview-description",
                        "Professional dark theme with blue accents"
                    }
                }
            }
            
            div { class: "available-themes",
                h3 { "Available Themes" }
                div { class: "themes-grid",
                    div { 
                        class: "theme-preview",
                        onclick: move |_| {
                            active_theme.set("Dark Professional".to_string());
                        },
                        div { class: "preview-header",
                            h4 { "Dark Professional" }
                            span { class: "badge active", "ACTIVE" }
                        }
                        div { class: "theme-colors",
                            div { class: "color-swatch", style: "background: #1a202c;" }
                            div { class: "color-swatch", style: "background: #2d3748;" }
                            div { class: "color-swatch", style: "background: #63b3ed;" }
                        }
                        div { class: "preview-description",
                            "Professional dark theme with blue accents"
                        }
                        button { class: "btn btn-primary", "Activate" }
                    }
                    
                    div { 
                        class: "theme-preview",
                        onclick: move |_| {
                            active_theme.set("Light Professional".to_string());
                        },
                        div { class: "preview-header",
                            h4 { "Light Professional" }
                        }
                        div { class: "theme-colors",
                            div { class: "color-swatch", style: "background: #ffffff;" }
                            div { class: "color-swatch", style: "background: #f7fafc;" }
                            div { class: "color-swatch", style: "background: #3182ce;" }
                        }
                        div { class: "preview-description",
                            "Clean light theme with subtle shadows"
                        }
                        button { class: "btn btn-outline", "Activate" }
                    }
                    
                    div { 
                        class: "theme-preview",
                        onclick: move |_| {
                            active_theme.set("Vibrant Colors".to_string());
                        },
                        div { class: "preview-header",
                            h4 { "Vibrant Colors" }
                        }
                        div { class: "theme-colors",
                            div { class: "color-swatch", style: "background: #0f0f23;" }
                            div { class: "color-swatch", style: "background: #1a1a3a;" }
                            div { class: "color-swatch", style: "background: #ff6b9d;" }
                        }
                        div { class: "preview-description",
                            "Bright and colorful theme for creative sites"
                        }
                        button { class: "btn btn-outline", "Activate" }
                    }
                }
            }
            
            div { class: "theme-actions",
                h3 { "Theme Actions" }
                div { class: "action-buttons",
                    button { class: "btn btn-outline",
                        "Create Custom Theme"
                    }
                    button { class: "btn btn-outline",
                        "Import Theme"
                    }
                    button { class: "btn btn-outline",
                        "Export Current Theme"
                    }
                }
            }
        }
        
        style { r#"
            .theme-manager {
                padding: 20px;
                max-width: 1200px;
                margin: 0 auto;
            }
            
            .description {
                color: var(--text-secondary, #a0aec0);
                margin-bottom: 30px;
            }
            
            .current-theme {
                margin-bottom: 40px;
            }
            
            .theme-preview {
                background: var(--bg-secondary, #2d3748);
                border: 1px solid var(--border-color, #4a5568);
                border-radius: 8px;
                padding: 20px;
                cursor: pointer;
                transition: all 0.2s ease;
            }
            
            .theme-preview:hover {
                transform: translateY(-2px);
                border-color: var(--accent-primary, #63b3ed);
            }
            
            .theme-preview.active {
                border-color: var(--accent-primary, #63b3ed);
                box-shadow: 0 0 0 2px rgba(99, 179, 237, 0.2);
            }
            
            .preview-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 15px;
            }
            
            .preview-header h4 {
                margin: 0;
                color: var(--text-primary, #e2e8f0);
            }
            
            .badge {
                padding: 4px 8px;
                border-radius: 4px;
                font-size: 12px;
                font-weight: 600;
                text-transform: uppercase;
            }
            
            .badge.active {
                background: var(--accent-primary, #63b3ed);
                color: #ffffff;
            }
            
            .theme-colors {
                display: flex;
                gap: 8px;
                margin-bottom: 15px;
            }
            
            .color-swatch {
                width: 30px;
                height: 30px;
                border-radius: 6px;
                border: 1px solid var(--border-color, #4a5568);
            }
            
            .preview-description {
                color: var(--text-secondary, #a0aec0);
                font-size: 14px;
                margin-bottom: 15px;
            }
            
            .themes-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                gap: 20px;
                margin-bottom: 40px;
            }
            
            .action-buttons {
                display: flex;
                gap: 15px;
                flex-wrap: wrap;
            }
            
            .btn {
                padding: 10px 20px;
                border-radius: 6px;
                border: 1px solid;
                cursor: pointer;
                font-weight: 500;
                transition: all 0.2s ease;
                text-decoration: none;
                display: inline-block;
            }
            
            .btn-primary {
                background: var(--accent-primary, #63b3ed);
                color: #ffffff;
                border-color: var(--accent-primary, #63b3ed);
            }
            
            .btn-primary:hover {
                background: var(--accent-secondary, #4299e1);
                border-color: var(--accent-secondary, #4299e1);
            }
            
            .btn-outline {
                background: transparent;
                color: var(--accent-primary, #63b3ed);
                border-color: var(--accent-primary, #63b3ed);
            }
            
            .btn-outline:hover {
                background: var(--accent-primary, #63b3ed);
                color: #ffffff;
            }
        "# }
    }
}

/// Quick theme selector component
#[component]
pub fn ThemeSelector() -> Element {
    let current_theme = use_signal(|| "Dark Professional".to_string());
    let show_dropdown = use_signal(|| false);
    
    rsx! {
        div { class: "theme-selector",
            button { 
                class: "theme-toggle",
                onclick: move |_| {
                    show_dropdown.set(!show_dropdown());
                },
                span { "🎨 {current_theme}" }
                span { class: "arrow", if show_dropdown() { "▲" } else { "▼" } }
            }
            
            if show_dropdown() {
                div { class: "theme-dropdown",
                    div { 
                        class: "theme-option active",
                        onclick: move |_| {
                            current_theme.set("Dark Professional".to_string());
                            show_dropdown.set(false);
                        },
                        "🌙 Dark Professional"
                    }
                    div { 
                        class: "theme-option",
                        onclick: move |_| {
                            current_theme.set("Light Professional".to_string());
                            show_dropdown.set(false);
                        },
                        "☀️ Light Professional"
                    }
                    div { 
                        class: "theme-option",
                        onclick: move |_| {
                            current_theme.set("Vibrant Colors".to_string());
                            show_dropdown.set(false);
                        },
                        "🌈 Vibrant Colors"
                    }
                }
            }
        }
        
        style { r#"
            .theme-selector {
                position: relative;
                display: inline-block;
            }
            
            .theme-toggle {
                background: var(--bg-secondary, #2d3748);
                border: 1px solid var(--border-color, #4a5568);
                color: var(--text-primary, #e2e8f0);
                padding: 8px 12px;
                border-radius: 6px;
                cursor: pointer;
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 14px;
                transition: border-color 0.2s ease;
            }
            
            .theme-toggle:hover {
                border-color: var(--accent-primary, #63b3ed);
            }
            
            .arrow {
                font-size: 12px;
                transition: transform 0.2s ease;
            }
            
            .theme-dropdown {
                position: absolute;
                top: 100%;
                right: 0;
                background: var(--bg-secondary, #2d3748);
                border: 1px solid var(--border-color, #4a5568);
                border-radius: 6px;
                padding: 8px 0;
                min-width: 180px;
                z-index: 1000;
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            }
            
            .theme-option {
                padding: 8px 16px;
                cursor: pointer;
                color: var(--text-primary, #e2e8f0);
                transition: background-color 0.2s ease;
                font-size: 14px;
            }
            
            .theme-option:hover {
                background: var(--bg-tertiary, #4a5568);
            }
            
            .theme-option.active {
                background: var(--accent-primary, #63b3ed);
                color: #ffffff;
            }
        "# }
    }
}