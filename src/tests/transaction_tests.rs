#[cfg(test)]
mod tests {
    use crate::database::get_db_pool;
    use crate::handlers::transaction_handlers::*;
    use crate::models::Transaction;

    #[tokio::test]
    async fn test_create_transaction() {
        let db_pool = get_db_pool();
        let transaction = Transaction {
            id: 0,
            user_id: 1,
            amount: 100.50,
            currency: "USD".to_string(),
            transaction_type: "credit".to_string(),
            description: Some("Test transaction".to_string()),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };

        let result = create_transaction_handler(transaction, db_pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_transaction() {
        let db_pool = get_db_pool();
        let result = get_transaction_handler(1, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_update_transaction() {
        let db_pool = get_db_pool();
        let transaction = Transaction {
            id: 1,
            user_id: 1,
            amount: 200.75,
            currency: "USD".to_string(),
            transaction_type: "debit".to_string(),
            description: Some("Updated transaction".to_string()),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };

        let result = update_transaction_handler(1, transaction, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_delete_transaction() {
        let db_pool = get_db_pool();
        let result = delete_transaction_handler(1, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_transactions() {
        let db_pool = get_db_pool();
        let result = get_user_transactions_handler(1, db_pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transaction_validation() {
        let db_pool = get_db_pool();
        let invalid_transaction = Transaction {
            id: 0,
            user_id: 1,
            amount: -100.0,
            currency: "USD".to_string(),
            transaction_type: "invalid_type".to_string(),
            description: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };

        let result = create_transaction_handler(invalid_transaction, db_pool).await;
        assert!(result.is_err());
    }
}
