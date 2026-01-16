# Data Flow Diagram - Eling Platform

> [!NOTE]
> Dokumen ini menjelaskan aliran data secara detail dalam Eling Platform, dari user interaction hingga database persistence, termasuk security checks dan event handling.

---

## 1. Complete System Data Flow

```mermaid
flowchart TB
    subgraph User["👤 User Layer"]
        UserInput[User Interaction]
        Display[Display Output]
    end
    
    subgraph Frontend["🎨 Frontend Layer (Vue 3)"]
        Components[UI Components]
        Store[Pinia Store]
        Validators[Input Validators]
    end
    
    subgraph Backend["⚙️ Backend Layer (Tauri/Rust)"]
        Commands[Command Handlers]
        Logic[Business Logic]
        Sequencer[Test Sequencer]
        Scorer[Scoring Engine]
        Security[Security Module]
    end
    
    subgraph Data["💾 Data Layer"]
        Cache[In-Memory Cache]
        DB[(SQLite Encrypted)]
    end
    
    subgraph External["🔌 External Systems"]
        Camera[Camera API]
        OS[OS APIs]
    end
    
    UserInput --> Components
    Components --> Validators
    Validators --> Store
    Store <-->|Tauri Commands| Commands
    Commands --> Logic
    Logic --> Sequencer
    Logic --> Scorer
    Logic --> Security
    Security <--> Camera
    Security <--> OS
    Sequencer --> Cache
    Logic --> DB
    DB --> Logic
    Logic --> Commands
    Commands -->|Events| Store
    Store --> Components
    Components --> Display
```

---

## 2. Test Session Lifecycle Data Flow

### 2.1 Session Initialization

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant UI as Vue UI
    participant Store as Pinia Store
    participant Cmd as Tauri Commands
    participant Seq as Test Sequencer
    participant DB as SQLite
    participant Sec as Security Module
    participant Cam as Camera
    
    User->>UI: Click "Start Test"
    UI->>Store: startSession(eventId)
    Store->>Cmd: invoke('create_session', {eventId})
    
    Cmd->>Sec: initializeSecurity()
    Sec->>Cam: startCameraStream()
    Cam-->>Sec: Stream ready
    Sec->>Sec: performFaceDetection()
    
    alt Face Detected
        Sec-->>Cmd: ✅ Identity verified
        Cmd->>DB: INSERT INTO sessions
        DB-->>Cmd: sessionId
        
        Cmd->>Seq: loadTestPackage(eventId)
        Seq->>DB: SELECT * FROM packages WHERE event_id = ?
        DB-->>Seq: Package data
        Seq->>Seq: parseToolSequence()
        Seq-->>Cmd: Test sequence ready
        
        Cmd-->>Store: emit('session:ready', {sessionId, sequence})
        Store->>UI: Update state
        UI-->>User: Show first instruction screen
    else No Face / Invalid
        Sec-->>Cmd: ❌ Identity verification failed
        Cmd-->>Store: emit('error', 'No face detected')
        Store->>UI: Show error
        UI-->>User: "Please position your face"
    end
```

---

### 2.2 Subtest Execution Flow

```mermaid
flowchart TD
    Start([Start Subtest]) --> ShowInst[Show Instructions]
    ShowInst --> UserReady{User Ready?}
    UserReady -->|Yes| FaceCheck[Face Verification]
    UserReady -->|No| ShowInst
    
    FaceCheck --> FaceValid{Face Valid?}
    FaceValid -->|No| LogViolation[Log Violation]
    LogViolation --> Warning[Show Warning]
    Warning --> FaceCheck
    
    FaceValid -->|Yes| StartTimer[Start Timer]
    StartTimer --> RenderQuestions[Render Questions]
    RenderQuestions --> Monitoring[Continuous Monitoring]
    
    Monitoring --> UserAnswer[User Answers]
    UserAnswer --> Validate[Validate Input]
    Validate --> ValidInput{Valid?}
    
    ValidInput -->|No| ShowError[Show Error]
    ShowError --> UserAnswer
    
    ValidInput -->|Yes| StoreLocal[Store in Pinia]
    StoreLocal --> AutoSave[Auto-save to Backend]
    
    AutoSave --> TimerCheck{Time Up?}
    TimerCheck -->|No| Monitoring
    TimerCheck -->|Yes| FinalSave[Final Save]
    
    FinalSave --> Encrypt[Encrypt Responses]
    Encrypt --> SaveDB[Save to DB]
    SaveDB --> NextCheck{More Subtests?}
    
    NextCheck -->|Yes| Transition[Transition Screen]
    Transition --> Start
    NextCheck -->|No| Complete([Session Complete])
