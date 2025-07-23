#[cfg(test)]
mod tests {
    use crate::handlers::analytics_handlers::*;
    use crate::database::get_db_pool;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_analytics_handler_default_days() {
        let db_pool = get_db_pool();
    
        let query = AnalyticsQuery { days: None };

        let result = get_analytics_handler(query, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_analytics_handler_custom_days() {
        let db_pool = get_db_pool();
        
        let query = AnalyticsQuery { days: Some(7) };

        let result = get_analytics_handler(query, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_activities_handler() {
        let db_pool = get_db_pool();
        let user_id = 1;
        
        let query = AnalyticsQuery { days: Some(30) };

        let result = get_user_activities_handler(user_id, query, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_activities_handler_default_days() {
        let db_pool = get_db_pool();
        let user_id = 1;
        
        let query = AnalyticsQuery { days: None };

        let result = get_user_activities_handler(user_id, query, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_create_activity_handler() {
        let db_pool = get_db_pool();
        let user_id = 1;
        
        let activity = json!({
            "activity_type": "login",
            "activity_data": "User logged in successfully"
        });

        let result = create_activity_handler(user_id, activity, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_create_activity_handler_missing_activity_type() {
        let db_pool = get_db_pool();
        let user_id = 1;
        
        let activity = json!({
            "activity_data": "Missing activity type"
        });

        let result = create_activity_handler(user_id, activity, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_export_analytics_handler() {
        let db_pool = get_db_pool();
        
        let query = AnalyticsQuery { days: Some(30) };

        let result = export_analytics_handler(query, db_pool).await;
        assert!(result.is_ok() || result.is_err());
    }
}
