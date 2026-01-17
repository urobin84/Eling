// Server configuration and control commands

use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::api_server::ApiServer;
use crate::db::Database;

#[derive(Clone)]
pub struct ServerState {
    pub is_running: Arc<Mutex<bool>>,
    pub port: Arc<Mutex<u16>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            port: Arc::new(Mutex::new(8080)),
        }
    }
}

/// Start API server (Admin only)
#[tauri::command]
pub async fn start_api_server(
    port: u16,
    state: State<'_, ServerState>,
    db_state: State<'_, Arc<Database>>,
) -> Result<String, String> {
    let mut is_running = state.is_running.lock().await;
    
    if *is_running {
        return Err("Server is already running".to_string());
    }

    // Update port
    *state.port.lock().await = port;

    // Clone database for server
    let db = (**db_state).clone();
    
    // Start server in background
    tokio::spawn(async move {
        let server = ApiServer::new(db, port);
        if let Err(e) = server.start().await {
            eprintln!("API Server error: {:?}", e);
        }
    });

    *is_running = true;

    Ok(format!("API Server started on port {}", port))
}

/// Stop API server
#[tauri::command]
pub async fn stop_api_server(
    state: State<'_, ServerState>,
) -> Result<String, String> {
    let mut is_running = state.is_running.lock().await;
    
    if !*is_running {
        return Err("Server is not running".to_string());
    }

    // TODO: Implement graceful shutdown
    *is_running = false;

    Ok("API Server stopped".to_string())
}

/// Get server status
#[tauri::command]
pub async fn get_server_status(
    state: State<'_, ServerState>,
) -> Result<serde_json::Value, String> {
    let is_running = *state.is_running.lock().await;
    let port = *state.port.lock().await;

    Ok(serde_json::json!({
        "running": is_running,
        "port": port,
        "url": format!("http://localhost:{}", port)
    }))
}

/// Get local IP addresses
#[tauri::command]
pub fn get_local_ips() -> Result<Vec<String>, String> {
    use std::net::UdpSocket;
    
    let mut ips = Vec::new();
    
    // Try to get local IP by connecting to external address
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if let Ok(()) = socket.connect("8.8.8.8:80") {
            if let Ok(addr) = socket.local_addr() {
                ips.push(addr.ip().to_string());
            }
        }
    }
    
    // Add localhost
    ips.push("127.0.0.1".to_string());
    
    Ok(ips)
}
