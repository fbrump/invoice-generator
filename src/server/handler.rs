use crate::server::errors::AppError;
use crate::server::models::transaction::Transaction;
use crate::server::serializes::CreateTransactionPayload;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::PgPool;
use tracing::{debug, info};
use uuid::Uuid;

pub async fn get_transctions(Extension(pool): Extension<PgPool>) -> Json<Vec<Transaction>> {
    let query_str = r#"
        SELECT * 
        FROM transactions 
    "#;
    let items = sqlx::query_as::<_, Transaction>(query_str)
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(items)
}

pub async fn get_transctions_by(
    Extension(pool): Extension<PgPool>,
    id: Path<Uuid>,
) -> Result<Json<Transaction>, AppError> {
    let query_str = r#"
        SELECT * 
        FROM transactions 
        WHERE id= $1
    "#;
    let result_item = sqlx::query_as::<_, Transaction>(query_str)
        .bind(&id.0)
        .fetch_one(&pool)
        .await;
    match result_item {
        Ok(item) => Ok(Json(item)),
        Err(_) => Err(AppError::BadRequest),
    }
}

pub async fn insert_transction(Json(payload): Json<CreateTransactionPayload>) -> impl IntoResponse {
    debug!("Entered in the handle");
    let id = Uuid::new_v4();
    let _ = payload.to_model(id);
    info!("Entered in the handle");
    (StatusCode::CREATED, Json(id))
}
