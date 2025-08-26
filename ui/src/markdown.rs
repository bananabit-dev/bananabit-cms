use std::sync::OnceLock;
use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser, Tag, Event};
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use std::time;
use std::fs;
use std::path::Path;

/// Component for rendering markdown content safely.
/// 
/// This component takes markdown text and renders it as HTML, handling various
/// markdown elements like headings, paragraphs, code blocks, links, and images.
/// 
/// # Features
/// 
/// - Syntax highlighting for code blocks
/// - Image handling with optional base paths
/// - External link detection and special handling
/// - Safe rendering without using dangerous_inner_html
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// use ui::Markdown;
/// 
/// #[component]
/// fn App() -> Element {
///     let markdown = "# Hello, world!\n\nThis is **markdown**!";
///     
///     rsx! {
///         Markdown {
///             content: markdown.to_string(),
///             image_base_path: Some("/assets/images".to_string())
///         }
///     }
/// }
/// ```
#[component]
pub fn Markdown(
    content: Option<String>,
    #[props(optional)] image_base_path: Option<String>,
    #[props(optional)] id: Option<String>,
    #[props(optional)] file_path: Option<String>,
) -> Element {
    // Handle the content prop - if it's None, use empty string
    let content_str = content.unwrap_or_else(|| String::new());
    
    // If file_path is provided, read the file content and override the content prop
    let final_content = if let Some(path) = file_path {
        match fs::read_to_string(Path::new(&path)) {
            Ok(file_content) => file_content,
            Err(e) => {
                eprintln!("Error reading markdown file: {}", e);
                // Fallback to the provided content or empty string
                content_str.clone()
            }
        }
    } else {
        content_str.clone()
    };
    
    let options = Options::all();
    let parser = Parser::new_ext(&final_content, options);
    
    let mut events = Vec::new();
    for event in parser {
        events.push(event);
    }
    
    // Use the provided ID or generate a simple one based on current timestamp
    let markdown_id = id.unwrap_or_else(|| {
        format!("markdown-{}", time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis())
    });
    
    rsx! {
        div {
            class: "markdown-container",
            id: {markdown_id},
            {render_markdown_events(events, image_base_path)}
        }
    }
}


/// Get syntax highlighting components (SyntaxSet and ThemeSet)
fn get_syntax_highlighter() -> &'static (SyntaxSet, ThemeSet) {
    static HIGHLIGHTER: OnceLock<(SyntaxSet, ThemeSet)> = OnceLock::new();
    
    HIGHLIGHTER.get_or_init(|| {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        (syntax_set, theme_set)
    })
}

