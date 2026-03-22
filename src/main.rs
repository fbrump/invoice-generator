use actix_web::{
    web::{self, Path},
    App, HttpResponse, HttpServer,
};
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
async fn get_transactions() -> HttpResponse {
    let items = db_transactions();
    HttpResponse::Ok().json(items)
}

async fn get_transactions_by_id(id: Path<u32>) -> HttpResponse {
    let id_parameter = *id;
    let result_item = db_transactions()
        .into_iter()
        .find(|item| item.id == id_parameter);

    match result_item {
        Some(_) => HttpResponse::Ok().json(result_item.expect("Error when trying to get one Item")),
        None => HttpResponse::BadRequest().json("Bad Request"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1".to_string();
    let port = "3000".to_string();

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Actix Web API" }))
            .service(
                web::scope("/api")
                    .route("/transactions/", web::get().to(get_transactions))
                    .route("/transactions/{id}", web::get().to(get_transactions_by_id)),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
