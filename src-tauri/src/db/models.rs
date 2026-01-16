#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tool {
    pub id: i64,
    pub name: String,
    pub tool_type: String,
    pub category: String,
    pub config: Value, // JSON
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ToolSubtest {
    pub id: i64,
    pub tool_id: i64,
    pub subtest_name: String,
    pub time_limit_seconds: Option<i64>,
    pub instructions: Value, // JSON
    pub question_count: i64,
    pub sequence_order: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Question {
    pub id: i64,
    pub subtest_id: i64,
    pub question_text: String,
    pub question_type: String,
    pub options: Option<Value>, // JSON
    pub media_url: Option<String>,
    pub sequence_order: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i64,
    pub event_name: String,
    pub description: Option<String>,
    pub status: String,
    pub event_code: Option<String>,
    pub max_participants: Option<i64>,
    pub enrollment_deadline: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub password_hash: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: i64,
    pub event_id: i64,
    pub user_id: Option<i64>,
    pub participant_id: String,
    pub status: String,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub metadata: Option<Value>, // JSON
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Response {
    pub id: i64,
    pub session_id: i64,
    pub tool_id: i64,
    pub subtest_id: Option<i64>,
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SurveillanceLog {
    pub id: i64,
    pub session_id: i64,
    pub event_type: String,
    pub severity: String,
    pub event_data: Option<Value>,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: i64,
    pub session_id: i64,
    pub scores: Value,
    pub interpretations: Value,
    pub report_html: Option<String>,
    pub generated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TestResultDTO {
    pub id: i64,
    pub candidate_id: i64,
    pub candidate_name: String,
    pub event_id: i64,
    pub event_name: String,
    pub tool_id: i64,
    pub tool_name: String,
    pub score: i64,       // Derived from report JSON or just raw score for now
    pub raw_score: i64,
    pub percentile: Option<i64>,
    pub interpretation: Option<String>,
    pub status: String,   // from session status or logic
    pub completed_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: i64,
    pub user_id: Option<i64>,
    pub title: String,
    pub message: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub related_session_id: Option<i64>,
    pub related_user_id: Option<i64>,
    pub is_read: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct KraepelinResult {
    pub id: i64,
    pub session_id: i64,
    pub column_index: i32,
    pub answers: String, // JSON array
    pub correct_count: i32,
    pub total_questions: i32,
    pub time_taken: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KraepelinStats {
    pub total_columns: i32,
    pub total_correct: i32,
    pub total_questions: i32,
    pub total_time: i32,
    pub avg_correct_per_column: f64,
    pub avg_time_per_column: f64,
    pub accuracy: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EventParticipant {
    pub id: i64,
    pub event_id: i64,
    pub user_id: i64,
    pub status: String,
    pub enrolled_at: String,
    pub completed_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDetails {
    pub id: i64,
    pub event_name: String,
    pub description: Option<String>,
    pub event_code: Option<String>,
    pub status: String,
    pub max_participants: Option<i32>,
    pub enrollment_deadline: Option<String>,
    pub created_at: String,
    pub participant_count: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ParticipantInfo {
    pub user_id: i64,
    pub username: String,
    pub email: Option<String>,
    pub status: String,
    pub enrolled_at: String,
    pub completed_at: Option<String>,
}
