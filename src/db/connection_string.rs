#[derive(Debug)]
pub struct ConnectionString {
    pub user: String,
    pub password: String,
    pub host: String,
    pub db_name: String
}

impl ConnectionString {
    pub fn fetch_connection_string() -> String {
        let db_user: String = std::env::var("DB_USER").expect("DB_USER must be set.");
        let db_password: String = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
        let db_host: String = std::env::var("DB_HOST").expect("DB_HOST must be set.");
        let db_name: String = std::env::var("DB_NAME").expect("DB_NAME must be set.");
        return format!("mysql://{}:{}@{}/{}", db_user, db_password, db_host, db_name);
    }
}
