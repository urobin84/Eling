use tauri::{Window, Runtime};

#[tauri::command]
pub async fn set_kiosk_mode<R: Runtime>(window: Window<R>, enable: bool) -> Result<(), String> {
    window.set_fullscreen(enable).map_err(|e| e.to_string())?;
    window.set_always_on_top(enable).map_err(|e| e.to_string())?;
    Ok(())
}
