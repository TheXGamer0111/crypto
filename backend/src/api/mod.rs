use warp::Filter;

pub fn start_api() {
    let get_contracts = warp::path("contracts")
        .and(warp::get())
        .map(|| warp::reply::json(&"List of contracts"));

    let get_transactions = warp::path("transactions")
        .and(warp::get())
        .map(|| warp::reply::json(&"List of transactions"));

    let routes = get_contracts.or(get_transactions);
    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}