```

---

## 3. Answer Submission Flow

### 3.1 Single Answer Flow (e.g., TIU Question)

```mermaid
sequenceDiagram
    participant UI as UI Component
    participant Store as Pinia Store
    participant Valid as Validator
    participant Cmd as Commands
    participant Enc as Encryption
    participant DB as Database
    
    UI->>Store: submitAnswer(questionId, answer)
    Store->>Valid: validate(answer)
    
    alt Invalid Input
        Valid-->>Store: ❌ Validation error
        Store-->>UI: Show error message
    else Valid Input
        Valid-->>Store: ✅ Valid
        Store->>Store: updateLocalState(questionId, answer)
        Store->>Cmd: invoke('save_response', {sessionId, questionId, answer})
        
        Cmd->>Enc: encrypt(answer, sessionKey)
        Enc-->>Cmd: encryptedData + nonce
        
        Cmd->>DB: INSERT INTO responses (encrypted_data, nonce)
        DB-->>Cmd: ✅ Saved (rowId)
        
        Cmd-->>Store: emit('response:saved', {questionId, rowId})
        Store-->>UI: ✅ Show success indicator
    end
```

---

### 3.2 Bulk Answer Flow (e.g., Kraepelin)

```mermaid
flowchart LR
    Input[User Types Numbers] --> Buffer[Local Buffer<br/>In-Memory Array]
    Buffer --> Debounce{Debounce<br/>500ms}
    Debounce -->|Timer Hit| Batch[Batch Responses]
    Batch --> Compress[Compress JSON]
    Compress --> Encrypt[Encrypt Payload]
    Encrypt --> Send[Send to Backend]
    Send --> Queue[SQLite Write Queue]
    Queue --> Write[Batch INSERT]
    Write --> ACK[Acknowledge]
    ACK --> Clear[Clear Local Buffer]
```

**Optimization:**
- **Debouncing:** Menghindari terlalu banyak writes
- **Batching:** Menggabungkan multiple inputs menjadi satu transaction
- **Compression:** Mengurangi size untuk bulk data

---

## 4. Surveillance Data Flow

### 4.1 Real-time Face Detection

```mermaid
graph TB
    Camera[Camera Stream] -->|30 FPS| Worker[Surveillance Worker<br/>Background Thread]
    
    Worker --> Extract[Frame Extraction]
    Extract --> Resize[Resize to 640x480]
    Resize --> Detect[Face Detection<br/>Haar Cascade / DNN]
    
    Detect --> Found{Face Found?}
    
    Found -->|No| NoFaceCounter[Increment No-Face Counter]
    NoFaceCounter --> Threshold1{Counter > 10?}
    Threshold1 -->|Yes| ViolationEvent1[Emit 'no_face' Event]
    Threshold1 -->|No| Continue1[Continue]
    
    Found -->|Yes| CountFaces[Count Faces]
    CountFaces --> Multiple{Count > 1?}
    
    Multiple -->|Yes| ViolationEvent2[Emit 'multiple_faces' Event]
    Multiple -->|No| ExtractFeatures[Extract Face Embedding]
    
    ExtractFeatures --> Compare[Compare with Baseline]
    Compare --> Similar{Similarity > 0.8?}
    
    Similar -->|No| ViolationEvent3[Emit 'face_mismatch' Event]
    Similar -->|Yes| ResetCounters[Reset Violation Counters]
    
    ViolationEvent1 --> LogDB[(Log to DB)]
    ViolationEvent2 --> LogDB
    ViolationEvent3 --> LogDB
    LogDB --> EmitUI[Emit to Frontend]
    EmitUI --> ShowWarning[Show Warning UI]
    
    ResetCounters --> Continue1
    Continue1 --> Worker
