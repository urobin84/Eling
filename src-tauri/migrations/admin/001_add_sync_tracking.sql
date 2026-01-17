-- Admin App Database Schema Extensions
-- Add sync tracking to existing tables

-- Add sync tracking columns to test_results
ALTER TABLE test_results ADD COLUMN IF NOT EXISTS received_at TEXT;
ALTER TABLE test_results ADD COLUMN IF NOT EXISTS client_session_id TEXT;
ALTER TABLE test_results ADD COLUMN IF NOT EXISTS sync_source TEXT DEFAULT 'direct'; -- 'direct' or 'client_sync'

-- Add sync tracking to recordings
ALTER TABLE recordings ADD COLUMN IF NOT EXISTS received_at TEXT;
ALTER TABLE recordings ADD COLUMN IF NOT EXISTS client_session_id TEXT;

-- Create sync log table for tracking all syncs from clients
CREATE TABLE IF NOT EXISTS sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_session_id TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    data_type TEXT NOT NULL,
    received_at TEXT NOT NULL DEFAULT (datetime('now')),
    payload_size INTEGER,
    client_ip TEXT,
    status TEXT DEFAULT 'success' CHECK(status IN ('success', 'partial', 'failed')),
    error_message TEXT
);

-- Create client connections table for tracking active clients
CREATE TABLE IF NOT EXISTS client_connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    client_identifier TEXT,
    last_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
    ip_address TEXT,
    status TEXT DEFAULT 'active' CHECK(status IN ('active', 'inactive', 'blocked'))
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_test_results_client_session ON test_results(client_session_id);
CREATE INDEX IF NOT EXISTS idx_sync_log_session ON sync_log(client_session_id);
CREATE INDEX IF NOT EXISTS idx_sync_log_user ON sync_log(user_id);
CREATE INDEX IF NOT EXISTS idx_client_connections_user ON client_connections(user_id);
