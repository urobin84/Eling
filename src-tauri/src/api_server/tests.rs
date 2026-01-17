// API Server Unit Tests

#[cfg(test)]
mod api_server_tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::Database;
    use crate::api_server::{health_check, get_event_by_code, submit_test_result};
    use axum::{
        Router,
        routing::{get, post},
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use serde_json::json;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_app() -> Router {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Create test tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY,
                event_name TEXT NOT NULL,
                event_code TEXT UNIQUE,
                description TEXT,
                status TEXT DEFAULT 'draft'
            );

            CREATE TABLE IF NOT EXISTS test_results (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                event_id INTEGER NOT NULL,
                answers TEXT,
                client_session_id TEXT,
                sync_source TEXT DEFAULT 'direct',
                received_at TEXT
            );

            CREATE TABLE IF NOT EXISTS sync_log (
                id INTEGER PRIMARY KEY,
                client_session_id TEXT NOT NULL,
                user_id INTEGER NOT NULL,
                data_type TEXT NOT NULL,
                received_at TEXT NOT NULL DEFAULT (datetime('now')),
                payload_size INTEGER,
                client_ip TEXT,
                status TEXT DEFAULT 'success',
                error_message TEXT
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let db = Arc::new(Database::new(pool));

        Router::new()
            .route("/api/health", get(health_check))
            .route("/api/events/:code", get(get_event_by_code))
            .route("/api/test-results", post(submit_test_result))
            .with_state(db)
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = setup_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        assert!(json["version"].is_string());
    }

    #[tokio::test]
    async fn test_get_event_by_code_not_found() {
        let app = setup_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/events/NOTFOUND")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_submit_test_result_success() {
        let app = setup_test_app().await;

        let payload = json!({
            "session_id": "test-123",
            "event_id": 1,
            "user_id": 1,
            "answers": [
                {"question_id": 1, "answer": "A", "answered_at": "2024-01-01T00:00:00Z"}
            ],
            "completed_at": "2024-01-01T00:00:00Z"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/test-results")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], true);
        assert!(json["result_id"].is_number());
    }

    #[tokio::test]
    async fn test_submit_test_result_invalid_payload() {
        let app = setup_test_app().await;

        let payload = json!({
            "session_id": "test-123"
            // Missing required fields
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/test-results")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

