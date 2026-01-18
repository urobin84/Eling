pub mod models;
pub mod seed;
pub mod candidate;
pub mod admin_sync;

use sqlx::{SqlitePool, Error, Row};
use self::models::*;



#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // --- Tools ---
    pub async fn create_tool(&self, name: &str, tool_type: &str, category: &str, description: &str) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO tools (name, tool_type, category, config)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(name)
        .bind(tool_type)
        .bind(category)
        .bind(serde_json::json!({ "description": description }))
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_all_tools(&self) -> Result<Vec<Tool>, Error> {
        sqlx::query_as::<_, Tool>("SELECT * FROM tools ORDER BY id")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_tool_by_id(&self, id: i64) -> Result<Tool, Error> {
        sqlx::query_as::<_, Tool>("SELECT * FROM tools WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_tool_by_name(&self, name: &str) -> Result<Tool, Error> {
        sqlx::query_as::<_, Tool>("SELECT * FROM tools WHERE name = ?")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_subtests_by_tool(&self, tool_id: i64) -> Result<Vec<ToolSubtest>, Error> {
        sqlx::query_as::<_, ToolSubtest>("SELECT * FROM tool_subtests WHERE tool_id = ? ORDER BY sequence_order")
            .bind(tool_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_questions_by_subtest(&self, subtest_id: i64) -> Result<Vec<Question>, Error> {
        sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE subtest_id = ? ORDER BY sequence_order")
            .bind(subtest_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_subtest(&self, tool_id: i64, name: &str, sequence: i64, time_limit: Option<i64>) -> Result<i64, Error> {
         let id = sqlx::query(
            r#"
            INSERT INTO tool_subtests (tool_id, subtest_name, sequence_order, time_limit_seconds, instructions, question_count)
            VALUES (?, ?, ?, ?, '{}', 0)
            "#
        )
        .bind(tool_id)
        .bind(name)
        .bind(sequence)
        .bind(time_limit)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn delete_subtest(&self, id: i64) -> Result<(), Error> {
        sqlx::query("DELETE FROM tool_subtests WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_question(&self, subtest_id: i64, text: &str, q_type: &str, options: serde_json::Value, sequence: i64) -> Result<i64, Error> {
         let id = sqlx::query(
            r#"
            INSERT INTO questions (subtest_id, question_text, question_type, options, sequence_order)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(subtest_id)
        .bind(text)
        .bind(q_type)
        .bind(options)
        .bind(sequence)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn delete_question(&self, id: i64) -> Result<(), Error> {
        sqlx::query("DELETE FROM questions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_question(&self, id: i64, text: &str, q_type: &str, options: serde_json::Value) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE questions 
            SET question_text = ?, question_type = ?, options = ?
            WHERE id = ?
            "#
        )
        .bind(text)
        .bind(q_type)
        .bind(options)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Sessions ---
    pub async fn create_session(
        &self, 
        event_id: i64, 
        participant_id: &str,
        metadata: Option<serde_json::Value>
    ) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO sessions (event_id, participant_id, metadata, status, started_at)
            VALUES (?, ?, ?, 'active', CURRENT_TIMESTAMP)
            RETURNING id
            "#
        )
        .bind(event_id)
        .bind(participant_id)
        .bind(metadata)
        .fetch_one(&self.pool)
        .await?
        .get(0);
        
        Ok(id)
    }

    pub async fn create_user(&self, username: &str, password_hash: &str, role: &str) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO users (username, password_hash, role, created_at)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(username)
        .bind(password_hash)
        .bind(role)
        .bind(chrono::Local::now().naive_local())
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn update_user_password(&self, username: &str, password_hash: &str) -> Result<(), Error> {
        sqlx::query("UPDATE users SET password_hash = ? WHERE username = ?")
            .bind(password_hash)
            .bind(username)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_user_avatar(&self, username: &str, avatar_url: Option<String>) -> Result<(), Error> {
        sqlx::query("UPDATE users SET avatar_url = ? WHERE username = ?")
            .bind(avatar_url)
            .bind(username)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_event(&self, name: &str, description: Option<String>, event_date: Option<String>) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO events (event_name, description, status, created_at, enrollment_deadline)
            VALUES (?, ?, 'draft', ?, ?)
            "#
        )
        .bind(name)
        .bind(description)
        .bind(chrono::Local::now().naive_local())
        .bind(event_date)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn delete_event(&self, event_id: i64) -> Result<(), Error> {
        // Due to ON DELETE CASCADE, this will automatically remove related:
        // - event_packages
        // - event_participants
        // - sessions
        sqlx::query("DELETE FROM events WHERE id = ?")
            .bind(event_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_tools_to_event(&self, event_id: i64, tool_ids: Vec<i64>) -> Result<(), Error> {
        println!("DEBUG: add_tools_to_event START. Event: {}, Tools: {:?}", event_id, tool_ids);
        for (index, tool_id) in tool_ids.iter().enumerate() {
            // First check if the tool exists
            let tool_exists = sqlx::query("SELECT id FROM tools WHERE id = ?")
                .bind(tool_id)
                .fetch_optional(&self.pool)
                .await?
                .is_some();
            
            if !tool_exists {
                println!("DEBUG: Tool {} does not exist, skipping", tool_id);
                continue; // Skip non-existent tools
            }

            println!("DEBUG: Tool {} exists. Checking package...", tool_id);

            // Check if package already exists for this tool
            let existing_package = sqlx::query("SELECT id FROM packages WHERE tool_id = ? AND version = '1.0'")
                .bind(tool_id)
                .fetch_optional(&self.pool)
                .await?;

            let package_id = if let Some(row) = existing_package {
                let id: i64 = sqlx::Row::get(&row, "id");
                println!("DEBUG: Using existing package {} for tool {}", id, tool_id);
                id
            } else {
                println!("DEBUG: Creating NEW package for tool {}", tool_id);
                // Create new package
                let id = sqlx::query(
                    r#"
                    INSERT INTO packages (tool_id, package_name, version, content_data)
                    VALUES (?, ?, '1.0', '{}')
                    "#
                )
                .bind(tool_id)
                .bind(format!("Package for tool {}", tool_id))
                .execute(&self.pool)
                .await?
                .last_insert_rowid();
                println!("DEBUG: Created new package {} for tool {}", id, tool_id);
                id
            };

            // Link package to event
            println!("DEBUG: Linking package {} to event {}", package_id, event_id);
            // Use INSERT OR IGNORE to prevent duplicate linking errors if retried
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO event_packages (event_id, package_id, sequence_order)
                VALUES (?, ?, ?)
                "#
            )
            .bind(event_id)
            .bind(package_id)
            .bind(index as i64)
            .execute(&self.pool)
            .await?;
            println!("DEBUG: Linked package {} to event {}", package_id, event_id);
        }
        println!("DEBUG: add_tools_to_event DONE");
        Ok(())
    }

    pub async fn get_event_packages(&self, event_id: i64) -> Result<Vec<(i64, String, String)>, Error> {
        println!("DEBUG: Fetching packages for event {}", event_id);
        let packages = sqlx::query_as::<_, (i64, String, String)>(
            r#"
            SELECT DISTINCT t.id, t.name, t.category
            FROM tools t
            JOIN packages p ON p.tool_id = t.id
            JOIN event_packages ep ON ep.package_id = p.id
            WHERE ep.event_id = ?
            ORDER BY t.id
            "#
        )
        .bind(event_id)
        .fetch_all(&self.pool)
        .await?;

        println!("DEBUG: Found {} packages for event {}", packages.len(), event_id);
        Ok(packages)
    }

    pub async fn get_all_events(&self) -> Result<Vec<Event>, Error> {
        sqlx::query_as::<_, Event>(
            r#"
            SELECT e.*, COUNT(ep.id) as participant_count
            FROM events e
            LEFT JOIN event_participants ep ON ep.event_id = e.id
            GROUP BY e.id
            ORDER BY e.created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_sessions_by_user(&self, user_id: i64) -> Result<Vec<Session>, Error> {
        sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE user_id = ? ORDER BY started_at DESC")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    // --- Reports / Results ---
    pub async fn get_all_test_results(&self) -> Result<Vec<TestResultDTO>, Error> {
        let sql = r#"
            SELECT 
                r.id as id,
                CAST(COALESCE(s.user_id, 0) AS INTEGER) as candidate_id,
                COALESCE(u.username, s.participant_id) as candidate_name,
                e.id as event_id,
                e.event_name as event_name,
                0 as tool_id,
                COALESCE(json_extract(s.metadata, '$.testName'), 'Assessment') as tool_name,
                CAST(COALESCE(json_extract(r.scores, '$.total_score'), 0) AS INTEGER) as score,
                CAST(COALESCE(json_extract(r.scores, '$.raw_score'), 0) AS INTEGER) as raw_score,
                CAST(json_extract(r.scores, '$.percentile') AS INTEGER) as percentile,
                COALESCE(
                    json_extract(r.interpretations, '$.ai_review'),
                    json_extract(r.interpretations, '$.response')
                ) as interpretation,
                s.status as status,
                CAST(s.id AS TEXT) as session_id,
                json_extract(s.metadata, '$.recordingId') as recording_id,
                r.generated_at as completed_at
            FROM reports r
            JOIN sessions s ON r.session_id = s.id
            JOIN events e ON s.event_id = e.id
            LEFT JOIN users u ON s.user_id = u.id
            ORDER BY r.generated_at DESC
        "#;

        sqlx::query_as::<_, TestResultDTO>(sql)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_test_result_by_id(&self, report_id: i64) -> Result<TestResultDTO, Error> {
        let sql = r#"
            SELECT 
                r.id as id,
                CAST(COALESCE(s.user_id, 0) AS INTEGER) as candidate_id,
                COALESCE(u.username, s.participant_id) as candidate_name,
                e.id as event_id,
                e.event_name as event_name,
                0 as tool_id,
                COALESCE(json_extract(s.metadata, '$.testName'), 'Assessment') as tool_name,
                CAST(COALESCE(json_extract(r.scores, '$.total_score'), 0) AS INTEGER) as score,
                CAST(COALESCE(json_extract(r.scores, '$.raw_score'), 0) AS INTEGER) as raw_score,
                CAST(json_extract(r.scores, '$.percentile') AS INTEGER) as percentile,
                COALESCE(
                    json_extract(r.interpretations, '$.ai_review'),
                    json_extract(r.interpretations, '$.response')
                ) as interpretation,
                s.status as status,
                CAST(s.id AS TEXT) as session_id,
                json_extract(s.metadata, '$.recordingId') as recording_id,
                r.generated_at as completed_at
            FROM reports r
            JOIN sessions s ON r.session_id = s.id
            JOIN events e ON s.event_id = e.id
            LEFT JOIN users u ON s.user_id = u.id
            WHERE r.id = ?
        "#;

        sqlx::query_as::<_, TestResultDTO>(sql)
            .bind(report_id)
            .fetch_one(&self.pool)
            .await
    }

    // --- Notifications ---
    pub async fn create_notification(
        &self,
        user_id: Option<i64>,
        title: &str,
        message: &str,
        notification_type: &str,
        related_session_id: Option<i64>,
        related_user_id: Option<i64>,
    ) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO notifications (user_id, title, message, type, related_session_id, related_user_id)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(user_id)
        .bind(title)
        .bind(message)
        .bind(notification_type)
        .bind(related_session_id)
        .bind(related_user_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_unread_notifications(&self, user_id: Option<i64>) -> Result<Vec<Notification>, Error> {
        let sql = if user_id.is_some() {
            "SELECT * FROM notifications WHERE (user_id = ? OR user_id IS NULL) AND is_read = 0 ORDER BY created_at DESC"
        } else {
            "SELECT * FROM notifications WHERE user_id IS NULL AND is_read = 0 ORDER BY created_at DESC"
        };
        
        sqlx::query_as::<_, Notification>(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn mark_notification_read(&self, notification_id: i64) -> Result<(), Error> {
        sqlx::query("UPDATE notifications SET is_read = 1 WHERE id = ?")
            .bind(notification_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn mark_all_notifications_read(&self, user_id: Option<i64>) -> Result<(), Error> {
        let sql = if user_id.is_some() {
            "UPDATE notifications SET is_read = 1 WHERE user_id = ? OR user_id IS NULL"
        } else {
            "UPDATE notifications SET is_read = 1 WHERE user_id IS NULL"
        };
        
        sqlx::query(sql)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // --- Kraepelin Results ---
    
    /// Create a Kraepelin result entry for a specific column
    pub async fn create_kraepelin_result(
        &self,
        session_id: i64,
        column_index: i32,
        answers: &str, // JSON string
        correct_count: i32,
        total_questions: i32,
        time_taken: i32,
    ) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO kraepelin_results 
            (session_id, column_index, answers, correct_count, total_questions, time_taken)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(session_id)
        .bind(column_index)
        .bind(answers)
        .bind(correct_count)
        .bind(total_questions)
        .bind(time_taken)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    /// Get all Kraepelin results for a session
    pub async fn get_kraepelin_results_by_session(&self, session_id: i64) -> Result<Vec<KraepelinResult>, Error> {
        sqlx::query_as::<_, KraepelinResult>(
            "SELECT * FROM kraepelin_results WHERE session_id = ? ORDER BY column_index"
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Calculate Kraepelin statistics for a session
    pub async fn calculate_kraepelin_statistics(&self, session_id: i64) -> Result<KraepelinStats, Error> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_columns,
                SUM(correct_count) as total_correct,
                SUM(total_questions) as total_questions,
                SUM(time_taken) as total_time,
                AVG(correct_count) as avg_correct_per_column,
                AVG(time_taken) as avg_time_per_column
            FROM kraepelin_results
            WHERE session_id = ?
            "#
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await?;

        let total_columns: i32 = row.get("total_columns");
        let total_correct: i32 = row.get("total_correct");
        let total_questions: i32 = row.get("total_questions");
        let total_time: i32 = row.get("total_time");
        let avg_correct_per_column: f64 = row.get("avg_correct_per_column");
        let avg_time_per_column: f64 = row.get("avg_time_per_column");

        let accuracy = if total_questions > 0 {
            (total_correct as f64 / total_questions as f64) * 100.0
        } else {
            0.0
        };

        Ok(KraepelinStats {
            total_columns,
            total_correct,
            total_questions,
            total_time,
            avg_correct_per_column,
            avg_time_per_column,
            accuracy,
        })
    }

    // --- Event Management ---
    
    /// Generate unique event code
    pub async fn generate_event_code(&self) -> Result<String, Error> {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789"; // Exclude ambiguous chars
        const CODE_LENGTH: usize = 6;
        
        loop {
            let code: String = (0..CODE_LENGTH)
                .map(|_| {
                    let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();
            
            // Check if code already exists
            let exists = sqlx::query("SELECT COUNT(*) as count FROM events WHERE event_code = ?")
                .bind(&code)
                .fetch_one(&self.pool)
                .await?
                .get::<i64, _>("count") > 0;
            
            if !exists {
                return Ok(code);
            }
        }
    }
    
    /// Get event by code
    pub async fn get_event_by_code(&self, code: &str) -> Result<Event, Error> {
        sqlx::query_as::<_, Event>("SELECT * FROM events WHERE event_code = ?")
            .bind(code)
            .fetch_one(&self.pool)
            .await
    }
    
    /// Update event code
    pub async fn update_event_code(&self, event_id: i64, code: &str) -> Result<(), Error> {
        sqlx::query("UPDATE events SET event_code = ? WHERE id = ?")
            .bind(code)
            .bind(event_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    /// Add participant to event
    pub async fn add_participant_to_event(
        &self,
        event_id: i64,
        user_id: i64,
        notes: Option<String>
    ) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO event_participants (event_id, user_id, notes)
            VALUES (?, ?, ?)
            "#
        )
        .bind(event_id)
        .bind(user_id)
        .bind(notes)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        Ok(id)
    }
    
    /// Remove participant from event
    pub async fn remove_participant_from_event(
        &self,
        event_id: i64,
        user_id: i64
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM event_participants WHERE event_id = ? AND user_id = ?")
            .bind(event_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    /// Get event participants
    pub async fn get_event_participants(&self, event_id: i64) -> Result<Vec<ParticipantInfo>, Error> {
        sqlx::query_as::<_, ParticipantInfo>(
            r#"
            SELECT 
                u.id as user_id,
                u.username,
                u.email,
                ep.status,
                ep.enrolled_at,
                ep.completed_at
            FROM event_participants ep
            JOIN users u ON ep.user_id = u.id
            WHERE ep.event_id = ?
            ORDER BY ep.enrolled_at DESC
            "#
        )
        .bind(event_id)
        .fetch_all(&self.pool)
        .await
    }
    
    /// Get user's events
    pub async fn get_user_events(&self, user_id: i64) -> Result<Vec<CandidateEvent>, Error> {
        sqlx::query_as::<_, CandidateEvent>(
            r#"
            SELECT 
                e.id,
                e.event_name,
                e.description,
                e.status,
                e.event_code,
                e.enrollment_deadline,
                e.created_at,
                ep.status as participant_status
            FROM events e
            JOIN event_participants ep ON e.id = ep.event_id
            WHERE ep.user_id = ?
            ORDER BY e.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }
    
    /// Check if user has access to event
    pub async fn check_participant_access(
        &self,
        event_id: i64,
        user_id: i64
    ) -> Result<bool, Error> {
        let count = sqlx::query(
            "SELECT COUNT(*) as count FROM event_participants WHERE event_id = ? AND user_id = ?"
        )
        .bind(event_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("count");
        
        Ok(count > 0)
    }
    
    /// Get event details with participant count
    pub async fn get_event_details(&self, event_id: i64) -> Result<EventDetails, Error> {
        // Get event info with participant count in one query
        let row = sqlx::query(
            r#"
            SELECT 
                e.id,
                e.event_name,
                e.description,
                e.event_code,
                e.status,
                e.max_participants,
                e.enrollment_deadline,
                e.created_at,
                COUNT(ep.id) as participant_count
            FROM events e
            LEFT JOIN event_participants ep ON e.id = ep.event_id
            WHERE e.id = ?
            GROUP BY e.id
            "#
        )
        .bind(event_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(EventDetails {
            id: row.get("id"),
            event_name: row.get("event_name"),
            description: row.get("description"),
            event_code: row.get("event_code"),
            status: row.get("status"),
            max_participants: row.get("max_participants"),
            enrollment_deadline: row.get("enrollment_deadline"),
            created_at: row.get::<chrono::NaiveDateTime, _>("created_at").to_string(),
            participant_count: row.get::<i64, _>("participant_count") as i32,
        })
    }

    // --- AI Review ---
    pub async fn update_report_ai_review(&self, report_id: i64, review: &str) -> Result<(), Error> {
        // Use json_set to update the 'ai_review' key in the interpretations JSON column
        // If interpretations is null, we initialize it as an object
        sqlx::query(
            r#"
            UPDATE reports 
            SET interpretations = json_set(COALESCE(interpretations, '{}'), '$.ai_review', ?)
            WHERE id = ?
            "#
        )
        .bind(review)
        .bind(report_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_report_by_id(&self, report_id: i64) -> Result<Report, Error> {
        sqlx::query_as::<_, Report>("SELECT * FROM reports WHERE id = ?")
            .bind(report_id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create_report(&self, session_id: i64, scores: serde_json::Value, interpretations: serde_json::Value) -> Result<i64, Error> {
        let id = sqlx::query(
            r#"
            INSERT INTO reports (session_id, scores, interpretations, generated_at)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(session_id)
        .bind(scores)
        .bind(interpretations)
        .bind(chrono::Local::now().naive_local())
        .execute(&self.pool)
        .await?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn delete_test_results(&self, report_ids: Vec<i64>) -> Result<(), Error> {
        // Bulk delete
        // Provide a placeholder string based on length
        if report_ids.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = report_ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM reports WHERE id IN ({})", placeholders.join(","));

        let mut query = sqlx::query(&sql);
        for id in report_ids {
            query = query.bind(id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    pub async fn reset_participant_session(&self, event_id: i64, user_id: i64) -> Result<(), Error> {
        // 1. Find the session ID for this user/event combo
        let session_query = sqlx::query(
            "SELECT id FROM sessions WHERE event_id = ? AND user_id = ?" // Assuming user_id is linked
        )
        .bind(event_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        // 2. Delete session (Cascade should handle reports, recordings etc if configured, else manual)
        // Check cascade support in SQLite schema. If not standard, we might need manual delete.
        // Assuming cascade for now based on previous 'delete_event' comment.
        if let Some(row) = session_query {
             let session_id: i64 = row.get("id");
             sqlx::query("DELETE FROM sessions WHERE id = ?")
                .bind(session_id)
                .execute(&self.pool)
                .await?;
        }

        // 3. Reset participant status to 'enrolled', clear completed_at
        sqlx::query(
            "UPDATE event_participants SET status = 'enrolled', completed_at = NULL WHERE event_id = ? AND user_id = ?"
        )
        .bind(event_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
