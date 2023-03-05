#[derive(Debug)]
pub struct ConnectionString {
    pub user: String,
    pub password: String,
    pub host: String
}

impl ConnectionString {
    pub fn fetch_connection_string() -> String {
        let db_user: String = std::env::var("DB_USER").expect("DB_USER must be set.");
        let db_password: String = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
        return format!("mysql://{}:{}@localhost/rusty", db_user, db_password);
    }
}
