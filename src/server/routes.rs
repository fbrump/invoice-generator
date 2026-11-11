mod axum_router {
    use axum::{routing::get, Extension, Router};
    use sqlx::{Pool, Postgres};
    use tower_http::trace::TraceLayer;
    use tracing::info;

    use crate::server::handler::{
        get_transactions_report, get_transctions, get_transctions_by, insert_transction,
    };

    #[allow(deprecated, unused)]
    pub fn configure_routers() -> Router {
        Router::new()
            .route("/monitoring/health", get(|| async { "OK" }))
            .route(
                "/transactions",
                get(get_transctions).post(insert_transction),
            )
            .route("/transactions/{id}", get(get_transctions_by))
            .route("/reports/{report_type}", get(get_transactions_report))
    }

    #[allow(deprecated, unused)]
    pub fn configure_app(app_routers: Router, pool: Pool<Postgres>) -> Router {
        Router::new()
            .route(
                "/",
                get(|| async {
                    info!("Root endpoint was called");
                    "Welcome to Invoice Generator API"
                }),
            )
            .nest("/api", app_routers)
            .layer(Extension(pool))
            .layer(TraceLayer::new_for_http())
    }
}

mod utoipa_axum_routers {
    use axum::{routing::get, Extension, Router};
    use sqlx::{Pool, Postgres};
    use tower_http::trace::TraceLayer;
    use tracing::info;
    use utoipa::OpenApi;
    use utoipa_axum::{router::OpenApiRouter, routes};

    use crate::server::{self, api_docs::ApiDoc};

    pub fn configure_routers() -> OpenApiRouter {
        OpenApiRouter::new()
            .route("/monitoring/health", get(|| async { "OK" }))
            .routes(routes!(server::handler::get_transactions_report,))
            .routes(routes!(
                server::handler::insert_transction,
                server::handler::get_transctions,
            ))
            .routes(routes!(server::handler::get_transctions_by,))
    }

    pub fn configure_app(
        app_router: OpenApiRouter,
        pool: Pool<Postgres>,
    ) -> (Router, utoipa::openapi::OpenApi) {
        OpenApiRouter::with_openapi(ApiDoc::openapi())
            .route(
                "/",
                get(|| async {
                    info!("Root endpoint was called");
                    "Welcome to Invoice Generator API"
                }),
            )
            .nest("/api", app_router)
            .layer(Extension(pool))
            .layer(TraceLayer::new_for_http())
            .split_for_parts()
    }
}

pub use utoipa_axum_routers::{configure_app, configure_routers};
