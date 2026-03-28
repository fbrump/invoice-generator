use axum::extract::{Json};
use crate::server::model::Transaction;
use crate::server::database::{db};

pub async fn get_transctions() ->  Json<Vec<Transaction>> {
    let items = db();
    Json(items)
}
