use utoipa::{openapi, OpenApi};
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(description = "Invoice Generator API"),
    paths(),
    tags(
        (name="reports", description ="Reports"),
        (name="transactions", description ="Transactions")
    )
)]
pub struct ApiDoc;

pub fn configure_swagger(api: openapi::OpenApi) -> SwaggerUi {
    SwaggerUi::new("/docs/swagger").url("/api-doc/openapi.json", api)
}

pub fn configure_scalar(api: openapi::OpenApi) -> Scalar<utoipa::openapi::OpenApi> {
    Scalar::with_url("/docs/scalar", api)
}

mod config_redoc {
    use utoipa::openapi;
    use utoipa_redoc::{Redoc, Servable};

    pub fn configure_redoc(api: openapi::OpenApi) -> Redoc<utoipa::openapi::OpenApi> {
        Redoc::with_url("/docs/redoc", api)
    }
}

pub use config_redoc::configure_redoc;
