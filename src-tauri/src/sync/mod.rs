// Candidate Sync Service
// Handles async data synchronization to admin server

use reqwest::{Client, multipart};
use serde::{Serialize, Deserialize};
use std::time::Duration;

use crate::db::candidate::CandidateDatabase;

#[derive(Debug)]
pub enum SyncError {
    NetworkError(reqwest::Error),
    ServerError(u16),
    DatabaseError(sqlx::Error),
    SerializationError(serde_json::Error),
}

impl From<reqwest::Error> for SyncError {
    fn from(err: reqwest::Error) -> Self {
        SyncError::NetworkError(err)
    }
}

impl From<sqlx::Error> for SyncError {
    fn from(err: sqlx::Error) -> Self {
        SyncError::DatabaseError(err)
    }
}

impl From<serde_json::Error> for SyncError {
    fn from(err: serde_json::Error) -> Self {
        SyncError::SerializationError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestResultPayload {
    session_id: String,
    event_id: i64,
    user_id: i64,
    answers: Vec<AnswerData>,
    completed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnswerData {
    question_id: i64,
    answer: Option<String>,
    answered_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubmitResponse {
    success: bool,
    message: String,
    result_id: Option<i64>,
}

pub struct SyncService {
    client: Client,
    server_url: String,
    db: CandidateDatabase,
}

impl SyncService {
    pub fn new(db: CandidateDatabase, server_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            client,
            server_url,
            db,
        }
    }

    /// Sync test result to server
    pub async fn sync_test_result(&self, session_id: &str) -> Result<i64, SyncError> {
        println!("🔄 Syncing test result for session: {}", session_id);

        // 1. Get session data
        let session = self.db.get_local_session(session_id).await?
            .ok_or(SyncError::DatabaseError(sqlx::Error::RowNotFound))?;

        // 2. Get all answers
        let answers = self.db.get_session_answers(session_id).await?;

        // 3. Build payload
        let payload = TestResultPayload {
            session_id: session_id.to_string(),
            event_id: session.event_id,
            user_id: session.user_id,
            answers: answers.into_iter().map(|a| AnswerData {
                question_id: a.question_id,
                answer: a.answer,
                answered_at: a.answered_at,
            }).collect(),
            completed_at: session.completed_at.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
        };

        // 4. Send to server
        let url = format!("{}/api/test-results", self.server_url);
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let result: SubmitResponse = response.json().await?;
            println!("✅ Test result synced successfully");
            Ok(result.result_id.unwrap_or(0))
        } else {
            eprintln!("❌ Server returned error: {}", response.status());
            Err(SyncError::ServerError(response.status().as_u16()))
        }
    }

    /// Sync recording to server
    pub async fn sync_recording(&self, session_id: &str, recording_type: &str) -> Result<(), SyncError> {
        println!("🔄 Syncing {} recording for session: {}", recording_type, session_id);

        // 1. Get recording metadata
        let recordings = self.db.get_session_recordings(session_id).await?;
        
        for (rec_type, file_path, _file_size) in recordings {
            if rec_type != recording_type {
                continue;
            }

            // 2. Read file
            let file_data = match tokio::fs::read(&file_path).await {
                Ok(data) => data,
                Err(_) => return Err(SyncError::ServerError(500)),
            };

            let filename = std::path::Path::new(&file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("recording.webm");

            // 3. Build multipart form
            let part = multipart::Part::bytes(file_data)
                .file_name(filename.to_string())
                .mime_str("video/webm")
                .unwrap();

            let form = multipart::Form::new()
                .text("session_id", session_id.to_string())
                .text("recording_type", recording_type.to_string())
                .part("video", part);

            // 4. Upload to server
            let url = format!("{}/api/recordings", self.server_url);
            let response = self.client
                .post(&url)
                .multipart(form)
                .send()
                .await?;

            if response.status().is_success() {
                println!("✅ Recording uploaded successfully");
                
                // Mark as uploaded in local DB
                self.db.mark_recording_uploaded(session_id, recording_type).await?;
            } else {
                eprintln!("❌ Server returned error: {}", response.status());
                return Err(SyncError::ServerError(response.status().as_u16()));
            }
        }

        Ok(())
    }

    /// Test connection to server
    pub async fn test_connection(&self) -> Result<bool, SyncError> {
        let url = format!("{}/api/health", self.server_url);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// Get event by code from server
    pub async fn get_event_by_code(&self, code: &str) -> Result<serde_json::Value, SyncError> {
        let url = format!("{}/api/events/{}", self.server_url, code);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let event = response.json().await?;
            Ok(event)
        } else {
            Err(SyncError::ServerError(response.status().as_u16()))
        }
    }
}

/// Background sync worker
pub async fn start_sync_worker(db: CandidateDatabase, server_url: String) {
    let sync_service = SyncService::new(db.clone(), server_url);

    tokio::spawn(async move {
        println!("🔄 Sync worker started");

        loop {
            // Check sync queue every 30 seconds
            tokio::time::sleep(Duration::from_secs(30)).await;

            match sync_service.db.get_pending_syncs().await {
                Ok(pending) => {
                    for item in pending {
                        println!("🔄 Processing sync item: {} ({})", item.id, item.data_type);

                        let result = match item.data_type.as_str() {
                            "test_result" => {
                                sync_service.sync_test_result(&item.session_id).await.map(|_| ())
                            }
                            "recording" => {
                                // Parse payload to get recording type
                                let payload: serde_json::Value = serde_json::from_str(&item.payload).unwrap_or_default();
                                let recording_type = payload["recording_type"].as_str().unwrap_or("camera");
                                sync_service.sync_recording(&item.session_id, recording_type).await
                            }
                            _ => {
                                eprintln!("❌ Unknown data type: {}", item.data_type);
                                continue;
                            }
                        };

                        match result {
                            Ok(_) => {
                                // Mark as synced
                                if let Err(e) = sync_service.db.mark_synced(item.id).await {
                                    eprintln!("❌ Error marking as synced: {:?}", e);
                                }

                                // Cleanup local data
                                if let Err(e) = sync_service.db.cleanup_synced_data(&item.session_id).await {
                                    eprintln!("❌ Error cleaning up: {:?}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Sync failed: {:?}", e);
                                
                                let error_msg = format!("{:?}", e);
                                if let Err(e) = sync_service.db.increment_sync_attempts(item.id, Some(&error_msg)).await {
                                    eprintln!("❌ Error incrementing attempts: {:?}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error fetching pending syncs: {:?}", e);
                }
            }
        }
    });
}

#[cfg(test)]
mod tests;

