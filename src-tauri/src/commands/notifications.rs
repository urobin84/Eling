use tauri::State;
use crate::db::{Database, models::Notification};

#[tauri::command]
pub async fn get_notifications(
    db: State<'_, Database>,
) -> Result<Vec<Notification>, String> {
    db.get_unread_notifications(None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_notification_read(
    notification_id: i64,
    db: State<'_, Database>,
) -> Result<(), String> {
    db.mark_notification_read(notification_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_all_notifications_read(
    db: State<'_, Database>,
) -> Result<(), String> {
    db.mark_all_notifications_read(None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_test_completion_notification(
    session_id: i64,
    candidate_name: String,
    db: State<'_, Database>,
) -> Result<i64, String> {
    let title = "Test Completed";
    let message = format!("{} has completed their test session", candidate_name);
    
    db.create_notification(
        None, // Broadcast to all admins
        title,
        &message,
        "test_completed",
        Some(session_id),
        None,
    )
    .await
    .map_err(|e| e.to_string())
}
