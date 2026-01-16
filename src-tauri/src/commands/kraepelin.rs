use crate::db::Database;
use tauri::State;

#[tauri::command]
pub async fn create_kraepelin_session(
    db: State<'_, Database>,
    user_id: i64,
    event_id: Option<i64>,
    tool_id: i64,
) -> Result<i64, String> {
    // For now, create a simple session using participant_id as string
    // In a real implementation, you'd want to link user_id properly
    let participant_id = format!("user_{}", user_id);
    let event_id_val = event_id.unwrap_or(1); // Default to event 1 if not provided
    
    let metadata = serde_json::json!({
        "user_id": user_id,
        "tool_id": tool_id,
        "test_type": "kraepelin"
    });
    
    // Create session
    let session_id = db
        .create_session(event_id_val, &participant_id, Some(metadata))
        .await
        .map_err(|e| format!("Failed to create session: {}", e))?;
    
    Ok(session_id)
}

#[tauri::command]
pub async fn save_kraepelin_column(
    db: State<'_, Database>,
    session_id: i64,
    column_index: i32,
    answers: Vec<i32>,
    correct_count: i32,
    total_questions: i32,
    time_taken: i32,
) -> Result<(), String> {
    // Convert answers to JSON string
    let answers_json = serde_json::to_string(&answers)
        .map_err(|e| format!("Failed to serialize answers: {}", e))?;
    
    // Save to database
    db.create_kraepelin_result(
        session_id,
        column_index,
        &answers_json,
        correct_count,
        total_questions,
        time_taken,
    )
    .await
    .map_err(|e| format!("Failed to save column result: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn complete_kraepelin_session(
    db: State<'_, Database>,
    session_id: i64,
    total_correct: i32,
    total_answered: i32,
    accuracy: f64,
) -> Result<(), String> {
    // Calculate statistics
    let stats = db
        .calculate_kraepelin_statistics(session_id)
        .await
        .map_err(|e| format!("Failed to calculate statistics: {}", e))?;
    
    // For now, just log the completion
    // In a full implementation, you'd update session status and create a report
    println!("Kraepelin session {} completed:", session_id);
    println!("  Total correct: {}", total_correct);
    println!("  Total answered: {}", total_answered);
    println!("  Accuracy: {}%", accuracy);
    println!("  Total columns: {}", stats.total_columns);
    println!("  Avg correct per column: {:.2}", stats.avg_correct_per_column);
    println!("  Avg time per column: {:.2}s", stats.avg_time_per_column);
    
    // TODO: Implement session status update and report creation
    // This would require adding those methods to the Database struct
    
    Ok(())
}
