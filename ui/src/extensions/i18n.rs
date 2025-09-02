use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Language definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub code: String,        // ISO 639-1 code (e.g., "en", "es", "fr")
    pub name: String,        // Display name (e.g., "English", "Español")
    pub native_name: String, // Native name (e.g., "English", "Español")
    pub direction: TextDirection,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

/// Translation entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub key: String,
    pub language_code: String,
    pub value: String,
    pub namespace: String, // e.g., "common", "posts", "admin"
}

/// Multi-language support extension
pub struct I18nExtension {
    languages: HashMap<String, Language>,
    translations: HashMap<String, HashMap<String, String>>, // lang_code -> (key -> value)
    default_language: String,
    current_language: String,
}

impl I18nExtension {
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
            translations: HashMap::new(),
            default_language: "en".to_string(),
            current_language: "en".to_string(),
        }
    }
    
    pub fn add_language(&mut self, language: Language) {
        self.languages.insert(language.code.clone(), language);
    }
    
    pub fn add_translation(&mut self, translation: Translation) {
        let lang_map = self.translations
            .entry(translation.language_code.clone())
            .or_insert_with(HashMap::new);
        lang_map.insert(translation.key.clone(), translation.value);
    }
    
    pub fn get_translation(&self, key: &str, lang_code: Option<&str>) -> String {
        let language = lang_code.unwrap_or(&self.current_language);
        
        if let Some(lang_map) = self.translations.get(language) {
            if let Some(translation) = lang_map.get(key) {
                return translation.clone();
            }
        }
        
        // Fallback to default language
        if language != self.default_language {
            if let Some(lang_map) = self.translations.get(&self.default_language) {
                if let Some(translation) = lang_map.get(key) {
                    return translation.clone();
                }
            }
        }
        
        // Return key if no translation found
        key.to_string()
    }
    
    pub fn set_current_language(&mut self, lang_code: &str) {
        if self.languages.contains_key(lang_code) {
            self.current_language = lang_code.to_string();
        }
    }
    
    pub fn get_available_languages(&self) -> Vec<&Language> {
        self.languages.values().filter(|lang| lang.active).collect()
    }
}

impl Extension for I18nExtension {
    fn id(&self) -> &'static str {
        "core.i18n"
    }
    
    fn name(&self) -> &'static str {
        "Multi-language Support"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add default languages
        self.add_language(Language {
            code: "en".to_string(),
            name: "English".to_string(),
            native_name: "English".to_string(),
            direction: TextDirection::LeftToRight,
            active: true,
        });
        
        self.add_language(Language {
            code: "es".to_string(),
            name: "Spanish".to_string(),
            native_name: "Español".to_string(),
            direction: TextDirection::LeftToRight,
            active: true,
        });
        
        self.add_language(Language {
            code: "fr".to_string(),
            name: "French".to_string(),
            native_name: "Français".to_string(),
            direction: TextDirection::LeftToRight,
            active: true,
        });
        
        // Add common translations
        let common_translations = vec![
            ("home", "en", "Home"),
            ("home", "es", "Inicio"),
            ("home", "fr", "Accueil"),
            ("blog", "en", "Blog"),
            ("blog", "es", "Blog"),
            ("blog", "fr", "Blog"),
            ("admin", "en", "Admin"),
            ("admin", "es", "Administración"),
            ("admin", "fr", "Administration"),
            ("settings", "en", "Settings"),
            ("settings", "es", "Configuración"),
            ("settings", "fr", "Paramètres"),
            ("save", "en", "Save"),
            ("save", "es", "Guardar"),
            ("save", "fr", "Enregistrer"),
            ("cancel", "en", "Cancel"),
            ("cancel", "es", "Cancelar"),
            ("cancel", "fr", "Annuler"),
            ("search", "en", "Search"),
            ("search", "es", "Buscar"),
            ("search", "fr", "Rechercher"),
        ];
        
        for (key, lang, value) in common_translations {
            self.add_translation(Translation {
                key: key.to_string(),
                language_code: lang.to_string(),
                value: value.to_string(),
                namespace: "common".to_string(),
            });
        }
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            ExtensionRoute {
                path: "/admin/i18n".to_string(),
                requires_auth: true,
                admin_only: true,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "LanguageManager".to_string(),
                description: "Manage languages and translations".to_string(),
            },
            ExtensionComponent {
                name: "LanguageSelector".to_string(),
                description: "Language switcher component".to_string(),
            },
            ExtensionComponent {
                name: "TranslationEditor".to_string(),
                description: "Edit translations for different languages".to_string(),
            },
        ]
    }
}

