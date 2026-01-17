use tauri::State;
use crate::db::Database;
use crate::db::models::User;
use bcrypt::{hash, verify, DEFAULT_COST};

#[tauri::command]
pub async fn register_user(
    db: State<'_, Database>,
    username: String,
    password: String,
    role: String
) -> Result<String, String> {
    // Hash password
    let password_hash = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;

    // Create user in DB
    db.create_user(&username, &password_hash, &role)
        .await
        .map_err(|e| e.to_string())?;

    Ok("User registered successfully".to_string())
}

#[tauri::command]
pub async fn login_user(
    db: State<'_, Database>,
    username: String,
    password: String
) -> Result<User, String> {
    // Get user
    println!("DEBUG: Attempting login for user: '{}'", username);
    
    let user = db.get_user_by_username(&username)
        .await
        .map_err(|e| {
            println!("DEBUG: User not found error: {}", e);
            "User not found".to_string()
        })?;

    println!("DEBUG: User found: id={}, role={}", user.id, user.role);

    // Verify password
    let hash = user.password_hash.as_ref().ok_or("Invalid user data")?;
    // println!("DEBUG: Hash found: {}", hash); // Don't log actual hashes in prod, but ok for local debug
    
    let valid = verify(&password, hash).map_err(|e| {
        println!("DEBUG: Bcrypt verify error: {}", e);
        e.to_string()
    })?;

    if valid {
        println!("DEBUG: Password verified successfully");
        Ok(user)
    } else {
        println!("DEBUG: Password verification failed");
        Err("Invalid password".to_string())
    }
}

#[tauri::command]
pub async fn update_avatar(
    db: State<'_, Database>,
    username: String,
    avatar_url: String
) -> Result<String, String> {
    db.update_user_avatar(&username, Some(avatar_url))
        .await
        .map_err(|e| e.to_string())?;
    Ok("Avatar updated successfully".to_string())
}

#[tauri::command]
pub async fn get_user_profile(
    db: State<'_, Database>,
    username: String
) -> Result<User, String> {
    let user = db.get_user_by_username(&username)
        .await
        .map_err(|e| e.to_string())?;
    Ok(user)
}
