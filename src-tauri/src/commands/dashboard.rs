use tauri::State;
use crate::db::Database;
use crate::db::models::{User, Event, Session};
use bcrypt;

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
