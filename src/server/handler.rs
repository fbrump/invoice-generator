use crate::server::model::Transaction;
use crate::server::{database::db, model::CreateTransactionPayload};
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{debug, info};
use uuid::Uuid;

pub async fn get_transctions() -> Json<Vec<Transaction>> {
    let items = db();
    Json(items)
}

pub async fn get_transctions_by(id: Path<Uuid>) -> Json<Transaction> {
    let items = db();
    let result_item = items.into_iter().find(|item| item.id == *id);
    Json(result_item.expect("Error when trying to get one item"))
}

pub async fn insert_transction(Json(payload): 
    Json<CreateTransactionPayload>) -> impl IntoResponse {
        debug!("Entered in the handle");
        let id = Uuid::new_v4();
        let _ = payload.to_model(id);
        info!("Entered in the handle");
        (StatusCode::CREATED, Json(id))
}
