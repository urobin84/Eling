pub mod dashboard;
pub mod auth;
pub mod tools;
pub mod notifications;
pub mod kraepelin;
pub mod events;

use tauri::State;
use crate::db::Database;
use crate::db::models::Tool;

#[tauri::command]
pub async fn get_tools(db: State<'_, Database>) -> Result<Vec<Tool>, String> {
    db.get_all_tools().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_session(
    event_id: i64, 
    participant_id: String,
    metadata: Option<serde_json::Value>,
    db: State<'_, Database>
) -> Result<i64, String> {
    db.create_session(event_id, &participant_id, metadata)
        .await
        .map_err(|e| e.to_string())
}