```

---

### 4.2 Violation Handling Flow

```mermaid
stateDiagram-v2
    [*] --> Normal: Test Running
    
    Normal --> Warning: Violation Detected
    Warning --> Normal: Violation Resolved
    Warning --> Critical: 3+ Violations
    
    Critical --> Paused: Pause Test
    Paused --> Normal: Admin Override / Face Fixed
    Paused --> Terminated: Timeout (60s)
    
    Terminated --> [*]: Save Partial Results
```

---

## 5. Security Event Flow

### 5.1 Input Blocking Flow

```mermaid
sequenceDiagram
    participant User
    participant OS as Operating System
    participant Hook as Input Hook (rdev)
    participant Filter as Key Filter
    participant Log as Violation Logger
    participant UI as Frontend
    
    User->>OS: Press Cmd+Tab
    OS->>Hook: Keyboard event captured
    Hook->>Filter: checkKeyboardEvent(event)
    
    Filter->>Filter: isBlacklisted(Cmd+Tab)?
    
    alt Blacklisted
        Filter->>Filter: preventDefault()
        Filter->>Log: logViolation('shortcut_attempt', 'Cmd+Tab')
        Log->>UI: emit('violation:input_blocked')
        UI-->>User: Show warning Toast
    else Allowed
        Filter->>OS: Allow event
        OS-->>User: Action executed
    end
```

---

### 5.2 Process Monitoring Flow

```mermaid
flowchart TD
    Timer[Periodic Timer<br/>Every 5 seconds] --> Scan[Scan Running Processes]
    Scan --> GetList[Get Process List<br/>OS API]
    GetList --> Compare[Compare with Blacklist]
    
    Compare --> Forbidden{Forbidden<br/>Process Found?}
    
    Forbidden -->|No| Sleep[Sleep 5s]
    Sleep --> Timer
    
    Forbidden -->|Yes| Multiple[Check Process Details]
    Multiple --> Log[Log Violation]
    Log --> Notify[Notify User]
    
    Notify --> Action{Severity}
    Action -->|Warning| Toast[Show Toast]
    Action -->|Critical| Pause[Pause Test]
    
    Toast --> Timer
    Pause --> WaitUser[Wait for Process Kill]
    WaitUser --> Verify{Process Killed?}
    Verify -->|Yes| Resume[Resume Test]
    Verify -->|No| Timeout[Timeout After 60s]
    
    Resume --> Timer
    Timeout --> Terminate[Terminate Session]
```

---

## 6. Scoring Data Flow

### 6.1 Score Calculation Flow

```mermaid
flowchart LR
    Complete[Test Complete] --> Fetch[Fetch All Responses]
    Fetch --> Decrypt[Decrypt Each Response]
    Decrypt --> Parse[Parse JSON Answers]
    
    Parse --> Match[Match with Answer Keys]
    Match --> Calculate[Calculate Raw Scores]
    
    Calculate --> Normalize[Normalize Scores]
    Normalize --> Transform[Apply Norms<br/>e.g., T-Score, IQ]
    
    Transform --> Interpret[Generate Interpretation]
    Interpret --> Report[Create Report JSON]
    
    Report --> SaveDB[(Save to DB)]
    SaveDB --> Export[Export Options]
    
    Export --> PDF[PDF Report]
    Export --> JSON[JSON Data]
    Export --> CSV[CSV Export]
