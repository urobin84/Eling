// Sync configuration and control commands

use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::sync::{SyncService, start_sync_worker};
use crate::db::candidate::CandidateDatabase;

#[derive(Clone)]
pub struct SyncState {
    pub server_url: Arc<Mutex<Option<String>>>,
    pub is_syncing: Arc<Mutex<bool>>,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            server_url: Arc::new(Mutex::new(None)),
            is_syncing: Arc::new(Mutex::new(false)),
        }
    }
}

/// Set server URL for sync (Candidate only)
#[tauri::command]
pub async fn set_server_url(
    url: String,
    state: State<'_, SyncState>,
) -> Result<String, String> {
    *state.server_url.lock().await = Some(url.clone());
    
    // Save to config file
    // TODO: Implement persistent config storage
    
    Ok(format!("Server URL set to: {}", url))
}

/// Get current server URL
#[tauri::command]
pub async fn get_server_url(
    state: State<'_, SyncState>,
) -> Result<Option<String>, String> {
    Ok(state.server_url.lock().await.clone())
}

/// Test connection to server
#[tauri::command]
pub async fn test_server_connection(
    state: State<'_, SyncState>,
    db_state: State<'_, Arc<CandidateDatabase>>,
) -> Result<bool, String> {
    let server_url = state.server_url.lock().await.clone()
        .ok_or("Server URL not set".to_string())?;

    let db = (**db_state).clone();
    let sync_service = SyncService::new(db, server_url);

    match sync_service.test_connection().await {
        Ok(connected) => Ok(connected),
        Err(e) => Err(format!("Connection failed: {:?}", e)),
    }
}

/// Manually trigger sync
#[tauri::command]
pub async fn trigger_sync(
    session_id: String,
    state: State<'_, SyncState>,
    db_state: State<'_, Arc<CandidateDatabase>>,
) -> Result<String, String> {
    let server_url = state.server_url.lock().await.clone()
        .ok_or("Server URL not set".to_string())?;

    let db = (**db_state).clone();
    let sync_service = SyncService::new(db, server_url);

    match sync_service.sync_test_result(&session_id).await {
        Ok(_) => Ok("Sync completed successfully".to_string()),
        Err(e) => Err(format!("Sync failed: {:?}", e)),
    }
}

/// Get sync queue status
#[tauri::command]
pub async fn get_sync_queue_status(
    db_state: State<'_, Arc<CandidateDatabase>>,
) -> Result<serde_json::Value, String> {
    let db = &**db_state;
    
    let pending = db.get_pending_syncs().await
        .map_err(|e| format!("Database error: {:?}", e))?;

    Ok(serde_json::json!({
        "pending_count": pending.len(),
        "items": pending
    }))
}

/// Start sync worker
pub async fn initialize_sync_worker(
    db: CandidateDatabase,
    server_url: String,
) {
    println!("🔄 Initializing sync worker...");
    start_sync_worker(db, server_url).await;
}
