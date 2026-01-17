use tauri::State;
use crate::db::Database;
use crate::db::models::{User, Event, Session};
use bcrypt;
use serde_json::json;

#[tauri::command]
pub async fn generate_ai_review(db: State<'_, Database>, result_id: i64) -> Result<String, String> {
    // 1. Fetch Report Data
    let report = db.get_report_by_id(result_id).await.map_err(|e| e.to_string())?;
    
    // 2. Fetch Context (Candidate Name, Tool, etc.) - We might need to join queries or just use what we have.
    // get_report_by_id only gives raw report. We can use the scores JSON.
    // For a better prompt, we'd ideally want candidate name and tool name, but report has session_id.
    // Optimization: Just analyze the scores for now.
    
    let scores = &report.scores;
    
    // Construct Prompt
    let prompt = format!(
        "You are an expert psychological assessment assistant. Analyze the following test scores and provide a concise, professional interpretation for the candidate. Focus on strengths and areas for development.\n\nScores: {}\n\nInterpretation:",
        scores
    );

    // 3. Call Ollama (gemma2:2b)
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": "gemma2:2b",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to AI engine: {}", e))?;
        
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
    description: Option<String>
) -> Result<i64, String> {
    db.create_event(&name, description)
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
