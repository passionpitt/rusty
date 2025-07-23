use crate::database::DbPool;
use crate::helpers::custom_error::CustomError;
use crate::helpers::db_connection::get_connection;
use crate::models::{ActivityTypeCount, AnalyticsResponse, DailyCount, UserActivity};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use serde_json::json;
use std::result::Result;
use warp::{Rejection, Reply};

#[derive(Deserialize)]
pub struct AnalyticsQuery {
    pub days: Option<u32>,
}

pub async fn get_analytics_handler(
    query: AnalyticsQuery,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;
    let days = query.days.unwrap_or(30);

    let total_users: u64 = conn
        .query_first("SELECT COUNT(*) FROM users")
        .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to get total users", e)))?
        .unwrap_or(0);

    let active_users_7d: u64 = conn.exec_first(
        "SELECT COUNT(DISTINCT user_id) FROM user_activities WHERE created_at >= DATE_SUB(NOW(), INTERVAL 7 DAY)",
        ()
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to get 7d active users", e)))?
    .unwrap_or(0);

    let active_users_30d: u64 = conn.exec_first(
        "SELECT COUNT(DISTINCT user_id) FROM user_activities WHERE created_at >= DATE_SUB(NOW(), INTERVAL 30 DAY)",
        ()
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to get 30d active users", e)))?
    .unwrap_or(0);

    let active_users_90d: u64 = conn.exec_first(
        "SELECT COUNT(DISTINCT user_id) FROM user_activities WHERE created_at >= DATE_SUB(NOW(), INTERVAL 90 DAY)",
        ()
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to get 90d active users", e)))?
    .unwrap_or(0);

    let registrations: Vec<(mysql::Value, u64)> = conn.exec(
        "SELECT DATE(created_at) as date, COUNT(*) as count FROM users WHERE created_at >= DATE_SUB(NOW(), INTERVAL ? DAY) GROUP BY DATE(created_at) ORDER BY date",
        (days,)
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to get user registrations", e)))?;

    let user_registrations_by_day: Vec<DailyCount> = registrations
        .into_iter()
        .map(|(date, count)| DailyCount {
            date: match date {
                mysql::Value::Date(year, month, day, _, _, _, _) => {
                    format!("{:04}-{:02}-{:02}", year, month, day)
                }
                mysql::Value::NULL => "".to_string(),
                _ => format!("{:?}", date),
            },
            count,
        })
        .collect();

    let activities: Vec<(String, u64)> = conn.exec(
        "SELECT activity_type, COUNT(*) as count FROM user_activities WHERE created_at >= DATE_SUB(NOW(), INTERVAL ? DAY) GROUP BY activity_type ORDER BY count DESC",
        (days,)
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to get activity by type", e)))?;

    let activity_by_type: Vec<ActivityTypeCount> = activities
        .into_iter()
        .map(|(activity_type, count)| ActivityTypeCount {
            activity_type,
            count,
        })
        .collect();

    let response = AnalyticsResponse {
        total_users,
        active_users_7d,
        active_users_30d,
        active_users_90d,
        user_registrations_by_day,
        activity_by_type,
    };

    Ok(warp::reply::json(&response))
}

pub async fn get_user_activities_handler(
    user_id: u64,
    query: AnalyticsQuery,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;
    let days = query.days.unwrap_or(30);

    let activities: Vec<UserActivity> = conn.exec(
        "SELECT id, user_id, activity_type, activity_data, created_at FROM user_activities WHERE user_id = ? AND created_at >= DATE_SUB(NOW(), INTERVAL ? DAY) ORDER BY created_at DESC",
        (user_id, days)
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to get user activities", e)))?;

    Ok(warp::reply::json(&activities))
}

pub async fn create_activity_handler(
    user_id: u64,
    activity: serde_json::Value,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    let activity_type = activity
        .get("activity_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            warp::reject::custom(CustomError::with_cause(
                "Missing activity_type",
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing activity_type"),
            ))
        })?;

    let activity_data = activity.get("activity_data").map(|v| v.to_string());

    conn.exec_drop(
        "INSERT INTO user_activities (user_id, activity_type, activity_data) VALUES (?, ?, ?)",
        (user_id, activity_type, activity_data),
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to create activity", e)))?;

    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn export_analytics_handler(
    query: AnalyticsQuery,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let analytics_response = get_analytics_handler(query, db_pool).await?;

    Ok(warp::reply::with_header(
        analytics_response,
        "content-type",
        "application/json; charset=utf-8",
    ))
}
