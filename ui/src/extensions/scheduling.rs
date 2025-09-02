use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent, Post};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let mut processed_ids = Vec::new();
        
        for (id, item) in &mut self.scheduled_items {
            if matches!(item.status, ScheduleStatus::Pending) && item.scheduled_at <= now {
                item.status = ScheduleStatus::Processing;
                processed_ids.push(*id);
                // In real implementation, this would trigger the actual action
                log::info!("Processing scheduled item {}: {:?}", id, item.action);
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
            scheduled_at: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(1))
                .unwrap()
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
            action: ScheduledAction::Publish,
            status: ScheduleStatus::Pending,
            created_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
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
    let active_tab = use_signal(|| "upcoming".to_string());
    
    rsx! {
        div { class: "scheduling-manager",
            h2 { "Content Scheduling" }
            p { class: "description",
                "Schedule posts to be published, updated, or removed at specific times. Perfect for maintaining a consistent publishing schedule."
            }
            
            div { class: "scheduling-tabs",
                div { class: "tab-navigation",
                    button { 
                        class: if active_tab() == "upcoming" { "tab-button active" } else { "tab-button" },
                        onclick: move |_| active_tab.set("upcoming".to_string()),
                        "Upcoming (1)"
                    }
                    button { 
                        class: if active_tab() == "history" { "tab-button active" } else { "tab-button" },
                        onclick: move |_| active_tab.set("history".to_string()),
                        "History"
                    }
                    button { 
                        class: if active_tab() == "calendar" { "tab-button active" } else { "tab-button" },
                        onclick: move |_| active_tab.set("calendar".to_string()),
                        "Calendar View"
                    }
                }
                
                div { class: "tab-content",
                    if active_tab() == "upcoming" {
                        div { class: "tab-panel",
                            div { class: "schedule-actions",
                                button { class: "btn btn-primary", "Schedule New Content" }
                                button { class: "btn btn-outline", "Bulk Actions" }
                            }
                            
                            div { class: "scheduled-items",
                                div { class: "schedule-item",
                                    div { class: "item-header",
                                        h4 { "📝 Publish: Extension Architecture Deep Dive" }
                                        span { class: "status-badge pending", "PENDING" }
                                    }
                                    div { class: "item-details",
                                        div { class: "detail-row",
                                            span { class: "label", "Scheduled:" }
                                            span { class: "value", "Tomorrow at 9:00 AM" }
                                        }
                                        div { class: "detail-row",
                                            span { class: "label", "Action:" }
                                            span { class: "value", "Publish Post" }
                                        }
                                        div { class: "detail-row",
                                            span { class: "label", "Created by:" }
                                            span { class: "value", "Admin" }
                                        }
                                    }
                                    div { class: "item-actions",
                                        button { class: "btn btn-sm btn-outline", "Edit" }
                                        button { class: "btn btn-sm btn-outline", "Run Now" }
                                        button { class: "btn btn-sm btn-danger", "Cancel" }
                                    }
                                }
                                
                                div { class: "empty-state",
                                    div { class: "empty-icon", "📅" }
                                    h3 { "No other scheduled content" }
                                    p { "Schedule posts to be published automatically at the perfect time." }
                                    button { class: "btn btn-primary", "Schedule Content" }
                                }
                            }
                        }
                    }
                    
                    if active_tab() == "history" {
                        div { class: "tab-panel",
                            div { class: "history-filters",
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
                            
                            div { class: "history-items",
                                div { class: "history-item completed",
                                    div { class: "item-header",
                                        h4 { "✅ Published: Welcome to BananaBit CMS" }
                                        span { class: "status-badge completed", "COMPLETED" }
                                    }
                                    div { class: "item-details",
                                        span { class: "timestamp", "Completed 2 days ago" }
                                    }
                                }
                                
                                div { class: "history-item failed",
                                    div { class: "item-header",
                                        h4 { "❌ Failed: Update Homepage Content" }
                                        span { class: "status-badge failed", "FAILED" }
                                    }
                                    div { class: "item-details",
                                        span { class: "timestamp", "Failed 1 week ago" }
                                        span { class: "error", "Error: Content not found" }
                                    }
                                }
                            }
                        }
                    }
                    
                    if active_tab() == "calendar" {
                        div { class: "tab-panel",
                            SchedulingCalendar {}
                        }
                    }
                }
            }
        }
        
        style { r#"
            .scheduling-manager {
                padding: 20px;
                max-width: 1200px;
                margin: 0 auto;
            }
            
            .description {
                color: var(--text-secondary, #a0aec0);
                margin-bottom: 30px;
            }
            
            .scheduling-tabs {
                background: var(--bg-secondary, #2d3748);
                border: 1px solid var(--border-color, #4a5568);
                border-radius: 8px;
            }
            
            .tab-navigation {
                display: flex;
                border-bottom: 1px solid var(--border-color, #4a5568);
            }
            
            .tab-button {
                padding: 15px 20px;
                background: transparent;
                border: none;
                color: var(--text-secondary, #a0aec0);
                cursor: pointer;
                transition: all 0.2s ease;
                border-bottom: 2px solid transparent;
            }
            
            .tab-button:hover {
                color: var(--text-primary, #e2e8f0);
                background: var(--bg-tertiary, #4a5568);
            }
            
            .tab-button.active {
                color: var(--accent-primary, #63b3ed);
                border-bottom-color: var(--accent-primary, #63b3ed);
            }
            
            .tab-content {
                padding: 30px;
            }
            
            .schedule-actions {
                display: flex;
                gap: 15px;
                margin-bottom: 30px;
            }
            
            .scheduled-items {
                space-y: 20px;
            }
            
            .schedule-item {
                background: var(--bg-primary, #1a202c);
                border: 1px solid var(--border-color, #4a5568);
                border-radius: 8px;
                padding: 20px;
                margin-bottom: 15px;
            }
            
            .item-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 15px;
            }
            
            .item-header h4 {
                margin: 0;
                color: var(--text-primary, #e2e8f0);
            }
            
            .status-badge {
                padding: 4px 8px;
                border-radius: 4px;
                font-size: 12px;
                font-weight: 600;
                text-transform: uppercase;
            }
            
            .status-badge.pending {
                background: var(--warning-color, #d69e2e);
                color: #ffffff;
            }
            
            .status-badge.completed {
                background: var(--success-color, #38a169);
                color: #ffffff;
            }
            
            .status-badge.failed {
                background: var(--error-color, #e53e3e);
                color: #ffffff;
            }
            
            .item-details {
                margin-bottom: 15px;
            }
            
            .detail-row {
                display: flex;
                margin-bottom: 5px;
            }
            
            .detail-row .label {
                width: 100px;
                color: var(--text-secondary, #a0aec0);
                font-weight: 500;
            }
            
            .detail-row .value {
                color: var(--text-primary, #e2e8f0);
            }
            
            .item-actions {
                display: flex;
                gap: 10px;
            }
            
            .empty-state {
                text-align: center;
                padding: 60px 20px;
                color: var(--text-secondary, #a0aec0);
            }
            
            .empty-icon {
                font-size: 48px;
                margin-bottom: 20px;
            }
            
            .empty-state h3 {
                margin: 0 0 10px 0;
                color: var(--text-primary, #e2e8f0);
            }
            
            .empty-state p {
                margin-bottom: 20px;
            }
            
            .history-filters {
                display: flex;
                gap: 15px;
                margin-bottom: 30px;
                flex-wrap: wrap;
            }
            
            .history-filters select,
            .history-filters input {
                padding: 8px 12px;
                border: 1px solid var(--border-color, #4a5568);
                border-radius: 6px;
                background: var(--bg-primary, #1a202c);
                color: var(--text-primary, #e2e8f0);
            }
            
            .history-item {
                background: var(--bg-primary, #1a202c);
                border: 1px solid var(--border-color, #4a5568);
                border-radius: 8px;
                padding: 20px;
                margin-bottom: 15px;
            }
            
            .timestamp {
                font-size: 14px;
                color: var(--text-secondary, #a0aec0);
            }
            
            .error {
                color: var(--error-color, #e53e3e);
                font-size: 14px;
                margin-left: 20px;
            }
            
            .btn {
                padding: 8px 16px;
                border-radius: 6px;
                border: 1px solid;
                cursor: pointer;
                font-weight: 500;
                transition: all 0.2s ease;
                text-decoration: none;
                display: inline-block;
            }
            
            .btn-sm {
                padding: 6px 12px;
                font-size: 14px;
            }
            
            .btn-primary {
                background: var(--accent-primary, #63b3ed);
                color: #ffffff;
                border-color: var(--accent-primary, #63b3ed);
            }
            
            .btn-outline {
                background: transparent;
                color: var(--accent-primary, #63b3ed);
                border-color: var(--accent-primary, #63b3ed);
            }
            
            .btn-danger {
                background: var(--error-color, #e53e3e);
                color: #ffffff;
                border-color: var(--error-color, #e53e3e);
            }
        "# }
    }
}

/// Calendar view for scheduled content
#[component]
pub fn SchedulingCalendar() -> Element {
    rsx! {
        div { class: "scheduling-calendar",
            div { class: "calendar-header",
                button { class: "btn btn-outline", "‹ Previous" }
                h3 { "January 2024" }
                button { class: "btn btn-outline", "Next ›" }
            }
            
            div { class: "calendar-grid",
                // Calendar days header
                div { class: "calendar-day-header", "Sun" }
                div { class: "calendar-day-header", "Mon" }
                div { class: "calendar-day-header", "Tue" }
                div { class: "calendar-day-header", "Wed" }
                div { class: "calendar-day-header", "Thu" }
                div { class: "calendar-day-header", "Fri" }
                div { class: "calendar-day-header", "Sat" }
                
                // Calendar days (simplified)
                for day in 1..32 {
                    div { 
                        class: if day == 15 { "calendar-day has-events" } else { "calendar-day" },
                        span { class: "day-number", "{day}" }
                        if day == 15 {
                            div { class: "event-indicator publish", "📝" }
                        }
                    }
                }
            }
            
            div { class: "calendar-legend",
                div { class: "legend-item",
                    span { class: "legend-color publish" }
                    span { "Publish" }
                }
                div { class: "legend-item",
                    span { class: "legend-color unpublish" }
                    span { "Unpublish" }
                }
                div { class: "legend-item",
                    span { class: "legend-color delete" }
                    span { "Delete" }
                }
            }
        }
        
        style { r#"
            .scheduling-calendar {
                max-width: 800px;
            }
            
            .calendar-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 20px;
            }
            
            .calendar-header h3 {
                margin: 0;
                color: var(--text-primary, #e2e8f0);
            }
            
            .calendar-grid {
                display: grid;
                grid-template-columns: repeat(7, 1fr);
                gap: 1px;
                background: var(--border-color, #4a5568);
                border: 1px solid var(--border-color, #4a5568);
            }
            
            .calendar-day-header {
                background: var(--bg-tertiary, #4a5568);
                padding: 10px;
                text-align: center;
                font-weight: 600;
                color: var(--text-primary, #e2e8f0);
                font-size: 14px;
            }
            
            .calendar-day {
                background: var(--bg-primary, #1a202c);
                min-height: 80px;
                padding: 8px;
                position: relative;
                cursor: pointer;
                transition: background-color 0.2s ease;
            }
            
            .calendar-day:hover {
                background: var(--bg-secondary, #2d3748);
            }
            
            .calendar-day.has-events {
                background: var(--bg-secondary, #2d3748);
            }
            
            .day-number {
                display: block;
                color: var(--text-primary, #e2e8f0);
                font-weight: 500;
                margin-bottom: 5px;
            }
            
            .event-indicator {
                position: absolute;
                bottom: 5px;
                right: 5px;
                width: 20px;
                height: 20px;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 12px;
            }
            
            .event-indicator.publish {
                background: var(--success-color, #38a169);
            }
            
            .calendar-legend {
                display: flex;
                gap: 20px;
                margin-top: 20px;
                justify-content: center;
            }
            
            .legend-item {
                display: flex;
                align-items: center;
                gap: 8px;
                color: var(--text-secondary, #a0aec0);
                font-size: 14px;
            }
            
            .legend-color {
                width: 16px;
                height: 16px;
                border-radius: 50%;
            }
            
            .legend-color.publish {
                background: var(--success-color, #38a169);
            }
            
            .legend-color.unpublish {
                background: var(--warning-color, #d69e2e);
            }
            
            .legend-color.delete {
                background: var(--error-color, #e53e3e);
            }
        "# }
    }
}