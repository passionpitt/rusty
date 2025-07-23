use mysql::prelude::FromRow;
use mysql::Row;
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserActivity {
    pub id: u64,
    pub user_id: u64,
    pub activity_type: String,
    pub activity_data: Option<String>,
    pub created_at: String,
}

impl FromRow for UserActivity {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        let created_at_value: mysql::Value = row.get("created_at").unwrap_or(mysql::Value::NULL);
        let created_at = match created_at_value {
            mysql::Value::Date(year, month, day, hour, minute, second, _) => {
                format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    year, month, day, hour, minute, second
                )
            }
            mysql::Value::NULL => "".to_string(),
            _ => created_at_value.to_string(),
        };

        Ok(UserActivity {
            id: row.get("id").unwrap_or_default(),
            user_id: row.get("user_id").unwrap_or_default(),
            activity_type: row.get("activity_type").unwrap_or_default(),
            activity_data: row.get("activity_data"),
            created_at,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSession {
    pub id: u64,
    pub user_id: u64,
    pub login_time: String,
    pub logout_time: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl FromRow for UserSession {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        Ok(UserSession {
            id: row.get("id").unwrap_or_default(),
            user_id: row.get("user_id").unwrap_or_default(),
            login_time: row.get("login_time").unwrap_or_default(),
            logout_time: row.get("logout_time"),
            ip_address: row.get("ip_address"),
            user_agent: row.get("user_agent"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub total_users: u64,
    pub active_users_7d: u64,
    pub active_users_30d: u64,
    pub active_users_90d: u64,
    pub user_registrations_by_day: Vec<DailyCount>,
    pub activity_by_type: Vec<ActivityTypeCount>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: u64,
    pub user_id: u64,
    pub amount: f64,
    pub currency: String,
    pub transaction_type: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl FromRow for Transaction {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        let created_at_value: mysql::Value = row.get("created_at").unwrap_or(mysql::Value::NULL);
        let created_at = match created_at_value {
            mysql::Value::Date(year, month, day, hour, minute, second, _) => {
                format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    year, month, day, hour, minute, second
                )
            }
            mysql::Value::NULL => "".to_string(),
            _ => created_at_value.to_string(),
        };

        let updated_at_value: mysql::Value = row.get("updated_at").unwrap_or(mysql::Value::NULL);
        let updated_at = match updated_at_value {
            mysql::Value::Date(year, month, day, hour, minute, second, _) => {
                format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    year, month, day, hour, minute, second
                )
            }
            mysql::Value::NULL => "".to_string(),
            _ => updated_at_value.to_string(),
        };

        Ok(Transaction {
            id: row.get("id").unwrap_or_default(),
            user_id: row.get("user_id").unwrap_or_default(),
            amount: row.get("amount").unwrap_or_default(),
            currency: row.get("currency").unwrap_or_default(),
            transaction_type: row.get("transaction_type").unwrap_or_default(),
            description: row.get("description"),
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct DailyCount {
    pub date: String,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct ActivityTypeCount {
    pub activity_type: String,
    pub count: u64,
}
