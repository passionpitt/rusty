use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Pool, MySql};
use super::connection_string::ConnectionString;

pub async fn create_database_connection() -> Result<Pool<MySql>, sqlx::Error> {
    let db_connection: String = ConnectionString::fetch_connection_string();
    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .connect(db_connection.as_str())
        .await?;
    return Ok(pool);
}
