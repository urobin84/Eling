// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tauri::Manager;
use std::fs;
use std::str::FromStr;
use bcrypt;

mod db;
mod commands;
mod tests;

pub mod tools {
    pub use crate::commands::tools::*;
}

use db::Database;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn check_db_health(_db: tauri::State<'_, Database>) -> Result<String, String> {
    // We can add a simple health check method to Database struct later
    // For now just return OK if state exists
    Ok("Database is healthy".to_string())
}

mod surveillance;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
                fs::create_dir_all(&app_dir).expect("failed to create app data dir");
                let db_path = app_dir.join("eling.db");
                let db_url = format!("sqlite:{}", db_path.to_string_lossy());

                let options = SqliteConnectOptions::from_str(&db_url)
                    .expect("failed to parse db url")
                    .create_if_missing(true);

                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect_with(options)
                    .await
                    .expect("failed to connect to database");

                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("failed to run migrations");

                let database = Database::new(pool);

                // Seed default admin
                let hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap_or_default();
                
                // Always try to update admin password to ensure it is correct
                let _ = database.update_user_password("admin", &hash).await;

                // Create if not exists
                if let Err(_) = database.get_user_by_username("admin").await {
                    let _ = database.create_user("admin", &hash, "admin").await;
                }

                // Seed Tools if empty
                if let Err(e) = db::seed::seed_tools(&database).await {
                    eprintln!("Failed to seed tools: {}", e);
                }

                // Seed Dummy Results (Force for Dev)
                if let Err(e) = db::seed::seed_dummy_results(&database).await {
                    eprintln!("Failed to seed dummy results: {}", e);
                }

                app.manage(database);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            check_db_health,
            commands::get_tools,
            commands::create_session,
            commands::auth::register_user,
            commands::auth::login_user,
            commands::dashboard::get_all_users,
            commands::dashboard::get_events,
            commands::dashboard::create_event,
            commands::dashboard::create_candidate,
            commands::dashboard::get_my_sessions,
            commands::dashboard::get_test_results,
            commands::tools::get_tool_structure,
            commands::tools::create_subtest,
            commands::tools::delete_subtest,
            commands::tools::create_question,
            commands::tools::delete_question,
            commands::tools::update_question,
            commands::notifications::get_notifications,
            commands::notifications::mark_notification_read,
            commands::notifications::mark_all_notifications_read,
            commands::notifications::create_test_completion_notification,
            commands::kraepelin::create_kraepelin_session,
            commands::kraepelin::save_kraepelin_column,
            commands::kraepelin::complete_kraepelin_session,
            commands::events::get_event_details,
            commands::events::get_event_participants,
            commands::events::enroll_candidate_to_event,
            commands::events::add_participant_to_event,
            commands::events::remove_participant_from_event,
            commands::events::get_my_events,
            commands::events::generate_event_code_cmd,
            surveillance::check_camera_permission,
            surveillance::capture_frame,
            surveillance::window::set_kiosk_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
