use warp::Filter;
use crate::database::DbPool;
use crate::handlers::transaction_handlers::*;

pub fn transaction_routes(db_pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_transaction = warp::path("transactions")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(create_transaction_handler);

    let get_transaction = warp::path!("transactions" / u64)
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(get_transaction_handler);

    let update_transaction = warp::path!("transactions" / u64)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(update_transaction_handler);

    let delete_transaction = warp::path!("transactions" / u64)
        .and(warp::delete())
        .and(with_db(db_pool.clone()))
        .and_then(delete_transaction_handler);

    let get_user_transactions = warp::path!("users" / u64 / "transactions")
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(get_user_transactions_handler);

    create_transaction
        .or(get_transaction)
        .or(update_transaction)
        .or(delete_transaction)
        .or(get_user_transactions)
}

fn with_db(db_pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
