use mysql::*;
use std::env;
use std::sync::{Arc, Mutex};

pub type DbPool = Arc<Mutex<Pool>>;

pub fn get_db_pool() -> DbPool {
    let db_user = env::var("MYSQL_USER").unwrap_or_else(|_| "root".to_string());
    let db_pwd = env::var("MYSQL_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let db_name = env::var("MYSQL_DATABASE").unwrap_or_else(|_| "database".to_string());

    let opts = OptsBuilder::new()
        .user(Some(db_user))
        .pass(Some(db_pwd))
        .db_name(Some(db_name))
        .ip_or_hostname(Some("mysql-db-primary".to_string()))
        .tcp_port(3306);

    let pool = Pool::new(opts).expect("Failed to create pool");

    Arc::new(Mutex::new(pool))
}
