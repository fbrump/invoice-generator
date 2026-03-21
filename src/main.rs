use axum::{extract::Path, response::Json, routing::get, Router};
use serde::Serialize;

// Contracts
#[derive(Debug, Serialize)]
struct TransactionDto {
    id: u32,
    name: String,
    merchant_name: String,
    value: f32,
}

// Database
fn db_transactions() -> Vec<TransactionDto> {
    vec![
        TransactionDto {
            id: 1,
            name: "Coffee".to_string(),
            merchant_name: "Lidl".to_string(),
            value: 5.50,
        },
        TransactionDto {
            id: 2,
            name: "Coke".to_string(),
            merchant_name: "Wallmart".to_string(),
            value: 7.80,
        },
    ]
}

// Handlers
async fn get_transactions() -> Json<Vec<TransactionDto>> {
    let items = db_transactions();
    Json(items)
}

async fn get_transactions_by_id(id: Path<u32>) -> Json<TransactionDto> {
    let id_parameter = *id;
    let result_item = db_transactions()
        .into_iter()
        .find(|item| item.id == id_parameter);
    Json(result_item.expect("Error when trying to get one Item"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "AXUM" }))
        .route("/api/transactions", get(get_transactions))
        .route("/api/transactions/{id}", get(get_transactions_by_id));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
