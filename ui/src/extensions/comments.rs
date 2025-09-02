use dioxus::prelude::*;
use super::{Extension, ExtensionComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comment data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub post_id: u32,
    pub author: String,
    pub email: String,
    pub content: String,
    pub created_at: String,
    pub approved: bool,
    pub parent_id: Option<u32>, // For threaded comments
}

/// Comments extension - handles comment system
pub struct CommentsExtension {
    comments: HashMap<u32, Comment>,
    post_comments: HashMap<u32, Vec<u32>>, // post_id -> comment_ids
    next_id: u32,
}

impl CommentsExtension {
    pub fn new() -> Self {
        Self {
            comments: HashMap::new(),
            post_comments: HashMap::new(),
            next_id: 1,
        }
    }
    
    pub fn add_comment(&mut self, mut comment: Comment) -> u32 {
        let comment_id = self.next_id;
        comment.id = comment_id;
        self.next_id += 1;
        
        // Add to post comments list
        self.post_comments
            .entry(comment.post_id)
            .or_insert_with(Vec::new)
            .push(comment_id);
        
        self.comments.insert(comment_id, comment);
        comment_id
    }
    
    pub fn get_comments_for_post(&self, post_id: u32) -> Vec<&Comment> {
        self.post_comments
            .get(&post_id)
            .map(|comment_ids| {
                comment_ids
                    .iter()
                    .filter_map(|id| self.comments.get(id))
                    .filter(|comment| comment.approved)
                    .collect()
            })
            .unwrap_or_default()
    }
    
    pub fn approve_comment(&mut self, comment_id: u32) -> bool {
        if let Some(comment) = self.comments.get_mut(&comment_id) {
            comment.approved = true;
            true
        } else {
            false
        }
    }
}

impl Extension for CommentsExtension {
    fn id(&self) -> &'static str {
        "core.comments"
    }
    
    fn name(&self) -> &'static str {
        "Comments System"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add some sample comments
        let sample_comment1 = Comment {
            id: 0, // Will be overridden
            post_id: 0, // First blog post
            author: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            content: "Great post! I love the extension architecture approach. It makes the CMS very flexible.".to_string(),
            created_at: "2024-01-01T10:00:00Z".to_string(),
            approved: true,
            parent_id: None,
        };
        
        let sample_comment2 = Comment {
            id: 0, // Will be overridden
            post_id: 0, // First blog post
            author: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            content: "I agree! Looking forward to seeing how this develops.".to_string(),
            created_at: "2024-01-01T12:00:00Z".to_string(),
            approved: true,
            parent_id: None,
        };
        
        self.add_comment(sample_comment1);
        self.add_comment(sample_comment2);
        
        Ok(())
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "CommentSection".to_string(),
                description: "Comment section for posts".to_string(),
            },
            ExtensionComponent {
                name: "CommentForm".to_string(),
                description: "Form for submitting comments".to_string(),
            },
        ]
    }
}

#[component]
pub fn CommentSection(post_id: u32) -> Element {
    rsx! {
        div {
            class: "comment-section",
            h3 { "Comments" }
            
            // Sample comments for demo
            div {
                class: "comments-list",
                
                div {
                    class: "comment",
                    div {
                        class: "comment-header",
                        strong { "John Doe" }
                        span { class: "comment-date", " • January 1, 2024" }
                    }
                    div {
                        class: "comment-content",
                        p { "Great post! I love the extension architecture approach. It makes the CMS very flexible." }
                    }
                }
                
                div {
                    class: "comment",
                    div {
                        class: "comment-header",
                        strong { "Jane Smith" }
                        span { class: "comment-date", " • January 1, 2024" }
                    }
                    div {
                        class: "comment-content",
                        p { "I agree! Looking forward to seeing how this develops." }
                    }
                }
            }
            
            CommentForm { post_id }
        }
    }
}

#[component]
pub fn CommentForm(post_id: u32) -> Element {
    let mut author = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    let mut submitted = use_signal(|| false);
    
    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        if !author().is_empty() && !email().is_empty() && !content().is_empty() {
            // In a real implementation, this would submit to the backend
            submitted.set(true);
            
            // Clear form after a delay (simulated)
            let mut reset_form = move || {
                author.set(String::new());
                email.set(String::new());
                content.set(String::new());
                submitted.set(false);
            };
            
            // In a real app, you'd use a proper async mechanism
            // For now, just immediately reset
            reset_form();
        }
    };
    
    rsx! {
        div {
            class: "comment-form",
            h4 { "Leave a Comment" }
            
            if submitted() {
                div {
                    class: "success-message",
                    p { "Thank you for your comment! It will be reviewed before being published." }
                }
            } else {
                form {
                    onsubmit: on_submit,
                    
                    div {
                        class: "form-group",
                        label { r#for: "author", "Name:" }
                        input {
                            r#type: "text",
                            id: "author",
                            value: "{author}",
                            oninput: move |e| author.set(e.value().clone()),
                            required: true
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { r#for: "email", "Email:" }
                        input {
                            r#type: "email",
                            id: "email",
                            value: "{email}",
                            oninput: move |e| email.set(e.value().clone()),
                            required: true
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { r#for: "content", "Comment:" }
                        textarea {
                            id: "content",
                            rows: "4",
                            value: "{content}",
                            oninput: move |e| content.set(e.value().clone()),
                            required: true
                        }
                    }
                    
                    div {
                        class: "form-group",
                        button {
                            r#type: "submit",
                            class: "submit-btn",
                            "Submit Comment"
                        }
                    }
                }
            }
        }
    }
}