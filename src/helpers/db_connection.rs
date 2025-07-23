use crate::database::DbPool;
use crate::helpers::custom_error::CustomError;
use mysql::PooledConn;
use warp::Rejection;

pub fn get_connection(db_pool: &DbPool) -> Result<PooledConn, Rejection> {
    db_pool.lock().unwrap().get_conn().map_err(|e| {
        warp::reject::custom(CustomError::with_cause(
            "Failed to create connection pool",
            e,
        ))
    })
}
