use dioxus::prelude::*;
use ui::App;

fn main() {
    use axum::{routing::get_service, Router};
    use dioxus::logger::tracing::*;
    use tower_http::services::ServeDir;

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let addr = dioxus::cli_config::fullstack_address_or_localhost();
        info!("🚀 Starting web server on http://{}", addr);

        // --- Build Axum Router ---
        // Determine the correct assets path - check if we're in Docker or local development
        let assets_path = if std::path::Path::new("assets").exists() {
            "assets" // Docker deployment path
        } else {
            "ba-server/assets" // Local development path
        };
        
        let app = Router::new()
            // Serve static assets from the appropriate directory
            .nest_service("/assets", get_service(ServeDir::new(assets_path)))
            // IMPORTANT: Dioxus needs to handle all routes for SPA
            .serve_dioxus_application(
                ServeConfig::builder()
                    .build()
                    .expect("Failed to build serve config"),
                App,
            );

        // --- Start server ---
        if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await {
            error!("🔥 Server error: {}", e);
        }
    });
}