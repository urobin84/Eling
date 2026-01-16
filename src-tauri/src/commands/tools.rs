use tauri::State;
use crate::db::Database;
use crate::db::models::{Tool, ToolSubtest, Question};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FullToolStructure {
    pub tool: Tool,
    pub subtests: Vec<FullSubtest>,
}

#[derive(Serialize, Deserialize)]
pub struct FullSubtest {
    pub subtest: ToolSubtest,
    pub questions: Vec<Question>,
}

#[tauri::command]
pub async fn get_tool_structure(db: State<'_, Database>, tool_id: i64) -> Result<FullToolStructure, String> {
    let tool = db.get_tool_by_id(tool_id).await.map_err(|e| e.to_string())?;
    let subtests = db.get_subtests_by_tool(tool_id).await.map_err(|e| e.to_string())?;

    let mut full_subtests = Vec::new();

    for subtest in subtests {
        let questions = db.get_questions_by_subtest(subtest.id).await.map_err(|e| e.to_string())?;
        full_subtests.push(FullSubtest {
            subtest,
            questions,
        });
    }

    Ok(FullToolStructure {
        tool,
        subtests: full_subtests,
    })
}

#[tauri::command]
pub async fn create_subtest(
    db: State<'_, Database>, 
    tool_id: i64, 
    name: String, 
    sequence: i64, 
    time_limit: Option<i64>
) -> Result<i64, String> {
    db.create_subtest(tool_id, &name, sequence, time_limit).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_subtest(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db.delete_subtest(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_question(
    db: State<'_, Database>,
    subtest_id: i64,
    text: String,
    q_type: String,
    options: serde_json::Value,
    sequence: i64
) -> Result<i64, String> {
    db.create_question(subtest_id, &text, &q_type, options, sequence).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_question(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db.delete_question(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_question(
    db: State<'_, Database>,
    id: i64,
    text: String,
    q_type: String,
    options: serde_json::Value
) -> Result<(), String> {
    db.update_question(id, &text, &q_type, options).await.map_err(|e| e.to_string())
}
