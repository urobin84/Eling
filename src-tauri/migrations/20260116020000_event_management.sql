-- Migration: Event Management System
-- Add event codes and participant management

-- Add event_code column to events table (without UNIQUE constraint in ALTER TABLE)
-- SQLite doesn't support adding UNIQUE columns to existing tables
ALTER TABLE events ADD COLUMN event_code TEXT DEFAULT NULL;
ALTER TABLE events ADD COLUMN max_participants INTEGER DEFAULT NULL;
ALTER TABLE events ADD COLUMN enrollment_deadline TEXT DEFAULT NULL;

-- Create unique index for event code (handles uniqueness)
CREATE UNIQUE INDEX IF NOT EXISTS idx_events_code_unique ON events(event_code) WHERE event_code IS NOT NULL;

-- Create event_participants junction table
CREATE TABLE IF NOT EXISTS event_participants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'enrolled', -- enrolled, in_progress, completed, withdrawn
    enrolled_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TEXT DEFAULT NULL,
    notes TEXT DEFAULT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(event_id, user_id) -- One enrollment per user per event
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_event_participants_event 
ON event_participants(event_id);

CREATE INDEX IF NOT EXISTS idx_event_participants_user 
ON event_participants(user_id);

CREATE INDEX IF NOT EXISTS idx_event_participants_status 
ON event_participants(event_id, status);