/// Language manager component
#[component]
pub fn LanguageManager() -> Element {
    let mut active_tab = use_signal(|| "languages".to_string());
    
    rsx! {
        div {
            h2 { "Multi-language Support" }
            p { "Manage languages and translations for your CMS. Create a truly global experience." }
            
            div {
                div {
                    button {
                        onclick: move |_| active_tab.set("languages".to_string()),
                        "Languages"
                    }
                    button {
                        onclick: move |_| active_tab.set("translations".to_string()),
                        "Translations"
                    }
                    button {
                        onclick: move |_| active_tab.set("import".to_string()),
                        "Import/Export"
                    }
                }
                
                div {
                    if active_tab() == "languages" {
                        div {
                            h3 { "Available Languages" }
                            
                            div {
                                div {
                                    h4 { "🇺🇸 English" }
                                    p { "English - Default Language" }
                                    span { "ACTIVE" }
                                    div {
                                        button { "Edit" }
                                        button { "Set as Default" }
                                    }
                                }
                                
                                div {
                                    h4 { "🇪🇸 Español" }
                                    p { "Spanish - 95% translated" }
                                    span { "ACTIVE" }
                                    div {
                                        button { "Edit" }
                                        button { "Deactivate" }
                                    }
                                }
                                
                                div {
                                    h4 { "🇫🇷 Français" }
                                    p { "French - 78% translated" }
                                    span { "ACTIVE" }
                                    div {
                                        button { "Edit" }
                                        button { "Deactivate" }
                                    }
                                }
                                
                                div {
                                    span { "+" }
                                    p { "Add Language" }
                                }
                            }
                        }
                    }
                    
                    if active_tab() == "translations" {
                        div {
                            h3 { "Translation Editor" }
                            
                            div {
                                select {
                                    option { "Common" }
                                    option { "Posts" }
                                    option { "Admin" }
                                    option { "Navigation" }
                                }
                                
                                input {
                                    r#type: "text",
                                    placeholder: "Search translations..."
                                }
                            }
                            
                            div {
                                div {
                                    div { "Key" }
                                    div { "English" }
                                    div { "Spanish" }
                                    div { "French" }
                                    div { "Actions" }
                                }
                                
                                div {
                                    div { "home" }
                                    div { 
                                        input { r#type: "text", value: "Home" }
                                    }
                                    div { 
                                        input { r#type: "text", value: "Inicio" }
                                    }
                                    div { 
                                        input { r#type: "text", value: "Accueil" }
                                    }
                                    div {
                                        button { "Save" }
                                        button { "Delete" }
                                    }
                                }
                                
                                div {
                                    div { "blog" }
                                    div { 
                                        input { r#type: "text", value: "Blog" }
                                    }
                                    div { 
                                        input { r#type: "text", value: "Blog" }
                                    }
                                    div { 
                                        input { r#type: "text", value: "Blog" }
                                    }
                                    div {
                                        button { "Save" }
                                        button { "Delete" }
                                    }
                                }
                            }
                            
                            div {
                                button { "Add Translation Key" }
                                button { "Bulk Import" }
                                button { "Generate Missing Keys" }
                            }
                        }
                    }
                    
                    if active_tab() == "import" {
                        div {
                            h3 { "Import & Export Translations" }
                            
                            div {
                                h4 { "Export Translations" }
                                p { "Download translations in various formats for external editing." }
                                
                                div {
                                    select {
                                        option { "JSON" }
                                        option { "CSV" }
                                        option { "YAML" }
                                        option { "PO/POT" }
                                    }
                                    
                                    select {
                                        option { "All Languages" }
                                        option { "English" }
                                        option { "Spanish" }
                                        option { "French" }
                                    }
                                    
                                    button { "Export" }
                                }
                            }
                            
                            div {
                                h4 { "Import Translations" }
                                p { "Upload translation files to bulk update your content." }
                                
                                div {
                                    input {
                                        r#type: "file",
                                        accept: ".json,.csv,.yaml,.yml,.po,.pot"
                                    }
                                    button { "Upload" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Language selector component
#[component]
pub fn LanguageSelector() -> Element {
    let mut current_language = use_signal(|| "en".to_string());
    let mut show_dropdown = use_signal(|| false);
    
    rsx! {
        div {
            button {
                onclick: move |_| {
                    show_dropdown.set(!show_dropdown());
                },
                span { "🌍 " }
                span { 
                    match current_language().as_str() {
                        "en" => "English",
                        "es" => "Español", 
                        "fr" => "Français",
                        _ => "Unknown"
                    }
                }
                span { if show_dropdown() { "▲" } else { "▼" } }
            }
            
            if show_dropdown() {
                div {
                    div {
                        onclick: move |_| {
                            current_language.set("en".to_string());
                            show_dropdown.set(false);
                        },
                        "🇺🇸 English"
                    }
                    div {
                        onclick: move |_| {
                            current_language.set("es".to_string());
                            show_dropdown.set(false);
                        },
                        "🇪🇸 Español"
                    }
                    div {
                        onclick: move |_| {
                            current_language.set("fr".to_string());
                            show_dropdown.set(false);
                        },
                        "🇫🇷 Français"
                    }
                }
            }
        }
    }
}