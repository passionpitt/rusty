use crate::database::DbPool;
use crate::handlers::user_handlers::*;
use warp::Filter;

pub fn user_routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_user = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(create_user_handler);

    let get_user = warp::path!("users" / u64)
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(get_user_handler);

    let update_user = warp::path!("users" / u64)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(update_user_handler);

    let delete_user = warp::path!("users" / u64)
        .and(warp::delete())
        .and(with_db(db_pool.clone()))
        .and_then(delete_user_handler);

    // Combine all routes
    create_user.or(get_user).or(update_user).or(delete_user)
}

fn with_db(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
