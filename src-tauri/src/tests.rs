#[cfg(test)]
mod tests {
    use crate::db::Database;
    use sqlx::sqlite::SqlitePoolOptions;
    use bcrypt;

    // Helper to generic a random DB name for isolation
    // fn generate_test_db_url() -> String { ... } - REMOVED to avoid extra deps


    async fn setup_test_db() -> Database {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to connect to in-memory DB");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Database::new(pool)
    }

    #[tokio::test]
    async fn test_password_hashing() {
        let password = "secured_password_123";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        
        assert_ne!(password, hash);
        assert!(bcrypt::verify(password, &hash).unwrap());
    }

    #[tokio::test]
    async fn test_create_candidate() {
        let db = setup_test_db().await;
        let username = "test_candidate";
        let password = "password123";
        
        // Manual hashing as done in command
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        
        let user_id = db.create_user(username, &hash, "participant")
            .await
            .expect("Failed to create candidate");

        assert!(user_id > 0);

        // Verify retrieval
        let user = db.get_user_by_username(username).await.expect("Failed to get user");
        assert_eq!(user.username, username);
        assert_eq!(user.role, "participant");
    }

    #[tokio::test]
    async fn test_duplicate_user_should_fail() {
        let db = setup_test_db().await;
        let username = "duplicate_user";
        let hash = "hash123";

        let _ = db.create_user(username, hash, "participant").await.unwrap();
        
        // Attempt duplicate creation
        let result = db.create_user(username, hash, "participant").await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_event() {
        let db = setup_test_db().await;
        let event_name = "Test Event 2026";
        let description = Some("Unit test event description".to_string());

        let event_id = db.create_event(event_name, description.clone())
            .await
            .expect("Failed to create event");

        assert!(event_id > 0);
        
        // Verify retrieval
        let events = db.get_all_events().await.expect("Failed to get events");
        let created_event = events.iter().find(|e| e.id == event_id).expect("Event not found");
        
        assert_eq!(created_event.event_name, event_name);
        assert_eq!(created_event.status, "draft"); 
    }

    #[tokio::test]
    async fn test_duplicate_event_name_should_fail() {
        let db = setup_test_db().await;
        let event_name = "Unique Event";
        
        let _ = db.create_event(event_name, None).await.unwrap();
        
        let result = db.create_event(event_name, None).await;
        assert!(result.is_err()); // Unique constraint check
    }

    #[tokio::test]
    async fn test_tiu_content_integrity() {
        let db = setup_test_db().await;
        
        // 1. Create Tool
        let tool_id = db.create_tool("TIU Test", "choice", "cognitive", "Unit Test TIU").await.unwrap();
        
        // 2. Create Subtest
        let sub_id = db.create_subtest(tool_id, "Verbal", 1, Some(300)).await.unwrap();
        
        // 3. Create Questions
        let _ = db.create_question(sub_id, "Q1", "multiple_choice", serde_json::json!({"correct": "A"}), 1).await.unwrap();
        let _ = db.create_question(sub_id, "Q2", "multiple_choice", serde_json::json!({"correct": "B"}), 2).await.unwrap();
        
        // 4. Verify Structure
        let subtests = db.get_subtests_by_tool(tool_id).await.unwrap();
        assert_eq!(subtests.len(), 1);
        assert_eq!(subtests[0].subtest_name, "Verbal");
        
        let questions = db.get_questions_by_subtest(sub_id).await.unwrap();
        assert_eq!(questions.len(), 2);
        assert_eq!(questions[0].question_text, "Q1");
    }
}
