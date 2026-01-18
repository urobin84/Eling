use tauri::State;
use crate::db::Database;
use crate::db::models::{User, Event, Session};
use bcrypt;
use serde_json::json;

#[tauri::command]
pub async fn generate_ai_review(db: State<'_, Database>, result_id: i64) -> Result<String, String> {
    // 1. Fetch Detailed Report Data
    let result = db.get_test_result_by_id(result_id).await.map_err(|e| e.to_string())?;
    
    // 2. Extract context
    let candidate_name = &result.candidate_name;
    let tool_name = &result.tool_name;
    let score = result.score;
    let raw_score = result.raw_score;
    let percentile = result.percentile.unwrap_or(0);
    
    // Construct Enhanced Psychometric Prompt
    let prompt = format!(
        "You are an expert psychometrician and psychological assessment assistant. \
        Analyze the following test results and provide a professional, encouraging, and insightful interpretation. \
        Avoid technical jargon where possible, and focus on practical implications.

Candidate Name: {}
Assessment Tool: {}
Final Score: {}
Raw Score: {}
Percentile: {}%

Please provide the interpretation in a structured way:
1. Executive Summary
2. Strengths
3. Areas for Development

Interpretation:",
        candidate_name, tool_name, score, raw_score, percentile
    );

    // 3. Call Ollama (gemma2:2b)
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": "gemma2:2b",
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.7,
                "top_p": 0.9
            }
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to AI engine (Ollama): {}. Please ensure Ollama is running.", e))?;
        
    if !res.status().is_success() {
        return Err(format!("AI Engine returned error: {}", res.status()));
    }
    
    let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let review = body["response"].as_str().unwrap_or("No response generated").to_string();

    // 4. Save Review
    db.update_report_ai_review(result_id, &review).await.map_err(|e| e.to_string())?;

    Ok(review)
}

#[tauri::command]
pub async fn update_test_interpretation(
    db: State<'_, Database>,
    result_id: i64,
    interpretation: String
) -> Result<(), String> {
    db.update_report_ai_review(result_id, &interpretation)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn get_all_users(db: State<'_, Database>) -> Result<Vec<User>, String> {
    db.get_all_users().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_events(db: State<'_, Database>) -> Result<Vec<Event>, String> {
    db.get_all_events().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_event(
    db: State<'_, Database>, 
    name: String, 
    description: Option<String>,
    event_date: Option<String>,
    tool_ids: Option<Vec<i64>>
) -> Result<i64, String> {
    println!("DEBUG: create_event called. Name: {}, Description: {:?}, Date: {:?}, Tools: {:?}", name, description, event_date, tool_ids);
    
    // Create the event
    let event_id = db.create_event(&name, description, event_date)
        .await
        .map_err(|e| e.to_string())?;
    
    println!("DEBUG: Created event id: {}", event_id);

    // Generate and set event code
    let event_code = db.generate_event_code()
        .await
        .map_err(|e| e.to_string())?;
    
    db.update_event_code(event_id, &event_code)
        .await
        .map_err(|e| e.to_string())?;
    
    // Associate tools with the event if provided
    if let Some(tools) = tool_ids {
        println!("DEBUG: tool_ids is Some. Length: {}", tools.len());
        if !tools.is_empty() {
            println!("DEBUG: Processing tool ids: {:?}", tools);
            db.add_tools_to_event(event_id, tools)
                .await
                .map_err(|e| {
                    println!("DEBUG: ERROR in add_tools_to_event: {}", e);
                    e.to_string()
                })?;
        } else {
            println!("DEBUG: tool_ids is Some but empty vector");
        }
    } else {
        println!("DEBUG: tool_ids is None");
    }
    
    Ok(event_id)
}

#[tauri::command]
pub async fn get_event_packages(
    db: State<'_, Database>,
    event_id: i64,
) -> Result<Vec<(i64, String, String)>, String> {
    db.get_event_packages(event_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_my_sessions(db: State<'_, Database>, user_id: i64) -> Result<Vec<Session>, String> {
    db.get_sessions_by_user(user_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_candidate(
    db: State<'_, Database>,
    username: String,
    password: String,
    _name: Option<String>
) -> Result<i64, String> {
    // Basic password hashing
    let hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;
    
    // We strictly create "participant" role here
    let role = "participant";
    
    // Note: Database::create_user takes (username, password_hash, role)
    // We might want to store 'name' somewhere? 
    // Checking User struct: id, username, role. No 'name' field in current User struct.
    // We'll stick to username for now, or maybe the user meant username IS the name.
    // If 'name' is distinct from 'username', we need a column for it.
    // For now I'll just use username.
    
    db.create_user(&username, &hash, role).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_test_results(db: State<'_, Database>) -> Result<Vec<crate::db::models::TestResultDTO>, String> {
    db.get_all_test_results().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn submit_test_results(
    db: State<'_, Database>,
    session_id: i64,
    scores: serde_json::Value,
    interpretations: Option<serde_json::Value>
) -> Result<i64, String> {
    println!("DEBUG: Submitting test results for session {}", session_id);
    let inter = interpretations.unwrap_or(json!({}));
    db.create_report(session_id, scores, inter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_test_results(
    db: State<'_, Database>,
    result_ids: Vec<i64>
) -> Result<(), String> {
    db.delete_test_results(result_ids)
        .await
        .map_err(|e| e.to_string())
}
