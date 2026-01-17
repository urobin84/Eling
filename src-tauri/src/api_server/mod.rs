// Admin REST API Server
// Receives data from candidate clients

use axum::{
    Router,
    routing::{post, get},
    extract::{State, Path, Multipart},
    http::StatusCode,
    Json,
};
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::db::Database;

// ===== Request/Response Types =====

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResultPayload {
    pub session_id: String,
    pub event_id: i64,
    pub user_id: i64,
    pub answers: Vec<AnswerData>,
    pub completed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerData {
    pub question_id: i64,
    pub answer: Option<String>,
    pub answered_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub success: bool,
    pub message: String,
    pub result_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventResponse {
    pub id: i64,
    pub event_name: String,
    pub event_code: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

// ===== API Server =====

pub struct ApiServer {
    db: Arc<Database>,
    port: u16,
}

impl ApiServer {
    pub fn new(db: Database, port: u16) -> Self {
        Self {
            db: Arc::new(db),
            port,
        }
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            // Health check
            .route("/api/health", get(health_check))
            // Event endpoints
            .route("/api/events/:code", get(get_event_by_code))
            // Test result submission
            .route("/api/test-results", post(submit_test_result))
            // Recording upload
            .route("/api/recordings", post(submit_recording))
            // CORS
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
            )
            .with_state(self.db);

        let addr = format!("0.0.0.0:{}", self.port);
        println!("🚀 API Server listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;


// ===== Handlers =====

/// Health check endpoint
pub(crate) async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

/// Get event by access code
pub(crate) async fn get_event_by_code(
    State(db): State<Arc<Database>>,
    Path(code): Path<String>,
) -> Result<Json<EventResponse>, StatusCode> {
    println!("📥 GET /api/events/{}", code);

    match db.get_event_by_code(&code).await {
        Ok(event) => {
            Ok(Json(EventResponse {
                id: event.id,
                event_name: event.event_name,
                event_code: event.event_code.unwrap_or_default(),
                description: event.description,
                status: event.status,
            }))
        }
        Err(e) => {
            eprintln!("❌ Error fetching event: {:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Submit test result from candidate
pub(crate) async fn submit_test_result(
    State(db): State<Arc<Database>>,
    Json(payload): Json<TestResultPayload>,
) -> Result<Json<SubmitResponse>, StatusCode> {
    println!("📥 POST /api/test-results - Session: {}", payload.session_id);

    // Convert answers to JSON
    let answers_json = match serde_json::to_string(&payload.answers) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("❌ Error serializing answers: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Save to database
    match db.save_synced_test_result(
        &payload.session_id,
        payload.user_id,
        payload.event_id,
        &answers_json,
    ).await {
        Ok(result_id) => {
            // Log sync
            db.log_sync(
                &payload.session_id,
                payload.user_id,
                "test_result",
                Some(answers_json.len() as i64),
                None,
                "success",
                None,
            ).await.ok();

            println!("✅ Test result saved: ID {}", result_id);

            Ok(Json(SubmitResponse {
                success: true,
                message: "Test result received successfully".to_string(),
                result_id: Some(result_id),
            }))
        }
        Err(e) => {
            eprintln!("❌ Error saving test result: {:?}", e);

            // Log failed sync
            db.log_sync(
                &payload.session_id,
                payload.user_id,
                "test_result",
                Some(answers_json.len() as i64),
                None,
                "failed",
                Some(&e.to_string()),
            ).await.ok();

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Submit recording from candidate
pub(crate) async fn submit_recording(
    State(_db): State<Arc<Database>>,
    mut multipart: Multipart,
) -> Result<Json<SubmitResponse>, StatusCode> {
    println!("📥 POST /api/recordings");

    let mut session_id: Option<String> = None;
    let mut recording_type: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;

    // Parse multipart form data
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "session_id" => {
                session_id = Some(field.text().await.unwrap_or_default());
            }
            "recording_type" => {
                recording_type = Some(field.text().await.unwrap_or_default());
            }
            "video" => {
                filename = field.file_name().map(|s| s.to_string());
                file_data = Some(field.bytes().await.unwrap_or_default().to_vec());
            }
            _ => {}
        }
    }

    let session_id = session_id.ok_or(StatusCode::BAD_REQUEST)?;
    let _recording_type = recording_type.ok_or(StatusCode::BAD_REQUEST)?;
    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    let filename = filename.unwrap_or_else(|| format!("{}.webm", session_id));

    // Save file to recordings directory
    let recordings_dir = std::path::Path::new("recordings");
    tokio::fs::create_dir_all(recordings_dir).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let file_path = recordings_dir.join(&filename);
    tokio::fs::write(&file_path, &file_data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("✅ Recording saved: {} ({} bytes)", filename, file_data.len());

    // TODO: Save metadata to database
    // For now, just return success

    Ok(Json(SubmitResponse {
        success: true,
        message: format!("Recording uploaded successfully: {}", filename),
        result_id: None,
    }))
}
