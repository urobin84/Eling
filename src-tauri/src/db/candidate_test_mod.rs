// Candidate Database Manager
// Handles local SQLite database for candidate app

use sqlx::{SqlitePool, Error, Row};
use serde::{Serialize, Deserialize};

// ... (existing code remains the same) ...

#[cfg(test)]
#[path = "candidate_tests.rs"]
mod candidate_tests;