```

---

### 6.2 Scoring Algorithm Example (TIU)

```mermaid
graph TD
    Start([Start Scoring]) --> LoadKey[Load Answer Key from DB]
    LoadKey --> LoadResp[Load User Responses]
    
    LoadResp --> LoopQuestion[For Each Question]
    LoopQuestion --> Compare{User Answer<br/>== Correct Answer?}
    
    Compare -->|Yes| AddPoint[raw_score += 1]
    Compare -->|No| NoPoint[Skip]
    
    AddPoint --> Next{More Questions?}
    NoPoint --> Next
    
    Next -->|Yes| LoopQuestion
    Next -->|No| CalcPercent[percentage = raw / total × 100]
    
    CalcPercent --> Lookup[Lookup Norm Table]
    Lookup --> GetCat[Get Category<br/>e.g., Tinggi, Sedang, Rendah]
    
    GetCat --> Return([Return Score Object])
```

---

## 7. Report Generation Flow

```mermaid
sequenceDiagram
    participant UI
    participant Cmd as Commands
    participant Scorer as Scoring Engine
    participant DB
    participant Template as Report Template
    participant Export as PDF Exporter
    
    UI->>Cmd: invoke('generate_report', {sessionId})
    Cmd->>DB: SELECT * FROM responses WHERE session_id = ?
    DB-->>Cmd: Encrypted responses
    
    Cmd->>Scorer: calculateScores(responses)
    loop For each tool
        Scorer->>Scorer: decryptResponses()
        Scorer->>Scorer: applyScoring Algorithm()
        Scorer->>Scorer: normalizeScores()
        Scorer->>Scorer: generateInterpretation()
    end
    Scorer-->>Cmd: Complete score object
    
    Cmd->>DB: INSERT INTO reports (session_id, scores)
    Cmd->>Template: loadTemplate('standard_report.hbs')
    Template-->>Cmd: HTML template
    
    Cmd->>Cmd: renderTemplate(scores, template)
    Cmd->>Export: convertToPDF(html)
    Export-->>Cmd: PDF buffer
    
    Cmd-->>UI: emit('report:ready', {pdfPath})
    UI->>UI: Show download button
```

---

## 8. Cache Management Flow

### 8.1 Session Cache Strategy

```mermaid
graph LR
    Request[Data Request] --> CacheCheck{In Cache?}
    
    CacheCheck -->|Hit| Return1[Return from Cache]
    CacheCheck -->|Miss| FetchDB[Fetch from DB]
    
    FetchDB --> Store[Store in Cache]
    Store --> Return2[Return Data]
    
    Return1 --> TTL{TTL Expired?}
    TTL -->|Yes| Invalidate[Invalidate Cache]
    TTL -->|No| Done1[Done]
    
    Invalidate --> FetchDB
    Return2 --> Done2[Done]
```

**Cache Items:**
- Test package definitions (TTL: Session lifetime)
- Current subtest data (TTL: Until subtest complete)
- User profile (TTL: Session lifetime)
- Answer keys (TTL: 1 hour)

---

## 9. Error Handling Flow

```mermaid
flowchart TD
    Error[Error Occurs] --> Classify{Error Type?}
    
    Classify -->|Network| NetworkHandler[No Network<br/>Expected - Ignore]
    Classify -->|Database| DBHandler[Database Error]
    Classify -->|Camera| CamHandler[Camera Error]
    Classify -->|Validation| ValHandler[Validation Error]
    
    DBHandler --> Critical{Critical?}
    Critical -->|Yes| Alert[Show Alert Modal]
    Critical -->|No| Toast1[Show Toast]
    
    Alert --> SaveState[Save Current State]
    SaveState --> GracefulExit[Graceful Exit]
    
    Toast1 --> Retry1{Retryable?}
    Retry1 -->|Yes| RetryLogic[Retry with Backoff]
    Retry1 -->|No| LogOnly[Log Error]
    
    CamHandler --> Fallback[Disable Surveillance<br/>Log Warning]
    Fallback --> Continue1[Continue Test]
    
    ValHandler --> UserFeedback[Show Field Error]
    UserFeedback --> WaitCorrection[Wait for Correction]
    
    NetworkHandler --> Continue2[Continue Offline]
    GracefulExit --> End([End])
    RetryLogic --> End
    LogOnly --> End
    Continue1 --> End
    WaitCorrection --> End
    Continue2 --> End
