use eling_lib::db::{self, Database};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use std::env;
use std::path::PathBuf;
use bcrypt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting database reset and seed process...");

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let mac_app_data_path = PathBuf::from(format!("{}/Library/Application Support/com.eling.psychotest/eling.db", home));
    let mut db_path = PathBuf::from("eling.db");
    
    if mac_app_data_path.exists() {
        println!("Found database at: {:?}", mac_app_data_path);
        db_path = mac_app_data_path;
    }
    
    println!("Target database: {:?}", db_path);

    // 1. Delete existing database file
    if db_path.exists() {
        println!("Deleting existing database file...");
        std::fs::remove_file(&db_path)?;
        println!("Database file deleted.");
    }

    // 2. Connect (creates new file)
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    println!("Connected to new database.");

    // 3. Run migrations
    println!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    println!("Migrations complete.");

    // 4. Initialize Database struct
    let database = Database::new(pool.clone());

    // 5. Seed Admin
    println!("Seeding admin user...");
    let hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST)?;
    // We can directly insert since it is fresh DB
    database.create_user("admin", &hash, "admin").await?;

    // 6. Seed Tools
    println!("Seeding tools...");
    db::seed::seed_tools(&database).await?;

    // 7. Seed Dummy Results
    println!("Seeding dummy results...");
    db::seed::seed_dummy_results(&database).await?;

    println!("Database reset and seeding completed successfully.");

    Ok(())
}
