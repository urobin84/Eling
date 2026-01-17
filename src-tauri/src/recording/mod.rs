use std::fs;
use std::path::PathBuf;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub session_id: String,
    pub user_id: i64,
    pub username: String,
    pub test_name: String,
    pub event_name: Option<String>,
    pub started_at: u64,
    pub ended_at: Option<u64>,
    pub duration: Option<u64>,
    pub camera_file: Option<String>,
    pub screen_file: Option<String>,
    pub camera_size: Option<u64>,
    pub screen_size: Option<u64>,
}

fn get_recordings_dir() -> PathBuf {
    let path = PathBuf::from("recordings");
    if !path.exists() {
        fs::create_dir_all(&path).ok();
    }
    path
}

fn get_session_dir(session_id: &str) -> PathBuf {
    let mut path = get_recordings_dir();
    path.push(session_id);
    if !path.exists() {
        fs::create_dir_all(&path).ok();
    }
    path
}

#[tauri::command]
pub fn save_camera_recording(session_id: String, video_data: String) -> Result<String, String> {
    let session_dir = get_session_dir(&session_id);
    let mut camera_path = session_dir.clone();
    camera_path.push("camera.webm");

    // Decode base64
    let video_bytes = general_purpose::STANDARD
        .decode(video_data)
        .map_err(|e| format!("Failed to decode video data: {}", e))?;

    // Write to file
    fs::write(&camera_path, video_bytes)
        .map_err(|e| format!("Failed to write camera recording: {}", e))?;

    Ok(camera_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn save_screen_recording(session_id: String, video_data: String) -> Result<String, String> {
    let session_dir = get_session_dir(&session_id);
    let mut screen_path = session_dir.clone();
    screen_path.push("screen.webm");

    // Decode base64
    let video_bytes = general_purpose::STANDARD
        .decode(video_data)
        .map_err(|e| format!("Failed to decode video data: {}", e))?;

    // Write to file
    fs::write(&screen_path, video_bytes)
        .map_err(|e| format!("Failed to write screen recording: {}", e))?;

    Ok(screen_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn save_recording_metadata(metadata: RecordingMetadata) -> Result<(), String> {
    let session_dir = get_session_dir(&metadata.session_id);
    let mut metadata_path = session_dir.clone();
    metadata_path.push("metadata.json");

    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

    fs::write(&metadata_path, json)
        .map_err(|e| format!("Failed to write metadata: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_session_recordings(session_id: String) -> Result<RecordingMetadata, String> {
    let session_dir = get_session_dir(&session_id);
    let mut metadata_path = session_dir.clone();
    metadata_path.push("metadata.json");

    if !metadata_path.exists() {
        return Err("Recording metadata not found".to_string());
    }

    let json = fs::read_to_string(&metadata_path)
        .map_err(|e| format!("Failed to read metadata: {}", e))?;

    let metadata: RecordingMetadata = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse metadata: {}", e))?;

    Ok(metadata)
}

#[tauri::command]
pub fn get_all_recordings() -> Result<Vec<RecordingMetadata>, String> {
    let recordings_dir = get_recordings_dir();
    let mut recordings = Vec::new();

    if let Ok(entries) = fs::read_dir(recordings_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let session_id = entry.file_name().to_string_lossy().to_string();
                if let Ok(metadata) = get_session_recordings(session_id) {
                    recordings.push(metadata);
                }
            }
        }
    }

    Ok(recordings)
}

#[tauri::command]
pub fn delete_session_recording(session_id: String) -> Result<(), String> {
    let session_dir = get_session_dir(&session_id);
    
    if session_dir.exists() {
        fs::remove_dir_all(&session_dir)
            .map_err(|e| format!("Failed to delete recording: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_recording_video_path(session_id: String, video_type: String) -> Result<String, String> {
    let session_dir = get_session_dir(&session_id);
    let mut video_path = session_dir.clone();
    
    match video_type.as_str() {
        "camera" => video_path.push("camera.webm"),
        "screen" => video_path.push("screen.webm"),
        _ => return Err("Invalid video type".to_string()),
    }

    if !video_path.exists() {
        return Err(format!("{} recording not found", video_type));
    }

    Ok(video_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn cleanup_old_recordings(days: u64) -> Result<usize, String> {
    let recordings_dir = get_recordings_dir();
    let mut deleted_count = 0;
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    
    let cutoff = now - (days * 24 * 60 * 60);

    if let Ok(entries) = fs::read_dir(recordings_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let session_id = entry.file_name().to_string_lossy().to_string();
                if let Ok(metadata) = get_session_recordings(session_id.clone()) {
                    if metadata.started_at < cutoff {
                        if delete_session_recording(session_id).is_ok() {
                            deleted_count += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(deleted_count)
}
