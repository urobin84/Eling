use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};

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

fn get_recordings_dir(app_handle: &AppHandle) -> PathBuf {
    let app_dir = app_handle.path().app_data_dir().expect("failed to get app data dir");
    let path = app_dir.join("recordings");
    if !path.exists() {
        fs::create_dir_all(&path).ok();
    }
    path
}

fn get_session_dir(app_handle: &AppHandle, session_id: &str) -> PathBuf {
    let mut path = get_recordings_dir(app_handle);
    path.push(session_id);
    if !path.exists() {
        fs::create_dir_all(&path).ok();
    }
    path
}

#[tauri::command]
pub fn save_camera_recording(app_handle: AppHandle, session_id: String, video_data: Vec<u8>) -> Result<String, String> {
    println!("DEBUG: save_camera_recording called for session: {}, data size: {} bytes", session_id, video_data.len());
    let session_dir = get_session_dir(&app_handle, &session_id);
    let mut camera_path = session_dir.clone();
    camera_path.push("camera.webm");

    // Write to file
    fs::write(&camera_path, video_data)
        .map_err(|e| format!("Failed to write camera recording: {}", e))?;
    
    println!("DEBUG: Camera recording saved to {:?}", camera_path);
    Ok(camera_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn save_screen_recording(app_handle: AppHandle, session_id: String, video_data: Vec<u8>) -> Result<String, String> {
    println!("DEBUG: save_screen_recording called for session: {}, data size: {} bytes", session_id, video_data.len());
    let session_dir = get_session_dir(&app_handle, &session_id);
    let mut screen_path = session_dir.clone();
    screen_path.push("screen.webm");

    // Write to file
    fs::write(&screen_path, video_data)
        .map_err(|e| format!("Failed to write screen recording: {}", e))?;

    println!("DEBUG: Screen recording saved to {:?}", screen_path);
    Ok(screen_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn save_recording_metadata(app_handle: AppHandle, metadata: RecordingMetadata) -> Result<(), String> {
    println!("DEBUG: save_recording_metadata called for session: {}", metadata.session_id);
    let session_dir = get_session_dir(&app_handle, &metadata.session_id);
    let mut metadata_path = session_dir.clone();
    metadata_path.push("metadata.json");

    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

    fs::write(&metadata_path, json)
        .map_err(|e| format!("Failed to write metadata: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_session_recordings(app_handle: AppHandle, session_id: String) -> Result<RecordingMetadata, String> {
    let session_dir = get_session_dir(&app_handle, &session_id);
    let mut metadata_path = session_dir.clone();
    metadata_path.push("metadata.json");

    if metadata_path.exists() {
        let json = fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?;

        let mut metadata: RecordingMetadata = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse metadata: {}", e))?;

        // Double check sizes if they are 0 (likely from old buggy version or pending)
        let mut camera_path = session_dir.clone();
        camera_path.push("camera.webm");
        let mut screen_path = session_dir.clone();
        screen_path.push("screen.webm");

        if metadata.camera_size.unwrap_or(0) == 0 && camera_path.exists() {
            metadata.camera_size = fs::metadata(&camera_path).ok().map(|m| m.len());
        }
        if metadata.screen_size.unwrap_or(0) == 0 && screen_path.exists() {
            metadata.screen_size = fs::metadata(&screen_path).ok().map(|m| m.len());
        }

        return Ok(metadata);
    }

    // Fallback: If metadata.json is missing, check if video files exist
    let mut camera_path = session_dir.clone();
    camera_path.push("camera.webm");
    let mut screen_path = session_dir.clone();
    screen_path.push("screen.webm");

    if camera_path.exists() || screen_path.exists() {
        return Ok(RecordingMetadata {
            session_id: session_id.clone(),
            user_id: 0,
            username: "Unknown Candidate".to_string(),
            test_name: "Assessment".to_string(),
            event_name: None,
            started_at: 0,
            ended_at: None,
            duration: None,
            camera_file: if camera_path.exists() { Some("camera.webm".to_string()) } else { None },
            screen_file: if screen_path.exists() { Some("screen.webm".to_string()) } else { None },
            camera_size: fs::metadata(&camera_path).ok().map(|m| m.len()),
            screen_size: fs::metadata(&screen_path).ok().map(|m| m.len()),
        });
    }

    Err("Recording not found".to_string())
}

#[tauri::command]
pub fn get_all_recordings(app_handle: AppHandle) -> Result<Vec<RecordingMetadata>, String> {
    let recordings_dir = get_recordings_dir(&app_handle);
    let mut recordings = Vec::new();

    if let Ok(entries) = fs::read_dir(recordings_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let session_id = entry.file_name().to_string_lossy().to_string();
                if let Ok(metadata) = get_session_recordings(app_handle.clone(), session_id) {
                    recordings.push(metadata);
                }
            }
        }
    }

    Ok(recordings)
}

#[tauri::command]
pub fn delete_session_recording(app_handle: AppHandle, session_id: String) -> Result<(), String> {
    let session_dir = get_session_dir(&app_handle, &session_id);
    
    if session_dir.exists() {
        fs::remove_dir_all(&session_dir)
            .map_err(|e| format!("Failed to delete recording: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_recording_data(app_handle: AppHandle, session_id: String, video_type: String) -> Result<Vec<u8>, String> {
    let session_dir = get_session_dir(&app_handle, &session_id);
    let mut video_path = session_dir.clone();
    
    match video_type.as_str() {
        "camera" => video_path.push("camera.webm"),
        "screen" => video_path.push("screen.webm"),
        _ => return Err("Invalid video type".to_string()),
    }

    if !video_path.exists() {
        return Err(format!("{} recording not found", video_type));
    }

    fs::read(&video_path)
        .map_err(|e| format!("Failed to read video file: {}", e))
}

#[tauri::command]
pub fn get_recording_video_path(app_handle: AppHandle, session_id: String, video_type: String) -> Result<String, String> {
    let session_dir = get_session_dir(&app_handle, &session_id);
    let mut video_path = session_dir.clone();
    
    match video_type.as_str() {
        "camera" => video_path.push("camera.webm"),
        "screen" => video_path.push("screen.webm"),
        _ => return Err("Invalid video type".to_string()),
    }

    if !video_path.exists() {
        return Err(format!("{} recording not found", video_type));
    }

    // Return absolute path
    let abs_path = fs::canonicalize(&video_path)
        .map_err(|e| format!("Failed to get absolute path: {}", e))?;
    
    Ok(abs_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn cleanup_old_recordings(app_handle: AppHandle, days: u64) -> Result<usize, String> {
    let recordings_dir = get_recordings_dir(&app_handle);
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
                if let Ok(metadata) = get_session_recordings(app_handle.clone(), session_id.clone()) {
                    if metadata.started_at < cutoff {
                        if delete_session_recording(app_handle.clone(), session_id).is_ok() {
                            deleted_count += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(deleted_count)
}
