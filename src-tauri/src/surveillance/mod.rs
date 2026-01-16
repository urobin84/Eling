pub mod window;

use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

use base64::{Engine as _, engine::general_purpose};
use std::io::Cursor;
use image::{ImageBuffer, Rgb, ImageOutputFormat};

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
    
    Ok(format!("data:image/jpeg;base64,{}", base64_string))
}
