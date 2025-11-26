use warp::Filter;

#[tokio::main]
async fn main() {
    let root_api = warp::path::end().map(|| "Root");
    let get_transactions_all = warp::path!("api" / "transactions").map(|| "Transactions");
    let get_transactions_by_id =
        warp::path!("api" / "transactions" / u32).map(|id: u32| format!("Transactions {}", id));

    let routers = warp::get().and(root_api.or(get_transactions_all).or(get_transactions_by_id));

    warp::serve(routers).run(([127, 0, 0, 1], 3030)).await;
}
