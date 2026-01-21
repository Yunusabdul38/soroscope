use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with env-filter support
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("SoroScope API Server Starting");

    // Build the application router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        // Add TraceLayer middleware to log request latency, method, and status codes
        .layer(TraceLayer::new_for_http());

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    
    tracing::info!("Server listening on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn health_check() -> &'static str {
    "OK"
}
