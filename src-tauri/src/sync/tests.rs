// Sync Service Unit Tests

#[cfg(test)]
mod sync_tests {
    use super::*;
    use crate::db::candidate::CandidateDatabase;
    use crate::sync::SyncService;
    use mockito::Server;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> CandidateDatabase {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Create test schema
        sqlx::query(
            r#"
            CREATE TABLE local_sessions (
                id INTEGER PRIMARY KEY,
                session_id TEXT UNIQUE NOT NULL,
                event_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                event_code TEXT,
                started_at TEXT NOT NULL DEFAULT (datetime('now')),
                completed_at TEXT,
                status TEXT DEFAULT 'in_progress'
            );

            CREATE TABLE local_answers (
                id INTEGER PRIMARY KEY,
                session_id TEXT NOT NULL,
                question_id INTEGER NOT NULL,
                answer TEXT,
                answered_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        CandidateDatabase::new(pool)
    }

    #[tokio::test]
    async fn test_connection_success() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/api/health")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status": "ok"}"#)
            .create_async()
            .await;

        let db = setup_test_db().await;
        let sync_service = SyncService::new(db, server.url());

        let result: Result<bool, _> = sync_service.test_connection().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_connection_failure() {
        let db = setup_test_db().await;
        let sync_service = SyncService::new(db, "http://invalid-server:9999".to_string());

        let result: Result<bool, _> = sync_service.test_connection().await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sync_test_result_success() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("POST", "/api/test-results")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success": true, "result_id": 1}"#)
            .create_async()
            .await;

        let db = setup_test_db().await;
        
        // Create test session and answers
        db.create_local_session("test-123", 1, 1, Some("CODE123"))
            .await
            .unwrap();
        db.save_local_answer("test-123", 1, Some("Answer A"))
            .await
            .unwrap();
        db.complete_session("test-123").await.unwrap();

        let sync_service = SyncService::new(db, server.url());

        let result: Result<i64, _> = sync_service.sync_test_result("test-123").await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_sync_test_result_server_error() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("POST", "/api/test-results")
            .with_status(500)
            .create_async()
            .await;

        let db = setup_test_db().await;
        
        db.create_local_session("test-456", 1, 1, None)
            .await
            .unwrap();

        let sync_service = SyncService::new(db, server.url());

        let result: Result<i64, _> = sync_service.sync_test_result("test-456").await;

        assert!(result.is_err());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_event_by_code_success() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/api/events/ABC123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1, "event_name": "Test Event", "event_code": "ABC123"}"#)
            .create_async()
            .await;

        let db = setup_test_db().await;
        let sync_service = SyncService::new(db, server.url());

        let result: Result<serde_json::Value, _> = sync_service.get_event_by_code("ABC123").await;

        assert!(result.is_ok());
        let event = result.unwrap();
        assert_eq!(event["event_code"], "ABC123");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_event_by_code_not_found() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/api/events/NOTFOUND")
            .with_status(404)
            .create_async()
            .await;

        let db = setup_test_db().await;
        let sync_service = SyncService::new(db, server.url());

        let result: Result<serde_json::Value, _> = sync_service.get_event_by_code("NOTFOUND").await;

        assert!(result.is_err());
        mock.assert_async().await;
    }
}
