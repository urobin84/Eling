use crate::db::Database;
use crate::db::models::{EventDetails, ParticipantInfo, Event};
use tauri::State;

#[tauri::command]
pub async fn get_event_details(
    db: State<'_, Database>,
    event_id: i64,
) -> Result<EventDetails, String> {
    db.get_event_details(event_id)
        .await
        .map_err(|e| format!("Failed to get event details: {}", e))
}

#[tauri::command]
pub async fn get_event_participants(
    db: State<'_, Database>,
    event_id: i64,
) -> Result<Vec<ParticipantInfo>, String> {
    db.get_event_participants(event_id)
        .await
        .map_err(|e| format!("Failed to get participants: {}", e))
}

#[tauri::command]
pub async fn enroll_candidate_to_event(
    db: State<'_, Database>,
    event_code: String,
    user_id: i64,
) -> Result<i64, String> {
    // Find event by code
    let event = db.get_event_by_code(&event_code)
        .await
        .map_err(|_| "Invalid event code".to_string())?;
    
    // Check if already enrolled
    let already_enrolled = db.check_participant_access(event.id, user_id)
        .await
        .map_err(|e| format!("Failed to check enrollment: {}", e))?;
    
    if already_enrolled {
        return Err("Already enrolled in this event".to_string());
    }
    
    // Enroll
    db.add_participant_to_event(event.id, user_id, None)
        .await
        .map_err(|e| format!("Failed to enroll: {}", e))
}

#[tauri::command]
pub async fn add_participant_to_event(
    db: State<'_, Database>,
    event_id: i64,
    user_id: i64,
) -> Result<(), String> {
    db.add_participant_to_event(event_id, user_id, None)
        .await
        .map_err(|e| format!("Failed to add participant: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn remove_participant_from_event(
    db: State<'_, Database>,
    event_id: i64,
    user_id: i64,
) -> Result<(), String> {
    db.remove_participant_from_event(event_id, user_id)
        .await
        .map_err(|e| format!("Failed to remove participant: {}", e))
}

#[tauri::command]
pub async fn reset_participant(
    db: State<'_, Database>,
    event_id: i64,
    user_id: i64
) -> Result<(), String> {
    db.reset_participant_session(event_id, user_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_my_events(
    db: State<'_, Database>,
    user_id: i64,
) -> Result<Vec<crate::db::models::CandidateEvent>, String> {
    db.get_user_events(user_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_event_code_cmd(
    db: State<'_, Database>,
    event_id: i64,
) -> Result<String, String> {
    let code = db.generate_event_code()
        .await
        .map_err(|e| format!("Failed to generate code: {}", e))?;
    
    // Update event with code
    db.update_event_code(event_id, &code)
        .await
        .map_err(|e| format!("Failed to update event: {}", e))?;
    
    Ok(code)
}

#[tauri::command]
pub async fn delete_events(
    db: State<'_, Database>,
    event_ids: Vec<i64>,
) -> Result<(), String> {
    for id in event_ids {
        db.delete_event(id).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}