```

---

## 10. Data Persistence Strategy

### 10.1 Write Strategies by Data Type

| Data Type | Strategy | Rationale |
|-----------|----------|-----------|
| **Responses** | Immediate encrypted write | Critical data, cannot be lost |
| **Session metadata** | Write on state change | Moderate importance |
| **Violations** | Batched write (10s interval) | Non-critical, can tolerate delay |
| **Camera frames** | Not persisted | Too large, reconstruct from violations |
| **Scores** | Write on completion | One-time calculation |
| **Reports** | Write on generation | User-triggered |

---

### 10.2 Transaction Management

```mermaid
sequenceDiagram
    participant App
    participant TxManager as Transaction Manager
    participant DB
    
    App->>TxManager: beginTransaction()
    TxManager->>DB: BEGIN TRANSACTION
    
    App->>TxManager: execute(query1)
    TxManager->>DB: INSERT responses
    
    App->>TxManager: execute(query2)
    TxManager->>DB: UPDATE session
    
    alt Success
        App->>TxManager: commit()
        TxManager->>DB: COMMIT
        DB-->>App: ✅ Success
    else Error
        App->>TxManager: rollback()
        TxManager->>DB: ROLLBACK
        DB-->>App: ❌ Rolled back
    end
```

---

## 11. Performance Optimization Data Flows

### 11.1 Lazy Loading Strategy

```mermaid
graph TD
    AppStart[App Start] --> LoadMinimal[Load Minimal Core]
    LoadMinimal --> ShowUI[Show UI Shell]
    
    ShowUI --> UserAction[Wait for User Action]
    UserAction --> DetermineNeeds{What's Needed?}
    
    DetermineNeeds -->|Start Test| LoadEngine[Load Test Engine]
    DetermineNeeds -->|View Report| LoadReporting[Load Reporting Module]
    DetermineNeeds -->|Settings| LoadSettings[Load Settings UI]
    
    LoadEngine --> DynamicImport1[Dynamic Import Components]
    LoadReporting --> DynamicImport2[Dynamic Import Components]
    LoadSettings --> DynamicImport3[Dynamic Import Components]
    
    DynamicImport1 --> Ready1[Ready]
    DynamicImport2 --> Ready2[Ready]
    DynamicImport3 --> Ready3[Ready]
```

---

### 11.2 Virtual Scrolling Flow (Kraepelin)

```mermaid
sequenceDiagram
    participant User
    participant VS as Virtual Scroller
    participant Cache as Render Cache
    participant DOM
    
    User->>VS: Scroll viewport
    VS->>VS: Calculate visible range
    Note over VS: Only render items 100-150
    
    VS->>Cache: Get items(100, 150)
    
    alt Items in cache
        Cache-->>VS: Return cached DOM
        VS->>DOM: Update viewport
    else Not in cache
        VS->>VS: Generate DOM for items
        VS->>Cache: Store in cache
        VS->>DOM: Render to viewport
    end
    
    DOM-->>User: Display updated view
```

---

## Summary: Data Flow Principles

> [!IMPORTANT]
> **Core Principles:**
> 1. **Security First:** Semua sensitive data dienkripsi sebelum persist
> 2. **Performance:** Debouncing, batching, dan caching untuk optimal UX
> 3. **Reliability:** Transaction management dan error handling
> 4. **Privacy:** Minimal data collection, local-only processing
> 5. **Auditability:** Comprehensive logging untuk compliance

**Key Metrics:**
- Response save latency: < 100ms
- Face detection latency: < 200ms (5 FPS)
- UI render time: < 16ms (60 FPS)
- Database query time: < 50ms (99th percentile)

