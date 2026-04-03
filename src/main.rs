use axum::{routing::get, Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    config::Config,
    server::handler::{get_transctions, get_transctions_by, insert_transction},
};

mod config;
mod server;

#[tokio::main]
async fn main() {
    let mut config: Config = Default::default();
    match Config::from_env() {
        Ok(_config) => {
            config = _config;
        }
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
        }
    };

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

    // database
    let pool = PgPoolOptions::new()
        .max_connections(15)
        .connect(&config.database_url)
        .await
        .expect("Database not connected");

    // app
    let router_api = Router::new()
        .route("/monitoring/health", get(|| async { "OK" }))
        .route(
            "/transactions/",
            get(get_transctions).post(insert_transction),
        )
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
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    info!("Starting server on {}", config.server_address);
    println!("Starting server on {}", config.server_address);
    // run app
    let listener = tokio::net::TcpListener::bind(config.server_address)
        .await
        .unwrap();

    // run it
    let _ = axum::serve(listener, app).await;
}
