use poem::{
    error::NotFoundError,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    web::{Json, Path},
    EndpointExt, Response, Route, Server,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TransactionDto {
    id: u32,
    name: String,
    merchant_name: String,
    value: f32,
}

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

#[handler]
async fn get_transactions() -> Json<Vec<TransactionDto>> {
    let items = db_transactions();
    Json(items)
}

#[handler]
async fn get_transactions_by_id(id: Path<u32>) -> String {
    format!("Get one transaction {}", id.to_string())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/api/transactions/", get(get_transactions))
        .at("/api/transactions/:id", get(get_transactions_by_id))
        .catch_error(|err: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(format!("Sorry, Not Found. See more the error {}", err))
        });

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
