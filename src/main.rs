use serde::{Deserialize, Serialize};

use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct TransactionDto {
    name: String,
    value: f32,
}

#[tokio::main]
async fn main() {
    let root_api = warp::path::end().map(|| "Root");
    // let get_transactions_all = warp::path!("api" / "transactions").map(|| "Transactions");
    let get_transactions_all = warp::get()
        .and(warp::path!("api" / "transactions"))
        .map(|| {
            let transaction = TransactionDto {
                name: "Coke".to_string(),
                value: 10.00,
            };
            warp::reply::json(&transaction)
        });
    let get_transactions_by_id =
        warp::path!("api" / "transactions" / u32).map(|id: u32| format!("Transactions {}", id));

    let routers = warp::get().and(root_api.or(get_transactions_all).or(get_transactions_by_id));

    warp::serve(routers).run(([127, 0, 0, 1], 3030)).await;
}
