use std::net::SocketAddr;

use axum::{Router, routing::{get}};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::server::handler::{get_transctions, get_transctions_by, insert_transction};

mod server;

#[tokio::main]
async fn main() {
    // 1. Initialize tracing + log bridging
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .with_file(true)
        .init();

    // app
    let router_api = Router::new()
        .route("/monitoring/health", get(|| async { "OK" }))
        .route("/transactions/",get(get_transctions).post(insert_transction))
        .route("/transactions/{id}", get(get_transctions_by));

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                info!("Root endpoint was called");
                "Welcome to Invoice Generator API"
            }),
        )
        .nest("/api", router_api)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);
    // run app
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // run it
    let _ = axum::serve(listener, app).await;
}
