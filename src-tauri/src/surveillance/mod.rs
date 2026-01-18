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
    // Try to access camera, but fail gracefully if busy (e.g., browser is using it)
    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = match Camera::new(index, requested) {
        Ok(cam) => cam,
        Err(e) => {
            // Camera is likely in use by browser - this is expected during tests
            return Err(format!("Camera busy or unavailable: {}", e));
        }
    };
    
    if let Err(e) = camera.open_stream() {
        return Err(format!("Cannot open camera stream (likely in use): {}", e));
    }
    
    let frame = match camera.frame() {
        Ok(f) => f,
        Err(e) => {
            let _ = camera.stop_stream(); // Clean up
            return Err(format!("Cannot capture frame: {}", e));
        }
    };
    
    let _ = camera.stop_stream(); // Always clean up

    // Get the actual frame format and data
    let width = frame.resolution().width_x;
    let height = frame.resolution().height_y;
    
    // Use nokhwa's built-in decoder to convert to RGB
    // This handles whatever format the camera provides (YUV, YUYV, etc.)
    let decoded = frame.decode_image::<RgbFormat>()
        .map_err(|e| format!("Failed to decode camera frame: {}", e))?;
    
    // Now we have proper RGB data
    let rgb_data = decoded.into_raw();
    
    // Validate RGB buffer size
    let expected_size = (width * height * 3) as usize;
    if rgb_data.len() != expected_size {
        return Err(format!(
            "RGB buffer size mismatch: expected {} bytes ({}x{}x3), got {} bytes",
            expected_size, width, height, rgb_data.len()
        ));
    }

    let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, rgb_data)
        .ok_or_else(|| format!("Failed to create image buffer ({}x{} RGB)", width, height))?;

    let mut image_data = Vec::new();
    let mut cursor = Cursor::new(&mut image_data);
    
    buffer.write_to(&mut cursor, ImageOutputFormat::Jpeg(80))
        .map_err(|e| format!("JPEG encoding failed: {}", e))?;

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
