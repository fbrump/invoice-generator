use crate::server::database::db;
use crate::server::model::Transaction;
use axum::extract::{Json, Path};
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
