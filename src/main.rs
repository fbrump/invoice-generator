use serde::{Deserialize, Serialize};

use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct TransactionDto {
    name: String,
    value: f32,
}

fn handler_get_transactions() -> Vec<TransactionDto> {
    let mut transactions: Vec<TransactionDto> = vec![];
    transactions.push(TransactionDto {
        name: "Coke".to_string(),
        value: 10.00,
    });
    transactions.push(TransactionDto {
        name: "Coffee".to_string(),
        value: 0.30,
    });
    transactions.push(TransactionDto {
        name: "Cookie".to_string(),
        value: 2.00,
    });
    transactions
}

#[tokio::main]
async fn main() {
    let root_api = warp::path::end().map(|| "Root");
    // let get_transactions_all = warp::path!("api" / "transactions").map(|| "Transactions");
    let get_transactions_all = warp::get()
        .and(warp::path!("api" / "transactions"))
        .map(|| warp::reply::json(&handler_get_transactions()));
    
    let get_transactions_by_id =
        warp::path!("api" / "transactions" / u32).map(|id: u32| format!("Transactions {}", id));

    let routers = warp::get().and(root_api.or(get_transactions_all).or(get_transactions_by_id));

    warp::serve(routers).run(([127, 0, 0, 1], 3030)).await;
}
