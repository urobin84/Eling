-- Initial Schema Creation

-- 2.1 Tools Table
CREATE TABLE IF NOT EXISTS tools (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    tool_type TEXT NOT NULL CHECK(tool_type IN ('choice', 'pair', 'speed', 'projective', 'leadership')),
    category TEXT NOT NULL CHECK(category IN ('cognitive', 'personality', 'performance', 'clinical', 'leadership', 'interest')),
    config JSON NOT NULL DEFAULT '{}',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tools_type ON tools(tool_type);
CREATE INDEX idx_tools_category ON tools(category);

-- 2.2 Tool Subtests Table
CREATE TABLE IF NOT EXISTS tool_subtests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tool_id INTEGER NOT NULL,
    subtest_name TEXT NOT NULL,
    time_limit_seconds INTEGER, -- NULL means untimed
    instructions JSON NOT NULL,
    question_count INTEGER NOT NULL DEFAULT 0,
    sequence_order INTEGER NOT NULL,
    FOREIGN KEY (tool_id) REFERENCES tools(id) ON DELETE CASCADE,
    UNIQUE(tool_id, sequence_order)
);

CREATE INDEX idx_subtests_tool ON tool_subtests(tool_id);

-- 2.3 Questions Table
CREATE TABLE IF NOT EXISTS questions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subtest_id INTEGER NOT NULL,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL CHECK(question_type IN ('multiple_choice', 'true_false', 'pair', 'text', 'drawing', 'kraepelin_column')),
    options JSON, -- For choice questions: ["A", "B", "C", "D"]
    media_url TEXT, -- For image/audio questions
    sequence_order INTEGER NOT NULL,
    FOREIGN KEY (subtest_id) REFERENCES tool_subtests(id) ON DELETE CASCADE,
    UNIQUE(subtest_id, sequence_order)
);

CREATE INDEX idx_questions_subtest ON questions(subtest_id);

-- 2.4 Answer Keys Table
CREATE TABLE IF NOT EXISTS answer_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tool_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,
    correct_answer TEXT NOT NULL, -- e.g., "B" or JSON for complex answers
    scoring_rule JSON, -- Custom scoring rules
    FOREIGN KEY (tool_id) REFERENCES tools(id) ON DELETE CASCADE,
    FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE,
    UNIQUE(question_id)
);

CREATE INDEX idx_answer_keys_tool ON answer_keys(tool_id);

-- 2.5 Packages Table
CREATE TABLE IF NOT EXISTS packages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tool_id INTEGER NOT NULL,
    package_name TEXT NOT NULL,
    version TEXT NOT NULL DEFAULT '1.0',
    content_data JSON NOT NULL, -- Complete package content (questions, keys, etc.)
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (tool_id) REFERENCES tools(id) ON DELETE CASCADE,
    UNIQUE(tool_id, package_name, version)
);

CREATE INDEX idx_packages_tool ON packages(tool_id);

-- 2.6 Events Table
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_name TEXT NOT NULL UNIQUE,
    description TEXT,
    status TEXT NOT NULL CHECK(status IN ('draft', 'active', 'archived')) DEFAULT 'draft',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_events_status ON events(status);

-- 2.7 Event Packages Table (Junction)
CREATE TABLE IF NOT EXISTS event_packages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    package_id INTEGER NOT NULL,
    sequence_order INTEGER NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE,
    UNIQUE(event_id, package_id),
    UNIQUE(event_id, sequence_order)
);

CREATE INDEX idx_event_packages_event ON event_packages(event_id);

-- 2.8 Users Table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT UNIQUE,
    role TEXT NOT NULL CHECK(role IN ('admin', 'operator', 'participant')) DEFAULT 'participant',
    password_hash TEXT, -- NULL for participants (no login required)
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_email ON users(email);

-- 2.9 Sessions Table
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    user_id INTEGER, -- NULL if anonymous participant
    participant_id TEXT NOT NULL, -- NIK / Employee ID
    status TEXT NOT NULL CHECK(status IN ('pending', 'active', 'paused', 'completed', 'terminated')) DEFAULT 'pending',
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    metadata JSON, -- Additional session info (device, OS, etc.)
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_sessions_event ON sessions(event_id);
CREATE INDEX idx_sessions_status ON sessions(status);
CREATE INDEX idx_sessions_participant ON sessions(participant_id);
CREATE INDEX idx_sessions_dates ON sessions(started_at, completed_at);

-- 2.10 Responses Table (Encrypted)
CREATE TABLE IF NOT EXISTS responses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    tool_id INTEGER NOT NULL,
    subtest_id INTEGER, -- NULL for tools without subtests
    encrypted_data BLOB NOT NULL, -- AES-256-GCM encrypted JSON
    nonce BLOB NOT NULL, -- 96-bit nonce for GCM
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (tool_id) REFERENCES tools(id) ON DELETE CASCADE,
    FOREIGN KEY (subtest_id) REFERENCES tool_subtests(id) ON DELETE SET NULL
);

CREATE INDEX idx_responses_session ON responses(session_id);
CREATE INDEX idx_responses_tool ON responses(tool_id);

-- 2.11 Surveillance Logs Table
CREATE TABLE IF NOT EXISTS surveillance_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    event_type TEXT NOT NULL CHECK(event_type IN (
        'no_face', 
        'multiple_faces', 
        'face_mismatch', 
        'input_blocked', 
        'process_detected',
        'window_unfocus', 
        'other'
    )),
    severity TEXT NOT NULL CHECK(severity IN ('info', 'warning', 'critical')) DEFAULT 'warning',
    event_data JSON, -- Additional context
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX idx_surveillance_session ON surveillance_logs(session_id);
CREATE INDEX idx_surveillance_type ON surveillance_logs(event_type);
CREATE INDEX idx_surveillance_time ON surveillance_logs(timestamp);

-- 2.12 Reports Table
CREATE TABLE IF NOT EXISTS reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL UNIQUE,
    scores JSON NOT NULL, -- All scores per tool
    interpretations JSON NOT NULL, -- Textual interpretations
    report_html TEXT, -- Pre-rendered HTML report
    generated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX idx_reports_session ON reports(session_id);
