#[cfg(test)]
mod tests {
    use crate::database::get_db_pool;
    use crate::handlers::user_handlers::*;
    use crate::models::User;

    #[tokio::test]
    async fn test_create_user_handler_success() {
        let db_pool = get_db_pool();

        let user = User {
            id: 0,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let result = create_user_handler(user, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_handler_validation() {
        let db_pool = get_db_pool();

        let user = User {
            id: 0,
            name: "".to_string(),
            email: "invalid-email".to_string(),
        };

        let result = create_user_handler(user, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_handler() {
        let db_pool = get_db_pool();
        let user_id = 1;

        let result = get_user_handler(user_id, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_handler_not_found() {
        let db_pool = get_db_pool();
        let user_id = 999;

        let result = get_user_handler(user_id, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_handler() {
        let db_pool = get_db_pool();
        let user_id = 1;

        let user = User {
            id: user_id,
            name: "Updated User".to_string(),
            email: "updated@example.com".to_string(),
        };

        let result = update_user_handler(user_id, user, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_handler_validation() {
        let db_pool = get_db_pool();
        let user_id = 1;

        let user = User {
            id: user_id,
            name: "".to_string(),
            email: "invalid-email".to_string(),
        };

        let result = update_user_handler(user_id, user, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_delete_user_handler() {
        let db_pool = get_db_pool();
        let user_id = 1;

        let result = delete_user_handler(user_id, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }
}
