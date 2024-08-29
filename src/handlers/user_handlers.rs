use warp::{Rejection, Reply};
use mysql::*;
use mysql::prelude::*;
use crate::database::DbPool;
use crate::models::User;
use serde_json::json;
use std::result::Result;
use crate::helpers::db_connection::get_connection;
use crate::helpers::custom_error::CustomError;

pub async fn create_user_handler(
    user: User,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    conn.exec_drop(
        "INSERT INTO users (name, email) VALUES (:name, :email)",
        params! {
            "name" => user.name,
            "email" => user.email,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to create new user", e)))?;

    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn get_user_handler(
    id: u64,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    let user: Option<User> = conn.exec_first(
        "SELECT id, name, email FROM users WHERE id = :id",
        params! {
            "id" => id,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to fetch user by user's id", e)))?;

    match user {
        Some(user) => Ok(warp::reply::json(&user)),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn update_user_handler(
    id: u64,
    user: User,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    conn.exec_drop(
        "UPDATE users SET name = :name, email = :email WHERE id = :id",
        params! {
            "id" => id,
            "name" => user.name,
            "email" => user.email,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to update user info", e)))?;

    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn delete_user_handler(
    id: u64,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    conn.exec_drop(
        "DELETE FROM users WHERE id = :id",
        params! {
            "id" => id,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to delete user", e)))?;

    Ok(warp::reply::json(&json!({"status": "deleted"})))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(custom_err) = err.find::<CustomError>() {
        let json = warp::reply::json(&json!({
            "error": custom_err.message,
            "cause": custom_err.cause.as_ref().map(|e| e.to_string()),
        }));
        Ok(warp::reply::with_status(json, warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    } else {
        Err(err)
    }
}
