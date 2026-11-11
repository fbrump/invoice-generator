use crate::{
    config::{configure_database, Config},
    server::{
        api_docs::{configure_redoc, configure_scalar, configure_swagger},
        routes,
    },
};
use tracing::info;
use tracing_subscriber::EnvFilter;

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
    let pool = configure_database(config.database_url).await;

    // app
    // let app_routers = configure_routers();
    let app_routers = routes::configure_routers();

    // let app = configure_app();
    let (router, api) = routes::configure_app(app_routers, pool);

    // Configure Documentation
    let router = router.merge(configure_swagger(api.clone()));
    let router = router.merge(configure_redoc(api.clone()));
    let router = router.merge(configure_scalar(api));

    info!("Starting server on {}", config.server_address);
    println!("Starting server on {}", config.server_address);

    // run app
    let listener = tokio::net::TcpListener::bind(config.server_address)
        .await
        .unwrap();

    // run it
    let _ = axum::serve(listener, router).await;
}
