use warp::{Rejection, Reply};
use mysql::*;
use mysql::prelude::*;
use crate::database::DbPool;
use crate::models::Transaction;
use serde_json::json;
use std::result::Result;
use crate::helpers::db_connection::get_connection;
use crate::helpers::custom_error::CustomError;

pub async fn create_transaction_handler(
    transaction: Transaction,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    conn.exec_drop(
        "INSERT INTO transactions (user_id, amount, currency, transaction_type, description) VALUES (:user_id, :amount, :currency, :transaction_type, :description)",
        params! {
            "user_id" => transaction.user_id,
            "amount" => transaction.amount,
            "currency" => transaction.currency,
            "transaction_type" => transaction.transaction_type,
            "description" => transaction.description,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to create transaction", e)))?;

    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn get_transaction_handler(
    id: u64,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    let transaction: Option<Transaction> = conn.exec_first(
        "SELECT id, user_id, amount, currency, transaction_type, description, created_at, updated_at FROM transactions WHERE id = :id",
        params! {
            "id" => id,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to fetch transaction", e)))?;

    match transaction {
        Some(transaction) => Ok(warp::reply::json(&transaction)),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn update_transaction_handler(
    id: u64,
    transaction: Transaction,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    conn.exec_drop(
        "UPDATE transactions SET amount = :amount, currency = :currency, transaction_type = :transaction_type, description = :description WHERE id = :id",
        params! {
            "id" => id,
            "amount" => transaction.amount,
            "currency" => transaction.currency,
            "transaction_type" => transaction.transaction_type,
            "description" => transaction.description,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to update transaction", e)))?;

    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn delete_transaction_handler(
    id: u64,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    conn.exec_drop(
        "DELETE FROM transactions WHERE id = :id",
        params! {
            "id" => id,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to delete transaction", e)))?;

    Ok(warp::reply::json(&json!({"status": "deleted"})))
}

pub async fn get_user_transactions_handler(
    user_id: u64,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = get_connection(&db_pool)?;

    let transactions: Vec<Transaction> = conn.exec(
        "SELECT id, user_id, amount, currency, transaction_type, description, created_at, updated_at FROM transactions WHERE user_id = :user_id ORDER BY created_at DESC",
        params! {
            "user_id" => user_id,
        },
    )
    .map_err(|e| warp::reject::custom(CustomError::with_cause("Failed to fetch user transactions", e)))?;

    Ok(warp::reply::json(&transactions))
}
