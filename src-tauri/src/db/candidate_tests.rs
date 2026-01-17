// Candidate Database Unit Tests

#[cfg(test)]
mod candidate_db_tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> CandidateDatabase {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS local_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT UNIQUE NOT NULL,
                event_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                event_code TEXT,
                started_at TEXT NOT NULL DEFAULT (datetime('now')),
                completed_at TEXT,
                status TEXT DEFAULT 'in_progress',
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS local_answers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                question_id INTEGER NOT NULL,
                answer TEXT,
                answered_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (session_id) REFERENCES local_sessions(session_id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS sync_queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data_type TEXT NOT NULL,
                session_id TEXT NOT NULL,
                payload TEXT NOT NULL,
                priority INTEGER DEFAULT 5,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                synced BOOLEAN DEFAULT 0,
                synced_at TEXT,
                sync_attempts INTEGER DEFAULT 0,
                last_attempt_at TEXT,
                last_error TEXT,
                FOREIGN KEY (session_id) REFERENCES local_sessions(session_id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS local_recordings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                recording_type TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size INTEGER,
                duration INTEGER,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                uploaded BOOLEAN DEFAULT 0,
                uploaded_at TEXT,
                FOREIGN KEY (session_id) REFERENCES local_sessions(session_id) ON DELETE CASCADE
            );
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to run migrations");

        CandidateDatabase::new(pool)
    }

    // ===== Session Management Tests =====

    #[tokio::test]
    async fn test_create_local_session() {
        let db = setup_test_db().await;

        let session_id = db
            .create_local_session("test-session-1", 1, 1, Some("CODE123"))
            .await
            .expect("Failed to create session");

        assert!(session_id > 0);
    }

    #[tokio::test]
    async fn test_get_local_session() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-2", 1, 1, Some("CODE123"))
            .await
            .unwrap();

        let session = db
            .get_local_session("test-session-2")
            .await
            .expect("Failed to get session")
            .expect("Session not found");