/// Safely highlight code using syntect and return colored lines
fn highlight_code_to_lines(code: &str, language: &str) -> Vec<Vec<(String, String)>> {
    // Get syntax set and theme
    let syntax_set = get_syntax_set();
    let theme_set = get_theme_set();
    
    // Map common language names to syntect tokens
    let language_token = match language.to_lowercase().as_str() {
        "js" | "javascript" => "js",
        "css" => "css",
        "html" => "html",
        "typescript" | "ts" => "typescript",
        "python" | "py" => "python",
        "ruby" | "rb" => "ruby",
        "rust" | "rs" => "rust",
        "go" => "go",
        "java" => "java",
        "c" => "c",
        "cpp" | "c++" => "cpp",
        "csharp" | "c#" => "cs",
        "php" => "php",
        "shell" | "bash" | "sh" => "bash",
        "yaml" | "yml" => "yaml",
        "json" => "json",
        "markdown" | "md" => "markdown",
        "sql" => "sql",
        _ => language,
    };
    
    // Get syntax reference for the language
    let syntax = syntax_set
        .find_syntax_by_token(language_token)
        .or_else(|| syntax_set.find_syntax_by_extension(language_token))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    
    // Try to get the Dracula theme first, then fall back to other themes
    let theme = theme_set.themes.get("Dracula")
        .or_else(|| theme_set.themes.get("base16-ocean.dark"))
        .or_else(|| theme_set.themes.get("InspiredGitHub"))
        .or_else(|| theme_set.themes.values().next())
        .expect("No themes available");
    
    // Create a syntax highlighter
    let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
    
    // Split the code into lines and highlight each line
    let mut highlighted_lines = Vec::new();
    
    for line in code.lines() {
        // Replace tabs with spaces for consistent display
        let line_with_spaces = line.replace("\t", "    ");
        
        match highlighter.highlight_line(&line_with_spaces, &syntax_set) {
            Ok(ranges) => {
                // Convert the highlighted ranges to (text, color_class) pairs
                let colored_segments: Vec<(String, String)> = ranges
                    .into_iter()
                    .map(|(style, text)| {
                        // Map the style to a CSS class name based on scope
                        let class_name = if style.font_style.contains(syntect::highlighting::FontStyle::ITALIC) {
                            "type".to_string()
                        } else {
                            match style.foreground {
                                // Dracula theme colors
                                syntect::highlighting::Color { r: 255, g: 121, b: 198, .. } => "keyword".to_string(),
                                syntect::highlighting::Color { r: 80, g: 250, b: 123, .. } => "function".to_string(),
                                syntect::highlighting::Color { r: 98, g: 114, b: 164, .. } => "comment".to_string(),
                                syntect::highlighting::Color { r: 241, g: 250, b: 140, .. } => "string".to_string(),
                                syntect::highlighting::Color { r: 189, g: 147, b: 249, .. } => "number".to_string(),
                                syntect::highlighting::Color { r: 139, g: 233, b: 253, .. } => "type".to_string(),
                                syntect::highlighting::Color { r: 248, g: 248, b: 242, .. } => "text".to_string(),
                                _ => {
                                    // Language-specific handling
                                    match language_token {
                                        "js" => {
                                            if text == "function" || text == "const" || text == "let" || text == "var" || text == "return" {
                                                "keyword".to_string()
                                            } else if text == "true" || text == "false" {
                                                "bool".to_string()
                                            } else if text.starts_with("\"") && text.ends_with("\"") || text.starts_with("'") && text.ends_with("'") {
                                                "string".to_string()
                                            } else if text.starts_with("<") && text.ends_with(">") {
                                                "type".to_string()
                                            } else if text.chars().all(|c| c.is_numeric() || c == '.' || c == '_') {
                                                "number".to_string()
                                            } else if text.starts_with("//") {
                                                "comment".to_string()
                                            } else if text.chars().next().map_or(false, |c| c.is_uppercase()) {
                                                "type".to_string()
                                            } else {
                                                "text".to_string()
                                            }
                                        },
                                        "html" => {
                                            if text.starts_with("<") && text.contains(">") {
                                                "keyword".to_string()
                                            } else if text.starts_with("\"") && text.ends_with("\"") {
                                                "string".to_string()
                                            } else if text.starts_with("<!--") || text.ends_with("-->") {
                                                "comment".to_string()
                                            } else {
                                                "text".to_string()
                                            }
                                        },
                                        "css" => {
                                            if text.ends_with(":") {
                                                "keyword".to_string()
                                            } else if text.starts_with(".") || text.starts_with("#") {
                                                "type".to_string()
                                            } else if text.ends_with("px") || text.ends_with("em") || text.ends_with("rem") || text.ends_with("%") {
                                                "number".to_string()
                                            } else if text.starts_with("#") && (text.len() == 4 || text.len() == 7) {
                                                "string".to_string()
                                            } else if text.starts_with("/*") || text.ends_with("*/") {
                                                "comment".to_string()
                                            } else {
                                                "text".to_string()
                                            }
                                        },
                                        // Special case for derive attributes in Rust
                                        _ => {
                                            if text.starts_with("#[derive") {
                                                "attribute".to_string()
                                            } else if text.starts_with("#[") || text.starts_with("@") {
                                                "attribute".to_string()
                                            } else if text == "true" || text == "false" {
                                                "bool".to_string()
                                            }
                                            // Fallback based on common syntax highlighting patterns
                                            else if text.starts_with("fn ") || text.starts_with("struct ") || text.starts_with("enum ") {
                                                "keyword".to_string()
                                            } else if text == "let" || text == "mut" || text == "const" || text == "return" {
                                                "keyword".to_string()
                                            } else if text.chars().all(|c| c.is_numeric() || c == '.' || c == '_') {
                                                "number".to_string()
                                            } else if text.starts_with("\"") && text.ends_with("\"") {
                                                "string".to_string()
                                            } else if text.starts_with("//") {
                                                "comment".to_string()
                                            } else if text.chars().next().map_or(false, |c| c.is_uppercase()) {
                                                "type".to_string()
                                            } else {
                                                "text".to_string()
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        
                        (text.to_string(), class_name)
                    })
                    .collect();
                
                highlighted_lines.push(colored_segments);
            },
            Err(_) => {
                // Fallback to plain text
                highlighted_lines.push(vec![(line_with_spaces.to_string(), "text".to_string())]);
            }
        }
    }
    
    highlighted_lines
}

/// Add line numbers to code and return a vector of line elements
fn add_line_numbers_elements(code: &str) -> Vec<Element> {
    let lines: Vec<&str> = code.lines().collect();
    let line_count = lines.len();
    let padding = line_count.to_string().len();
    
    lines.iter().enumerate()
        .map(|(i, line)| {
            let line_num = i + 1;
            let padded_num = format!("{:>width$}", line_num, width = padding);
            
            rsx! {
                div {
                    class: "code-line",
                    span {
                        class: "line-number",
                        aria_hidden: true,
                        tabindex: -1,
                        {padded_num}
                    }
                    span {
                        class: "line-content",
                        {line.replace("\t", "    ")}
                    }
                }
            }
        })
        .collect()
}

/// Render markdown events to Dioxus elements
fn render_markdown_events<'a>(events: Vec<Event<'a>>, image_base_path: Option<String>) -> impl Iterator<Item = Element> {
    let mut elements = Vec::new();
    let mut current_text = String::new();
    let mut list_stack = Vec::new();
    
    // Convert events to a slice to avoid moving it
    let events_slice = events.as_slice();
    
    // Track if we're inside a table to handle table structure properly
    let mut in_table_head = false;
    
    // Create an index to track our position in the events
    let mut i = 0;
    
    while i < events_slice.len() {
        match &events_slice[i] {
            Event::Start(tag) => {
                // Flush any accumulated text before processing a new tag
                if !current_text.is_empty() {
                    elements.push(rsx! { span { {current_text.clone()} } });
                    current_text.clear();
                }
                
                // Handle opening tags
                match tag {
                    Tag::Paragraph => {
                        // Collect all events until the matching End(Paragraph)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Paragraph);
                        elements.push(rsx! { p { class: "markdown-paragraph", {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                    },
                    Tag::Heading(level, _, _) => {
                        let class = format!("markdown-heading-{}", *level as u8);
                        // Collect all events until the matching End(Heading)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Heading(*level, None, Vec::new()));
                        match level {
                            pulldown_cmark::HeadingLevel::H1 => elements.push(rsx! { h1 { class: class, {render_markdown_events(content, image_base_path.clone())} } }),
                            pulldown_cmark::HeadingLevel::H2 => elements.push(rsx! { h2 { class: class, {render_markdown_events(content, image_base_path.clone())} } }),
                            pulldown_cmark::HeadingLevel::H3 => elements.push(rsx! { h3 { class: class, {render_markdown_events(content, image_base_path.clone())} } }),
                            pulldown_cmark::HeadingLevel::H4 => elements.push(rsx! { h4 { class: class, {render_markdown_events(content, image_base_path.clone())} } }),
                            pulldown_cmark::HeadingLevel::H5 => elements.push(rsx! { h5 { class: class, {render_markdown_events(content, image_base_path.clone())} } }),
                            pulldown_cmark::HeadingLevel::H6 => elements.push(rsx! { h6 { class: class, {render_markdown_events(content, image_base_path.clone())} } }),
                        }
                        i = new_index;
                    },
                    Tag::BlockQuote => {
                        // Collect all events until the matching End(BlockQuote)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::BlockQuote);
                        elements.push(rsx! { blockquote { class: "markdown-blockquote", {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                    },
                    Tag::CodeBlock(kind) => {
                        let language = match &kind {
                            pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                                // Normalize language name
                                let lang_str = lang.to_string();
                                if lang_str.is_empty() {
                                    "text".to_string()
                                } else {
                                    lang_str
                                }
                            },
                            _ => "text".to_string(),
                        };
                        
                        // Collect all events until the matching End(CodeBlock)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::CodeBlock(kind.clone()));
                        let code_content = collect_text_until_end(events_slice, Tag::CodeBlock(kind.clone()));
                        
                        // Check if code is small enough to not need scrolling
                        let lines = code_content.lines().count();
                        let max_line_length = code_content.lines().map(|line| line.len()).max().unwrap_or(0);
                        let needs_scroll = max_line_length > 80 || lines > 15;
                        
                        let scroll_class = if needs_scroll { "needs-scroll" } else { "no-scroll" };
                        
                        if !language.is_empty() {
                            // Use syntax highlighting with our new approach
                            let highlighted_lines = highlight_code_to_lines(&code_content, &language);
                            let line_count = highlighted_lines.len();
                            let padding = line_count.to_string().len();
                            
                            elements.push(rsx! {
                                div {
                                    class: "markdown-code-block language-{language} {scroll_class}",
                                    pre {
                                        code {
                                            class: "syntax-highlighted line-numbers",
                                            {{ highlighted_lines.iter().enumerate().map(|(i, segments)| {
                                                let line_num = i + 1;
                                                let padded_num = format!("{:>width$}", line_num, width = padding);
                                                
                                                rsx! {
                                                    div {
                                                        class: "code-line",
                                                        span {
                                                            class: "line-number",
                                                            aria_hidden: "true",
                                                            tabindex: "-1",
                                                            {padded_num}
                                                        }
                                                        span {
                                                            class: "line-content",
                                                            {{ segments.iter().map(|(text, class_name)| {
                                                                let class = format!("syntax-{}", class_name);
                                                                rsx! {
                                                                    span {
                                                                        class: {class},
                                                                        {text.clone()}
                                                                    }
                                                                }
                                                            }) }}
                                                        }
                                                    }
                                                }
                                            }) }}
                                        }
                                    }
                                }
                            });
                        } else {
                            // Plain code block without highlighting
                            let code_lines = add_line_numbers_elements(&code_content);
                            
                            elements.push(rsx! {
                                div {
                                    class: "markdown-code-block {scroll_class}",
                                    pre {
                                        code {
                                            class: "line-numbers",
                                            {code_lines.into_iter()}
                                        }
                                    }
                                }
                            });
                        }
                        i = new_index;
                    },
                    Tag::List(first_item_number) => {
                        // Check if this is a task list by looking ahead at the content
                        let is_task_list = if i + 2 < events_slice.len() {
                            match &events_slice[i + 1] {
                                Event::Start(Tag::Item) => {
                                    if i + 2 < events_slice.len() {
                                        match &events_slice[i + 2] {
                                            Event::Text(text) => {
                                                text.starts_with("[ ] ") || 
                                                text.starts_with("[x] ") || 
                                                text.starts_with("[X] ")
                                            },
                                            _ => false,
                                        }
                                    } else {
                                        false
                                    }
                                },
                                _ => false,
                            }
                        } else {
                            false
                        };
                        
                        list_stack.push(*first_item_number);
                        
                        match first_item_number {
                            Some(number) => {
                                // Collect all events until the matching End(List)
                                let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::List(*first_item_number));
                                let list_class = if is_task_list { "markdown-list markdown-task-list" } else { "markdown-list" };
                                elements.push(rsx! { ol { class: list_class, start: "{number}", {render_markdown_events(content, image_base_path.clone())} } });
                                i = new_index;
                            },
                            None => {
                                // Collect all events until the matching End(List)
                                let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::List(*first_item_number));
                                let list_class = if is_task_list { "markdown-list markdown-task-list" } else { "markdown-list" };
                                elements.push(rsx! { ul { class: list_class, {render_markdown_events(content, image_base_path.clone())} } });
                                i = new_index;
                            }
                        }
                    },
                    Tag::Item => {
                        // Check if this is a task list item by looking ahead at the content
                        let is_task_item = if i + 1 < events_slice.len() {
                            match &events_slice[i + 1] {
                                Event::Text(text) => {
                                    text.starts_with("[ ] ") || 
                                    text.starts_with("[x] ") || 
                                    text.starts_with("[X] ")
                                },
                                _ => false,
                            }
                        } else {
                            false
                        };
                        
                        if is_task_item {
                            // Collect all events until the matching End(Item)
                            let (mut content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Item);
                            
                            // Process the first text event to extract the checkbox
                            if !content.is_empty() {
                                if let Event::Text(text) = &content[0] {
                                    let text_str = text.to_string();
                                    let checked = text_str.starts_with("[x] ") || text_str.starts_with("[X] ");
                                    let remaining_text = text_str[4..].to_string();
                                    
                                    // Replace the first text event with our custom checkbox and the remaining text
                                    content[0] = Event::Text(remaining_text.into());
                                    
                                    let check_status = if checked { "checked" } else { "unchecked" };
                                    
                                    elements.push(rsx! { 
                                        li { 
                                            class: "markdown-task-list-item",
                                            div {
                                                class: "markdown-task-checkbox-container",
                                                div {
                                                    class: format!("markdown-task-checkbox markdown-task-checkbox-{}", if checked { "checked" } else { "unchecked" }),
                                                    role: "checkbox",
                                                    aria_checked: if checked { "true" } else { "false" },
                                                    tabindex: "0",
                                                }
                                                span {
                                                    class: "markdown-task-text",
                                                    {render_markdown_events(content, image_base_path.clone())}
                                                }
                                            }
                                        } 
                                    });
                                }
                            }
                            
                            i = new_index;
                        } else {
                            // Regular list item
                            // Collect all events until the matching End(Item)
                            let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Item);
                            elements.push(rsx! { li { class: "markdown-list-item", {render_markdown_events(content, image_base_path.clone())} } });
                            i = new_index;
                        }
                    },
                    Tag::FootnoteDefinition(_) => {
                        // Skip footnote definitions for now
                        i += 1;
                    },
                    Tag::Table(alignments) => {
                        // Collect all events until the matching End(Table)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Table(alignments.clone()));
                        elements.push(rsx! { table { class: "markdown-table", {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                    },
                    Tag::TableHead => {
                        in_table_head = true;
                        // Collect all events until the matching End(TableHead)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::TableHead);
                        elements.push(rsx! { thead { {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                        in_table_head = false;
                    },
                    Tag::TableRow => {
                        // Collect all events until the matching End(TableRow)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::TableRow);
                        elements.push(rsx! { tr { {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                    },
                    Tag::TableCell => {
                        // Determine if this is a header cell or a data cell
                        let cell_type = if in_table_head { "th" } else { "td" };
                        
                        // Collect all events until the matching End(TableCell)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::TableCell);
                        let cell_content = render_markdown_events(content, image_base_path.clone());
                        
                        if cell_type == "th" {
                            elements.push(rsx! { th { class: "markdown-table-header", {cell_content} } });
                        } else {
                            elements.push(rsx! { td { class: "markdown-table-cell", {cell_content} } });
                        }
                        i = new_index;
                    },
                    Tag::Emphasis => {
                        // Collect all events until the matching End(Emphasis)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Emphasis);
                        elements.push(rsx! { em { class: "markdown-emphasis", {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                    },
                    Tag::Strong => {
                        // Collect all events until the matching End(Strong)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Strong);
                        elements.push(rsx! { strong { class: "markdown-strong", {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                    },
                    Tag::Strikethrough => {
                        // Collect all events until the matching End(Strikethrough)
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Strikethrough);
                        elements.push(rsx! { del { class: "markdown-strikethrough", {render_markdown_events(content, image_base_path.clone())} } });
                        i = new_index;
                    },
                    Tag::Link(link_type, url, title) => {
                        let url_str = url.to_string();
                        let title_str = title.to_string();
                        let link_class = if url_str.starts_with("http://") || url_str.starts_with("https://") {
                            "markdown-link markdown-external-link"
                        } else {
                            "markdown-link"
                        };
                        
                        // Collect the content of the link before advancing the index
                        let (content, new_index) = collect_until_end_with_index(events_slice, i, Tag::Link(*link_type, url.clone(), title.clone()));
                        
                        let link = if url_str.starts_with("http://") || url_str.starts_with("https://") {
                            rsx! { a {
                                class: {link_class},
                                href: {url_str},
                                title: {title_str},
                                target: "_blank",
                                rel: "noopener noreferrer",
                                {render_markdown_events(content, image_base_path.clone())}
                            }}
                        } else {
                            rsx! { a {
                                class: {link_class},
                                href: {url_str},
                                title: {title_str},
                                {render_markdown_events(content, image_base_path.clone())}
                            }}
                        };
                        
                        elements.push(link);
                        i = new_index;
                    },
                    Tag::Image(link_type, url, title) => {
                        let mut url_str = url.to_string();
                        let title_str = title.to_string();
                        
                        // Handle image base path if provided
                        if let Some(base) = &image_base_path {
                            if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
                                // For relative paths, prepend the base path
                                if url_str.starts_with('/') {
                                    url_str = format!("{}{}", base, url_str);
                                } else {
                                    url_str = format!("{}/{}", base, url_str);
                                }
                            }
                        }
                        
                        let alt_text = collect_text_until_end(events_slice, Tag::Image(*link_type, url.clone(), title.clone()));
                        let alt_text_clone = alt_text.clone();
                        
                        elements.push(rsx! {
                            figure {
                                class: "markdown-image-container",
                                img {
                                    class: "markdown-image",
                                    src: "{url_str}",
                                    alt: "{alt_text_clone}",
                                    title: "{title_str}",
                                    loading: "lazy",
                                }
                                figcaption {
                                    class: "markdown-image-caption",
                                    {alt_text}
                                }
                            }
                        });
                        
                        // Skip past the end tag
                        let (_, new_index) = collect_until_end_with_index(events_slice, i, Tag::Image(*link_type, url.clone(), title.clone()));
                        i = new_index;
                    },
                }
            },
            Event::End(_) => {
                // End tags are handled by collect_until_end_with_index
                i += 1;
            },
            Event::Text(text) => {
                // Only add text if it's not empty after trimming
                let text_str = text.to_string();
                if !text_str.trim().is_empty() {
                    current_text.push_str(&text_str);
                }
                i += 1;
            },
            Event::Code(code) => {
                if !current_text.is_empty() {
                    elements.push(rsx! { span { {current_text.clone()} } });
                    current_text.clear();
                }
                
                elements.push(rsx! { code { class: "markdown-inline-code", {code.to_string()} } });
                i += 1;
            },
            Event::Html(html) => {
                current_text.push_str(&html);
                i += 1;
            },
            Event::FootnoteReference(reference) => {
                if !current_text.is_empty() {
                    elements.push(rsx! { span { {current_text.clone()} } });
                    current_text.clear();
                }
                
                elements.push(rsx! { sup { class: "markdown-footnote-ref", {format!("[{}]", reference)} } });
                i += 1;
            },
            Event::SoftBreak => {
                current_text.push(' ');
                i += 1;
            },
            Event::HardBreak => {
                if !current_text.is_empty() {
                    elements.push(rsx! { span { {current_text.clone()} } });
                    current_text.clear();
                }
                
                elements.push(rsx! { br {} });
                i += 1;
            },
            Event::Rule => {
                if !current_text.is_empty() {
                    elements.push(rsx! { span { {current_text.clone()} } });
                    current_text.clear();
                }
                
                elements.push(rsx! { hr { class: "markdown-thematic-break" } });
                i += 1;
            },
            Event::TaskListMarker(checked) => {
                if !current_text.is_empty() {
                    elements.push(rsx! { span { {current_text.clone()} } });
                    current_text.clear();
                }
                
                elements.push(rsx! { 
                    div {
                        class: format!("markdown-task-checkbox markdown-task-checkbox-{}", if *checked { "checked" } else { "unchecked" }),
                        role: "checkbox",
                        aria_checked: if *checked { "true" } else { "false" },
                        tabindex: "0",
                    }
                });
                i += 1;
            },
        }
    }
    
    // Flush any remaining text
    if !current_text.is_empty() {
        elements.push(rsx! { span { {current_text} } });
    }
    
    elements.into_iter()
}

/// Helper function to collect events until a matching end tag, returning the collected events and the new index
fn collect_until_end_with_index<'a>(events: &[Event<'a>], start_index: usize, start_tag: Tag<'a>) -> (Vec<Event<'a>>, usize) {
    let mut collected = Vec::new();
    let mut depth = 0;
    let mut i = start_index;
    
    while i < events.len() {
        match &events[i] {
            Event::Start(tag) if tag_matches(tag, &start_tag) => {
                depth += 1;
                if depth > 1 {
                    collected.push(events[i].clone());
                }
            },
            Event::End(tag) if tag_matches(tag, &start_tag) => {
                depth -= 1;
                if depth == 0 {
                    break;
                } else {
                    collected.push(events[i].clone());
                }
            },
            _ if depth > 0 => {
                collected.push(events[i].clone());
            },
            _ => {}
        }
        i += 1;
    }
    
    (collected, i)
}

/// Helper function to check if two tags match (ignoring alignment in tables)
fn tag_matches(a: &Tag, b: &Tag) -> bool {
    match (a, b) {
        (Tag::Paragraph, Tag::Paragraph) => true,
        (Tag::Heading(a_level, a_id, a_classes), Tag::Heading(b_level, b_id, b_classes)) => {
            a_level == b_level && a_id == b_id && a_classes == b_classes
        },
        (Tag::BlockQuote, Tag::BlockQuote) => true,
        (Tag::CodeBlock(a_kind), Tag::CodeBlock(b_kind)) => a_kind == b_kind,
        (Tag::List(a_num), Tag::List(b_num)) => a_num == b_num,
        (Tag::Item, Tag::Item) => true,
        (Tag::FootnoteDefinition(a_id), Tag::FootnoteDefinition(b_id)) => a_id == b_id,
        (Tag::Table(_), Tag::Table(_)) => true,
        (Tag::TableHead, Tag::TableHead) => true,
        (Tag::TableRow, Tag::TableRow) => true,
        (Tag::TableCell, Tag::TableCell) => true,
        (Tag::Emphasis, Tag::Emphasis) => true,
        (Tag::Strong, Tag::Strong) => true,
        (Tag::Strikethrough, Tag::Strikethrough) => true,
        (Tag::Link(a_type, a_url, a_title), Tag::Link(b_type, b_url, b_title)) => {
            a_type == b_type && a_url == b_url && a_title == b_title
        },
        (Tag::Image(a_type, a_url, a_title), Tag::Image(b_type, b_url, b_title)) => {
            a_type == b_type && a_url == b_url && a_title == b_title
        },
        _ => false,
    }
}

/// Helper function to collect events until a matching end tag
fn collect_until_end<'a>(events: &[Event<'a>], tag: Tag<'a>) -> Vec<Event<'a>> {
    let (collected, _) = collect_until_end_with_index(events, 0, tag);
    collected
}

/// Helper function to collect text until a matching end tag
fn collect_text_until_end<'a>(events: &[Event<'a>], start_tag: Tag<'a>) -> String {
    let mut text = String::new();
    let mut depth = 0;
    let mut i = 0;
    
    while i < events.len() {
        match &events[i] {
            Event::Start(tag) if tag_matches(tag, &start_tag) => {
                depth += 1;
            },
            Event::End(tag) if tag_matches(tag, &start_tag) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            },
            Event::Text(content) if depth > 0 => {
                text.push_str(content);
            },
            Event::Code(content) if depth > 0 => {
                text.push_str(content);
            },
            Event::SoftBreak if depth > 0 => {
                text.push('\n');
            },
            Event::HardBreak if depth > 0 => {
                text.push_str("\n\n");
            },
            _ => {}
        }
        i += 1;
    }
    
    text
}

fn get_syntax_set() -> &'static SyntaxSet {
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    
    SYNTAX_SET.get_or_init(|| {
        SyntaxSet::load_defaults_newlines()
    })
}

fn get_theme_set() -> &'static ThemeSet {
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
    
    THEME_SET.get_or_init(|| {
        ThemeSet::load_defaults()
    })
}
