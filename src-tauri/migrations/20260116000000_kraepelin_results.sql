-- Migration: Create kraepelin_results table
-- This table stores detailed results for each column in a Kraepelin test session

CREATE TABLE IF NOT EXISTS kraepelin_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    column_index INTEGER NOT NULL,
    answers TEXT NOT NULL, -- JSON array of answers: [5, 7, 3, 8, ...]
    correct_count INTEGER NOT NULL,
    total_questions INTEGER NOT NULL,
    time_taken INTEGER NOT NULL, -- Time in seconds to complete this column
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES test_sessions(id) ON DELETE CASCADE
);

-- Index for efficient session lookups
CREATE INDEX IF NOT EXISTS idx_kraepelin_session 
ON kraepelin_results(session_id);

-- Index for efficient column lookups within a session
CREATE INDEX IF NOT EXISTS idx_kraepelin_column 
ON kraepelin_results(session_id, column_index);
