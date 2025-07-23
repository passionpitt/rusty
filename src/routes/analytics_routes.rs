use crate::database::DbPool;
use crate::handlers::analytics_handlers::*;
use warp::Filter;

pub fn analytics_routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_analytics = warp::path("analytics")
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(get_analytics_handler);

    let get_user_activities = warp::path!("analytics" / "users" / u64 / "activities")
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(get_user_activities_handler);

    let create_activity = warp::path!("analytics" / "users" / u64 / "activities")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(create_activity_handler);

    let export_analytics = warp::path!("analytics" / "export")
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(export_analytics_handler);

    get_analytics
        .or(get_user_activities)
        .or(create_activity)
        .or(export_analytics)
}

fn with_db(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