        assert_eq!(session.session_id, "test-session-2");
        assert_eq!(session.event_id, 1);
        assert_eq!(session.user_id, 1);
        assert_eq!(session.status, "in_progress");
    }

    #[tokio::test]
    async fn test_update_session_status() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-3", 1, 1, None)
            .await
            .unwrap();

        db.update_session_status("test-session-3", "completed")
            .await
            .expect("Failed to update status");

        let session = db.get_local_session("test-session-3").await.unwrap().unwrap();
        assert_eq!(session.status, "completed");
    }

    #[tokio::test]
    async fn test_complete_session() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-4", 1, 1, None)
            .await
            .unwrap();

        db.complete_session("test-session-4")
            .await
            .expect("Failed to complete session");

        let session = db.get_local_session("test-session-4").await.unwrap().unwrap();
        assert_eq!(session.status, "completed");
        assert!(session.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_session_not_found() {
        let db = setup_test_db().await;

        let session = db
            .get_local_session("non-existent")
            .await
            .expect("Query failed");

        assert!(session.is_none());
    }

    // ===== Answer Management Tests =====

    #[tokio::test]
    async fn test_save_local_answer() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-5", 1, 1, None)
            .await
            .unwrap();

        let answer_id = db
            .save_local_answer("test-session-5", 1, Some("Answer A"))
            .await
            .expect("Failed to save answer");

        assert!(answer_id > 0);
    }

    #[tokio::test]
    async fn test_get_session_answers() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-6", 1, 1, None)
            .await
            .unwrap();

        db.save_local_answer("test-session-6", 1, Some("Answer 1"))
            .await
            .unwrap();
        db.save_local_answer("test-session-6", 2, Some("Answer 2"))
            .await
            .unwrap();
        db.save_local_answer("test-session-6", 3, Some("Answer 3"))
            .await
            .unwrap();

        let answers = db
            .get_session_answers("test-session-6")
            .await
            .expect("Failed to get answers");

        assert_eq!(answers.len(), 3);
        assert_eq!(answers[0].question_id, 1);
        assert_eq!(answers[1].question_id, 2);
        assert_eq!(answers[2].question_id, 3);
    }

    // ===== Sync Queue Tests =====

    #[tokio::test]
    async fn test_add_to_sync_queue() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-7", 1, 1, None)
            .await
            .unwrap();

        let queue_id = db
            .add_to_sync_queue("test_result", "test-session-7", "{}", 5)
            .await
            .expect("Failed to add to queue");

        assert!(queue_id > 0);
    }

    #[tokio::test]
    async fn test_get_pending_syncs() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-8", 1, 1, None)
            .await
            .unwrap();

        db.add_to_sync_queue("test_result", "test-session-8", "{}", 1)
            .await
            .unwrap();
        db.add_to_sync_queue("recording", "test-session-8", "{}", 5)
            .await
            .unwrap();

        let pending = db
            .get_pending_syncs()
            .await
            .expect("Failed to get pending syncs");

        assert_eq!(pending.len(), 2);
        // Should be ordered by priority (1 before 5)
        assert_eq!(pending[0].priority, 1);
        assert_eq!(pending[1].priority, 5);
    }

    #[tokio::test]
    async fn test_mark_synced() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-9", 1, 1, None)
            .await
            .unwrap();

        let queue_id = db
            .add_to_sync_queue("test_result", "test-session-9", "{}", 5)
            .await
            .unwrap();

        db.mark_synced(queue_id)
            .await
            .expect("Failed to mark as synced");

        let pending = db.get_pending_syncs().await.unwrap();
        assert_eq!(pending.len(), 0); // Should not include synced items
    }

    #[tokio::test]
    async fn test_increment_sync_attempts() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-10", 1, 1, None)
            .await
            .unwrap();

        let queue_id = db
            .add_to_sync_queue("test_result", "test-session-10", "{}", 5)
            .await
            .unwrap();

        db.increment_sync_attempts(queue_id, Some("Network error"))
            .await
            .expect("Failed to increment attempts");

        // Verify attempts incremented (would need to query directly)
        let pending = db.get_pending_syncs().await.unwrap();
        assert_eq!(pending[0].sync_attempts, 1);
        assert_eq!(pending[0].last_error, Some("Network error".to_string()));
    }

    #[tokio::test]
    async fn test_max_retry_limit() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-11", 1, 1, None)
            .await
            .unwrap();

        let queue_id = db
            .add_to_sync_queue("test_result", "test-session-11", "{}", 5)
            .await
            .unwrap();

        // Increment 3 times
        for _ in 0..3 {
            db.increment_sync_attempts(queue_id, Some("Error"))
                .await
                .unwrap();
        }

        let pending = db.get_pending_syncs().await.unwrap();
        // Should not return items with 3+ attempts
        assert_eq!(pending.len(), 0);
    }

    // ===== Cleanup Tests =====

    #[tokio::test]
    async fn test_cleanup_synced_data() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-12", 1, 1, None)
            .await
            .unwrap();

        db.save_local_answer("test-session-12", 1, Some("Answer"))
            .await
            .unwrap();

        let queue_id = db
            .add_to_sync_queue("test_result", "test-session-12", "{}", 5)
            .await
            .unwrap();

        db.mark_synced(queue_id).await.unwrap();

        db.cleanup_synced_data("test-session-12")
            .await
            .expect("Failed to cleanup");

        let answers = db.get_session_answers("test-session-12").await.unwrap();
        assert_eq!(answers.len(), 0);

        let session = db.get_local_session("test-session-12").await.unwrap().unwrap();
        assert_eq!(session.status, "synced");
    }

    #[tokio::test]
    async fn test_cascade_delete_on_session() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-13", 1, 1, None)
            .await
            .unwrap();

        db.save_local_answer("test-session-13", 1, Some("Answer"))
            .await
            .unwrap();

        db.add_to_sync_queue("test_result", "test-session-13", "{}", 5)
            .await
            .unwrap();

        // Delete session
        sqlx::query("DELETE FROM local_sessions WHERE session_id = ?")
            .bind("test-session-13")
            .execute(&db.pool)
            .await
            .unwrap();

        // Answers and queue items should be deleted too
        let answers = db.get_session_answers("test-session-13").await.unwrap();
        assert_eq!(answers.len(), 0);
    }

    // ===== Recording Tests =====

    #[tokio::test]
    async fn test_save_recording_metadata() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-14", 1, 1, None)
            .await
            .unwrap();

        let recording_id = db
            .save_recording_metadata("test-session-14", "camera", "/path/to/video.webm", 1024, Some(60))
            .await
            .expect("Failed to save recording");

        assert!(recording_id > 0);
    }

    #[tokio::test]
    async fn test_get_session_recordings() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-15", 1, 1, None)
            .await
            .unwrap();

        db.save_recording_metadata("test-session-15", "camera", "/path/camera.webm", 1024, None)
            .await
            .unwrap();

        db.save_recording_metadata("test-session-15", "screen", "/path/screen.webm", 2048, None)
            .await
            .unwrap();

        let recordings = db
            .get_session_recordings("test-session-15")
            .await
            .expect("Failed to get recordings");

        assert_eq!(recordings.len(), 2);
    }

    #[tokio::test]
    async fn test_mark_recording_uploaded() {
        let db = setup_test_db().await;

        db.create_local_session("test-session-16", 1, 1, None)
            .await
            .unwrap();

        db.save_recording_metadata("test-session-16", "camera", "/path/video.webm", 1024, None)
            .await
            .unwrap();

        db.mark_recording_uploaded("test-session-16", "camera")
            .await
            .expect("Failed to mark uploaded");

        let recordings = db.get_session_recordings("test-session-16").await.unwrap();
        // Should not return uploaded recordings
        assert_eq!(recordings.len(), 0);
    }
}
