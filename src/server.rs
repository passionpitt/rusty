use crate::database::get_db_pool;
use crate::handlers::user_handlers::handle_rejection;
use crate::routes::analytics_routes::analytics_routes;
use crate::routes::transaction_routes::transaction_routes;
use crate::routes::user_routes::user_routes;
use warp::Filter;

pub async fn run_server() {
    let db_pool = get_db_pool();
    let user_routes_filter = user_routes(db_pool.clone());
    let analytics_routes_filter = analytics_routes(db_pool.clone());
    let transaction_routes_filter = transaction_routes(db_pool.clone());
    let routes = user_routes_filter
        .or(analytics_routes_filter)
        .or(transaction_routes_filter)
        .recover(handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
