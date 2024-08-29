use warp::Rejection;
use crate::database::DbPool;
use mysql::PooledConn;
use crate::handlers::user_handlers::CustomError;

pub fn get_connection(db_pool: &DbPool) -> Result<PooledConn, Rejection> {
    db_pool
        .lock()
        .unwrap()
        .get_conn()
        .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to create connection pool", e)))
}
