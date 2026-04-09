use crate::server::errors::AppError;
use crate::server::models::transaction::Transaction;
use crate::server::serializes::CreateTransactionPayload;
use axum::extract::{Json, Path};
use axum::Extension;
use sqlx::PgPool;
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

pub async fn insert_transction(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateTransactionPayload>,
) -> Result<Json<Uuid>, AppError> {
    let id = Uuid::new_v4();
    let transaction = payload.to_model(id);
    let query_str = r#"
        INSERT INTO transactions 
        (id, description, merchant_name, value, currency, created_at)
        VALUES
        ($1,$2,$3,$4,$5,$6) 
        RETURNING id;
    "#;
    println!("{}", query_str);
    println!("{:?}", transaction);
    let result_item = sqlx::query_scalar::<_, Uuid>(query_str)
        .bind(&transaction.id)
        .bind(&transaction.description)
        .bind(&transaction.merchant_name)
        .bind(&transaction.value)
        .bind(&transaction.currency)
        .bind(&transaction.created_at)
        .fetch_one(&pool)
        .await;

    match result_item {
        Ok(item_id) => Ok(Json(item_id)),
        Err(_) => Err(AppError::BadRequest),
    }
}
