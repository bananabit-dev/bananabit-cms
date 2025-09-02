use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use client::time::now_iso8601;

/// Scheduled content entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledContent {
    pub id: u32,
    pub content_type: ContentType,
    pub content_id: u32,
    pub scheduled_at: String, // ISO 8601 timestamp
    pub action: ScheduledAction,
    pub status: ScheduleStatus,
    pub created_at: String,
    pub created_by: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Post,
    Page,
    Media,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduledAction {
    Publish,
    Unpublish,
    Delete,
    Update,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Content scheduling extension
pub struct SchedulingExtension {
    scheduled_items: HashMap<u32, ScheduledContent>,
    next_id: u32,
}

impl SchedulingExtension {
    pub fn new() -> Self {
        Self {
            scheduled_items: HashMap::new(),
            next_id: 1,
        }
    }
    
    pub fn schedule_content(&mut self, mut item: ScheduledContent) -> u32 {
        item.id = self.next_id;
        item.status = ScheduleStatus::Pending;
        self.scheduled_items.insert(self.next_id, item);
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    pub fn get_scheduled_items(&self) -> Vec<&ScheduledContent> {
        self.scheduled_items.values().collect()
    }
    
    pub fn get_pending_items(&self) -> Vec<&ScheduledContent> {
        self.scheduled_items
            .values()
            .filter(|item| matches!(item.status, ScheduleStatus::Pending))
            .collect()
    }
    
    pub fn cancel_scheduled_item(&mut self, id: u32) -> Option<ScheduledContent> {
        self.scheduled_items.remove(&id)
    }
    
    pub fn update_item_status(&mut self, id: u32, status: ScheduleStatus) -> bool {
        if let Some(item) = self.scheduled_items.get_mut(&id) {
            item.status = status;
            true
        } else {
            false
        }
    }
    
    /// Process pending scheduled items (would be called by a background task)
    pub fn process_pending_items(&mut self) -> Vec<u32> {
        let now = now_iso8601();
        let mut processed_ids = Vec::new();
        
        for (id, item) in &mut self.scheduled_items {
            if matches!(item.status, ScheduleStatus::Pending) && item.scheduled_at <= now {
                item.status = ScheduleStatus::Processing;
                processed_ids.push(*id);
                // In real implementation, this would trigger the actual action
                println!("Processing scheduled item {}: {:?}", id, item.action);
            }
        }
        
        processed_ids
    }
}

impl Extension for SchedulingExtension {
    fn id(&self) -> &'static str {
        "core.scheduling"
    }
    
    fn name(&self) -> &'static str {
        "Content Scheduling"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add some sample scheduled content
        let sample_schedule = ScheduledContent {
            id: 0,
            content_type: ContentType::Post,
            content_id: 1,
            scheduled_at: "2024-12-31T23:59:59Z".to_string(), // Fixed future date for demo
            action: ScheduledAction::Publish,
            status: ScheduleStatus::Pending,
            created_at: now_iso8601(),
            created_by: 1,
        };
        
        self.schedule_content(sample_schedule);
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            ExtensionRoute {
                path: "/admin/scheduling".to_string(),
                requires_auth: true,
                admin_only: false,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "SchedulingManager".to_string(),
                description: "Manage scheduled content actions".to_string(),
            },
            ExtensionComponent {
                name: "ScheduleForm".to_string(),
                description: "Form for scheduling content actions".to_string(),
            },
            ExtensionComponent {
                name: "SchedulingCalendar".to_string(),
                description: "Calendar view of scheduled content".to_string(),
            },
        ]
    }
}

/// Scheduling manager component
#[component]
pub fn SchedulingManager() -> Element {
    let mut active_tab = use_signal(|| "upcoming".to_string());
    
    rsx! {
        div {
            h2 { "Content Scheduling" }
            p { "Schedule posts to be published, updated, or removed at specific times. Perfect for maintaining a consistent publishing schedule." }
            
            div {
                div {
                    button { 
                        onclick: move |_| active_tab.set("upcoming".to_string()),
                        "Upcoming (1)"
                    }
                    button { 
                        onclick: move |_| active_tab.set("history".to_string()),
                        "History"
                    }
                    button { 
                        onclick: move |_| active_tab.set("calendar".to_string()),
                        "Calendar View"
                    }
                }
                
                div {
                    if active_tab() == "upcoming" {
                        div {
                            div {
                                button { "Schedule New Content" }
                                button { "Bulk Actions" }
                            }
                            
                            div {
                                div {
                                    div {
                                        h4 { "📝 Publish: Extension Architecture Deep Dive" }
                                        span { "PENDING" }
                                    }
                                    div {
                                        div {
                                            span { "Scheduled:" }
                                            span { "Tomorrow at 9:00 AM" }
                                        }
                                        div {
                                            span { "Action:" }
                                            span { "Publish Post" }
                                        }
                                        div {
                                            span { "Created by:" }
                                            span { "Admin" }
                                        }
                                    }
                                    div {
                                        button { "Edit" }
                                        button { "Run Now" }
                                        button { "Cancel" }
                                    }
                                }
                                
                                div {
                                    div { "📅" }
                                    h3 { "No other scheduled content" }
                                    p { "Schedule posts to be published automatically at the perfect time." }
                                    button { "Schedule Content" }
                                }
                            }
                        }
                    }
                    
                    if active_tab() == "history" {
                        div {
                            div {
                                select {
                                    option { "All Actions" }
                                    option { "Published" }
                                    option { "Unpublished" }
                                    option { "Deleted" }
                                }
                                input {
                                    r#type: "date",
                                    placeholder: "From date"
                                }
                                input {
                                    r#type: "date",
                                    placeholder: "To date"
                                }
                            }
                            
                            div {
                                div {
                                    div {
                                        h4 { "✅ Published: Welcome to BananaBit CMS" }
                                        span { "COMPLETED" }
                                    }
                                    div {
                                        span { "Completed 2 days ago" }
                                    }
                                }
                                
                                div {
                                    div {
                                        h4 { "❌ Failed: Update Homepage Content" }
                                        span { "FAILED" }
                                    }
                                    div {
                                        span { "Failed 1 week ago" }
                                        span { "Error: Content not found" }
                                    }
                                }
                            }
                        }
                    }
                    
                    if active_tab() == "calendar" {
                        div {
                            SchedulingCalendar {}
                        }
                    }
                }
            }
        }
    }
}

/// Calendar view for scheduled content
#[component]
pub fn SchedulingCalendar() -> Element {
    rsx! {
        div {
            div {
                button { "‹ Previous" }
                h3 { "January 2024" }
                button { "Next ›" }
            }
            
            div {
                // Calendar days header
                div { "Sun" }
                div { "Mon" }
                div { "Tue" }
                div { "Wed" }
                div { "Thu" }
                div { "Fri" }
                div { "Sat" }
                
                // Calendar days (simplified)
                for day in 1..32 {
                    div { 
                        span { "{day}" }
                        if day == 15 {
                            div { "📝" }
                        }
                    }
                }
            }
            
            div {
                div {
                    span { }
                    span { "Publish" }
                }
                div {
                    span { }
                    span { "Unpublish" }
                }
                div {
                    span { }
                    span { "Delete" }
                }
            }
        }
    }
}