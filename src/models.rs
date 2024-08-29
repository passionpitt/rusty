use mysql::Row;
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl FromRow for User {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        Ok(User {
            id: row.get("id").unwrap_or_default(),
            name: row.get("name").unwrap_or_default(),
            email: row.get("email").unwrap_or_default(),
        })
    }
}
