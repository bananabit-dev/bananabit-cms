use dioxus::prelude::*;
use super::{Extension, ExtensionRoute, ExtensionComponent, AnalyticsEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use client::time::{now_iso8601, today_date};

/// Analytics metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: String,
    pub metadata: HashMap<String, String>,
}

/// Page view analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsPageView {
    pub url: String,
    pub title: String,
    pub user_agent: String,
    pub referrer: Option<String>,
    pub timestamp: String,
    pub duration: Option<u32>, // in seconds
}

/// Performance analytics extension
pub struct AnalyticsExtension {
    metrics: Vec<Metric>,
    page_views: Vec<AnalyticsPageView>,
    daily_stats: HashMap<String, DailyStats>, // date -> stats
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_views: u32,
    pub unique_visitors: u32,
    pub avg_session_duration: f64,
    pub bounce_rate: f64,
    pub top_pages: Vec<(String, u32)>, // (url, views)
}

impl AnalyticsExtension {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            page_views: Vec::new(),
            daily_stats: HashMap::new(),
        }
    }
    
    pub fn track_page_view(&mut self, page_view: AnalyticsPageView) {
        self.page_views.push(page_view);
    }
    
    pub fn record_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }
    
    pub fn get_daily_stats(&self, date: &str) -> Option<&DailyStats> {
        self.daily_stats.get(date)
    }
    
    pub fn generate_daily_stats(&mut self, date: &str) {
        let views_today: Vec<&AnalyticsPageView> = self.page_views
            .iter()
            .filter(|view| view.timestamp.starts_with(date))
            .collect();
            
        let total_views = views_today.len() as u32;
        let unique_visitors = views_today.len() as u32; // Simplified
        
        let stats = DailyStats {
            date: date.to_string(),
            total_views,
            unique_visitors,
            avg_session_duration: 180.0, // Mock data
            bounce_rate: 0.35,           // Mock data
            top_pages: vec![
                ("/".to_string(), total_views / 2),
                ("/post/welcome-to-bananabit-cms".to_string(), total_views / 4),
            ],
        };
        
        self.daily_stats.insert(date.to_string(), stats);
    }
}

impl Extension for AnalyticsExtension {
    fn id(&self) -> &'static str {
        "core.analytics"
    }
    
    fn name(&self) -> &'static str {
        "Performance Analytics"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add some sample data
        let today = today_date();
        
        // Sample page views
        for i in 0..50 {
            self.track_page_view(AnalyticsPageView {
                url: if i % 3 == 0 { "/" } else { "/post/welcome-to-bananabit-cms" }.to_string(),
                title: if i % 3 == 0 { "Home" } else { "Welcome to BananaBit CMS" }.to_string(),
                user_agent: "Mozilla/5.0 Browser".to_string(),
                referrer: if i % 4 == 0 { Some("https://google.com".to_string()) } else { None },
                timestamp: format!("{}T{:02}:00:00Z", today, (i % 24)),
                duration: Some(120 + (i * 10) % 300),
            });
        }
        
        // Generate daily stats
        self.generate_daily_stats(&today);
        
        // Sample metrics
        self.record_metric(Metric {
            name: "page_load_time".to_string(),
            value: 1.2,
            timestamp: now_iso8601(),
            metadata: [("page".to_string(), "/".to_string())].iter().cloned().collect(),
        });
        
        Ok(())
    }
    
    fn routes(&self) -> Vec<ExtensionRoute> {
        vec![
            ExtensionRoute {
                path: "/admin/analytics".to_string(),
                requires_auth: true,
                admin_only: false,
            },
        ]
    }
    
    fn components(&self) -> Vec<ExtensionComponent> {
        vec![
            ExtensionComponent {
                name: "AnalyticsDashboard".to_string(),
                description: "View performance metrics and analytics".to_string(),
            },
            ExtensionComponent {
                name: "TrafficChart".to_string(),
                description: "Visual representation of site traffic".to_string(),
            },
        ]
    }
}

/// Analytics dashboard component
#[component]
pub fn AnalyticsDashboard() -> Element {
    rsx! {
        div {
            h2 { "Performance Analytics" }
            p { "Monitor your site's performance, traffic, and user engagement metrics." }
            
            // Summary cards
            div {
                div {
                    h3 { "Today's Overview" }
                    div {
                        div {
                            h4 { "47" }
                            p { "Page Views" }
                            span { "↑ 12%" }
                        }
                        div {
                            h4 { "32" }
                            p { "Unique Visitors" }
                            span { "↑ 8%" }
                        }
                        div {
                            h4 { "3:02" }
                            p { "Avg. Session" }
                            span { "↑ 5%" }
                        }
                        div {
                            h4 { "35%" }
                            p { "Bounce Rate" }
                            span { "↓ 3%" }
                        }
                    }
                }
            }
            
            // Charts section
            div {
                div {
                    h3 { "Traffic Overview" }
                    div {
                        // Simplified chart representation
                        div {
                            div { "📊 Traffic chart would go here" }
                            p { "Page views over the last 7 days" }
                        }
                    }
                }
                
                div {
                    h3 { "Performance Metrics" }
                    div {
                        div {
                            h4 { "Page Load Time" }
                            p { "1.2s average" }
                            div { "█████░░░░░ Good" }
                        }
                        div {
                            h4 { "Server Response" }
                            p { "0.8s average" }
                            div { "████████░░ Excellent" }
                        }
                        div {
                            h4 { "Database Queries" }
                            p { "12 avg per page" }
                            div { "██████░░░░ Fair" }
                        }
                    }
                }
            }
            
            // Top content section
            div {
                h3 { "Top Content" }
                div {
                    div {
                        div {
                            div { "Page" }
                            div { "Views" }
                            div { "Duration" }
                            div { "Bounce Rate" }
                        }
                        
                        div {
                            div { "/" }
                            div { "24" }
                            div { "2:45" }
                            div { "28%" }
                        }
                        
                        div {
                            div { "/post/welcome-to-bananabit-cms" }
                            div { "18" }
                            div { "4:12" }
                            div { "22%" }
                        }
                        
                        div {
                            div { "/admin" }
                            div { "5" }
                            div { "8:30" }
                            div { "15%" }
                        }
                    }
                }
            }
            
            // Real-time section
            div {
                h3 { "Real-time Activity" }
                div {
                    div {
                        h4 { "Active Users" }
                        p { "3 users online" }
                    }
                    
                    div {
                        h4 { "Recent Page Views" }
                        div {
                            div { "/ - 2 minutes ago" }
                            div { "/post/welcome-to-bananabit-cms - 3 minutes ago" }
                            div { "/ - 5 minutes ago" }
                            div { "/admin/analytics - 8 minutes ago" }
                        }
                    }
                }
            }
            
            // Export section
            div {
                h3 { "Export Data" }
                div {
                    button { "Export CSV" }
                    button { "Export JSON" }
                    button { "Generate Report" }
                    button { "Schedule Email Reports" }
                }
            }
        }
    }
}