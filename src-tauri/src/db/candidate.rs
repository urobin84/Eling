// Candidate Database Manager
// Handles local SQLite database for candidate app

use sqlx::{SqlitePool, Error, Row};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct CandidateDatabase {
    pool: SqlitePool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSession {
    pub id: i64,
    pub session_id: String,
    pub event_id: i64,
    pub user_id: i64,
    pub event_code: Option<String>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalAnswer {
    pub id: i64,
    pub session_id: String,
    pub question_id: i64,
    pub answer: Option<String>,
    pub answered_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncQueueItem {
    pub id: i64,
    pub data_type: String,
    pub session_id: String,
    pub payload: String,
    pub priority: i32,
    pub created_at: String,
    pub synced: bool,
    pub sync_attempts: i32,
    pub last_error: Option<String>,
}

impl CandidateDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // ===== Local Sessions =====
    
    pub async fn create_local_session(
        &self,
        session_id: &str,
        event_id: i64,
        user_id: i64,
        event_code: Option<&str>,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO local_sessions (session_id, event_id, user_id, event_code) 
             VALUES (?, ?, ?, ?)"
        )
        .bind(session_id)
        .bind(event_id)
        .bind(user_id)
        .bind(event_code)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_local_session(&self, session_id: &str) -> Result<Option<LocalSession>, Error> {
        let session = sqlx::query_as::<_, LocalSession>(
            "SELECT * FROM local_sessions WHERE session_id = ?"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn update_session_status(&self, session_id: &str, status: &str) -> Result<(), Error> {
        sqlx::query("UPDATE local_sessions SET status = ? WHERE session_id = ?")
            .bind(status)
            .bind(session_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn complete_session(&self, session_id: &str) -> Result<(), Error> {
        sqlx::query(
            "UPDATE local_sessions 
             SET status = 'completed', completed_at = datetime('now') 
             WHERE session_id = ?"
        )
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ===== Local Answers =====

    pub async fn save_local_answer(
        &self,
        session_id: &str,
        question_id: i64,
        answer: Option<&str>,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO local_answers (session_id, question_id, answer) 
             VALUES (?, ?, ?)"
        )
        .bind(session_id)
        .bind(question_id)
        .bind(answer)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_session_answers(&self, session_id: &str) -> Result<Vec<LocalAnswer>, Error> {
        let answers = sqlx::query_as::<_, LocalAnswer>(
            "SELECT * FROM local_answers WHERE session_id = ? ORDER BY answered_at"
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(answers)
    }

    // ===== Sync Queue =====

    pub async fn add_to_sync_queue(
        &self,
        data_type: &str,
        session_id: &str,
        payload: &str,
        priority: i32,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO sync_queue (data_type, session_id, payload, priority) 
             VALUES (?, ?, ?, ?)"
        )
        .bind(data_type)
        .bind(session_id)
        .bind(payload)
        .bind(priority)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_pending_syncs(&self) -> Result<Vec<SyncQueueItem>, Error> {
        let items = sqlx::query_as::<_, SyncQueueItem>(
            "SELECT * FROM sync_queue 
             WHERE synced = 0 AND sync_attempts < 3 
             ORDER BY priority ASC, created_at ASC 
             LIMIT 10"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn mark_synced(&self, queue_id: i64) -> Result<(), Error> {
        sqlx::query(
            "UPDATE sync_queue 
             SET synced = 1, synced_at = datetime('now') 
             WHERE id = ?"
        )
        .bind(queue_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn increment_sync_attempts(&self, queue_id: i64, error: Option<&str>) -> Result<(), Error> {
        sqlx::query(
            "UPDATE sync_queue 
             SET sync_attempts = sync_attempts + 1, 
                 last_attempt_at = datetime('now'),
                 last_error = ? 
             WHERE id = ?"
        )
        .bind(error)
        .bind(queue_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ===== Cleanup =====

    pub async fn cleanup_synced_data(&self, session_id: &str) -> Result<(), Error> {
        // Delete synced queue items
        sqlx::query("DELETE FROM sync_queue WHERE session_id = ? AND synced = 1")
            .bind(session_id)
            .execute(&self.pool)
            .await?;

        // Delete local answers
        sqlx::query("DELETE FROM local_answers WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;

        // Update session status
        sqlx::query("UPDATE local_sessions SET status = 'synced' WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ===== Events Cache =====

    pub async fn cache_event(&self, event_id: i64, event_code: &str, event_name: &str, description: Option<&str>) -> Result<(), Error> {
        sqlx::query(
            "INSERT OR REPLACE INTO events_cache (id, event_code, event_name, description) 
             VALUES (?, ?, ?, ?)"
        )
        .bind(event_id)
        .bind(event_code)
        .bind(event_name)
        .bind(description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ===== Recording Management =====

    pub async fn save_recording_metadata(
        &self,
        session_id: &str,
        recording_type: &str,
        file_path: &str,
        file_size: i64,
        duration: Option<i64>,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO local_recordings (session_id, recording_type, file_path, file_size, duration) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(session_id)
        .bind(recording_type)
        .bind(file_path)
        .bind(file_size)
        .bind(duration)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_session_recordings(&self, session_id: &str) -> Result<Vec<(String, String, i64)>, Error> {
        let recordings = sqlx::query_as::<_, (String, String, i64)>(
            "SELECT recording_type, file_path, file_size 
             FROM local_recordings 
             WHERE session_id = ? AND uploaded = 0"
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(recordings)
    }

    pub async fn mark_recording_uploaded(&self, session_id: &str, recording_type: &str) -> Result<(), Error> {
        sqlx::query(
            "UPDATE local_recordings 
             SET uploaded = 1, uploaded_at = datetime('now') 
             WHERE session_id = ? AND recording_type = ?"
        )
        .bind(session_id)
        .bind(recording_type)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// Implement sqlx::FromRow for custom types
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for LocalSession {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            session_id: row.try_get("session_id")?,
            event_id: row.try_get("event_id")?,
            user_id: row.try_get("user_id")?,
            event_code: row.try_get("event_code")?,
            started_at: row.try_get("started_at")?,
            completed_at: row.try_get("completed_at")?,
            status: row.try_get("status")?,
        })
    }
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for LocalAnswer {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            session_id: row.try_get("session_id")?,
            question_id: row.try_get("question_id")?,
            answer: row.try_get("answer")?,
            answered_at: row.try_get("answered_at")?,
        })
    }
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for SyncQueueItem {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            data_type: row.try_get("data_type")?,
            session_id: row.try_get("session_id")?,
            payload: row.try_get("payload")?,
            priority: row.try_get("priority")?,
            created_at: row.try_get("created_at")?,
            synced: row.try_get("synced")?,
            sync_attempts: row.try_get("sync_attempts")?,
            last_error: row.try_get("last_error")?,
        })
    }
}
