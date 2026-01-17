-- Candidate App Local Database Schema
-- This database stores temporary data during test execution
-- Data is synced to admin server after test completion

-- Local test sessions (active tests)
CREATE TABLE IF NOT EXISTS local_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT UNIQUE NOT NULL,
    event_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    event_code TEXT,
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    status TEXT DEFAULT 'in_progress' CHECK(status IN ('in_progress', 'completed', 'synced', 'failed')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Local answers (temporary storage)
CREATE TABLE IF NOT EXISTS local_answers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    question_id INTEGER NOT NULL,
    answer TEXT,
    answered_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (session_id) REFERENCES local_sessions(session_id) ON DELETE CASCADE
);

-- Sync queue for async data submission
CREATE TABLE IF NOT EXISTS sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    data_type TEXT NOT NULL CHECK(data_type IN ('test_result', 'recording', 'metadata')),
    session_id TEXT NOT NULL,
    payload TEXT NOT NULL, -- JSON data
    priority INTEGER DEFAULT 5, -- 1=highest, 10=lowest
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    synced BOOLEAN DEFAULT 0,
    synced_at TEXT,
    sync_attempts INTEGER DEFAULT 0,
    last_attempt_at TEXT,
    last_error TEXT,
    FOREIGN KEY (session_id) REFERENCES local_sessions(session_id) ON DELETE CASCADE
);

-- Cached event information (downloaded from server)
CREATE TABLE IF NOT EXISTS events_cache (
    id INTEGER PRIMARY KEY,
    event_code TEXT UNIQUE NOT NULL,
    event_name TEXT NOT NULL,
    description TEXT,
    status TEXT,
    cached_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT
);

-- Recording metadata (local)
CREATE TABLE IF NOT EXISTS local_recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    recording_type TEXT NOT NULL CHECK(recording_type IN ('camera', 'screen')),
    file_path TEXT NOT NULL,
    file_size INTEGER,
    duration INTEGER, -- seconds
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    uploaded BOOLEAN DEFAULT 0,
    uploaded_at TEXT,
    FOREIGN KEY (session_id) REFERENCES local_sessions(session_id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_local_sessions_status ON local_sessions(status);
CREATE INDEX IF NOT EXISTS idx_local_sessions_user ON local_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_local_answers_session ON local_answers(session_id);
CREATE INDEX IF NOT EXISTS idx_sync_queue_synced ON sync_queue(synced, priority);
CREATE INDEX IF NOT EXISTS idx_sync_queue_session ON sync_queue(session_id);
CREATE INDEX IF NOT EXISTS idx_local_recordings_session ON local_recordings(session_id);
CREATE INDEX IF NOT EXISTS idx_local_recordings_uploaded ON local_recordings(uploaded);
