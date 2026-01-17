// Admin Database Extensions
// Additional methods for sync tracking

use sqlx::{Error, Row};
use serde::{Serialize, Deserialize};

use super::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncLogEntry {
    pub id: i64,
    pub client_session_id: String,
    pub user_id: i64,
    pub data_type: String,
    pub received_at: String,
    pub payload_size: Option<i64>,
    pub client_ip: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientConnection {
    pub id: i64,
    pub user_id: i64,
    pub client_identifier: Option<String>,
    pub last_seen_at: String,
    pub ip_address: Option<String>,
    pub status: String,
}

impl Database {
    // ===== Sync Log =====

    pub async fn log_sync(
        &self,
        client_session_id: &str,
        user_id: i64,
        data_type: &str,
        payload_size: Option<i64>,
        client_ip: Option<&str>,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO sync_log 
             (client_session_id, user_id, data_type, payload_size, client_ip, status, error_message) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(client_session_id)
        .bind(user_id)
        .bind(data_type)
        .bind(payload_size)
        .bind(client_ip)
        .bind(status)
        .bind(error_message)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_sync_logs(&self, user_id: Option<i64>, limit: i64) -> Result<Vec<SyncLogEntry>, Error> {
        let logs = if let Some(uid) = user_id {
            sqlx::query_as::<_, SyncLogEntry>(
                "SELECT * FROM sync_log WHERE user_id = ? ORDER BY received_at DESC LIMIT ?"
            )
            .bind(uid)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, SyncLogEntry>(
                "SELECT * FROM sync_log ORDER BY received_at DESC LIMIT ?"
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(logs)
    }

    // ===== Client Connections =====

    pub async fn update_client_connection(
        &self,
        user_id: i64,
        client_identifier: Option<&str>,
        ip_address: Option<&str>,
    ) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO client_connections (user_id, client_identifier, ip_address) 
             VALUES (?, ?, ?) 
             ON CONFLICT(user_id) DO UPDATE SET 
                last_seen_at = datetime('now'),
                ip_address = excluded.ip_address"
        )
        .bind(user_id)
        .bind(client_identifier)
        .bind(ip_address)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_active_clients(&self) -> Result<Vec<ClientConnection>, Error> {
        let clients = sqlx::query_as::<_, ClientConnection>(
            "SELECT * FROM client_connections 
             WHERE status = 'active' 
             AND datetime(last_seen_at) > datetime('now', '-5 minutes')
             ORDER BY last_seen_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }

    // ===== Test Results with Sync Tracking =====

    pub async fn save_synced_test_result(
        &self,
        client_session_id: &str,
        user_id: i64,
        event_id: i64,
        answers: &str,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO test_results 
             (user_id, event_id, answers, client_session_id, sync_source, received_at) 
             VALUES (?, ?, ?, ?, 'client_sync', datetime('now'))"
        )
        .bind(user_id)
        .bind(event_id)
        .bind(answers)
        .bind(client_session_id)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }
}

// Implement FromRow for sync types
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for SyncLogEntry {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            client_session_id: row.try_get("client_session_id")?,
            user_id: row.try_get("user_id")?,
            data_type: row.try_get("data_type")?,
            received_at: row.try_get("received_at")?,
            payload_size: row.try_get("payload_size")?,
            client_ip: row.try_get("client_ip")?,
            status: row.try_get("status")?,
            error_message: row.try_get("error_message")?,
        })
    }
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for ClientConnection {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            client_identifier: row.try_get("client_identifier")?,
            last_seen_at: row.try_get("last_seen_at")?,
            ip_address: row.try_get("ip_address")?,
            status: row.try_get("status")?,
        })
    }
}
