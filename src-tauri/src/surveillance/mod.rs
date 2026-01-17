pub mod window;

use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

use base64::{Engine as _, engine::general_purpose};
use std::io::Cursor;
use image::{ImageBuffer, Rgb, ImageOutputFormat};
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

// Session tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSession {
    pub session_id: String,
    pub user_id: i64,
    pub username: String,
    pub event_id: Option<i64>,
    pub event_name: Option<String>,
    pub test_name: String,
    pub started_at: u64,
    pub last_frame: Option<String>,
    pub last_update: u64,
}

// Global session store
lazy_static::lazy_static! {
    static ref ACTIVE_SESSIONS: Mutex<HashMap<String, ActiveSession>> = Mutex::new(HashMap::new());
}

#[tauri::command]
pub fn check_camera_permission() -> bool {
    true 
}

#[tauri::command]
pub async fn capture_frame() -> Result<String, String> {
    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = Camera::new(index, requested).map_err(|e| e.to_string())?;
    camera.open_stream().map_err(|e| e.to_string())?;
    
    let frame = camera.frame().map_err(|e| e.to_string())?;
    camera.stop_stream().map_err(|e| e.to_string())?; 

    // Manually construct ImageBuffer using image 0.24 to avoid version conflict with nokhwa's image 0.25
    let width = frame.resolution().width_x;
    let height = frame.resolution().height_y;
    let raw_data = frame.buffer().to_vec();

    let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, raw_data)
        .ok_or("Failed to create image buffer")?;

    let mut image_data = Vec::new();
    let mut cursor = Cursor::new(&mut image_data);
    
    // image 0.24 uses ImageOutputFormat
    buffer.write_to(&mut cursor, ImageOutputFormat::Jpeg(80))
        .map_err(|e| e.to_string())?;

    let base64_string = general_purpose::STANDARD.encode(&image_data);
    
    Ok(base64_string)
}

// Register a new active session
#[tauri::command]
pub fn register_session(
    session_id: String,
    user_id: i64,
    username: String,
    event_id: Option<i64>,
    event_name: Option<String>,
    test_name: String,
) -> Result<(), String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let session = ActiveSession {
        session_id: session_id.clone(),
        user_id,
        username,
        event_id,
        event_name,
        test_name,
        started_at: now,
        last_frame: None,
        last_update: now,
    };

    let mut sessions = ACTIVE_SESSIONS.lock().map_err(|e| e.to_string())?;
    sessions.insert(session_id, session);
    
    Ok(())
}

// Update session with new frame
#[tauri::command]
pub async fn update_session_frame(session_id: String) -> Result<(), String> {
    let frame = capture_frame().await?;
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let mut sessions = ACTIVE_SESSIONS.lock().map_err(|e| e.to_string())?;
    if let Some(session) = sessions.get_mut(&session_id) {
        session.last_frame = Some(frame);
        session.last_update = now;
    }
    
    Ok(())
}

// Get all active sessions
#[tauri::command]
pub fn get_active_sessions() -> Result<Vec<ActiveSession>, String> {
    let sessions = ACTIVE_SESSIONS.lock().map_err(|e| e.to_string())?;
    Ok(sessions.values().cloned().collect())
}

// Get frame for specific session
#[tauri::command]
pub fn get_session_frame(session_id: String) -> Result<Option<String>, String> {
    let sessions = ACTIVE_SESSIONS.lock().map_err(|e| e.to_string())?;
    Ok(sessions.get(&session_id).and_then(|s| s.last_frame.clone()))
}

// Remove session (when test ends)
#[tauri::command]
pub fn unregister_session(session_id: String) -> Result<(), String> {
    let mut sessions = ACTIVE_SESSIONS.lock().map_err(|e| e.to_string())?;
    sessions.remove(&session_id);
    Ok(())
}

// Cleanup stale sessions (not updated in 5 minutes)
#[tauri::command]
pub fn cleanup_stale_sessions() -> Result<usize, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let mut sessions = ACTIVE_SESSIONS.lock().map_err(|e| e.to_string())?;
    let before_count = sessions.len();
    
    sessions.retain(|_, session| {
        now - session.last_update < 300 // 5 minutes
    });
    
    let removed = before_count - sessions.len();
    Ok(removed)
}
