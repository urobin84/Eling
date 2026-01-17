# ELING API - Quick Reference

## Base URL
```
http://localhost:8080/api
```

## Endpoints

### 1. Health Check
```bash
curl http://localhost:8080/api/health
```

### 2. Get Event
```bash
curl http://localhost:8080/api/events/ABC123
```

### 3. Submit Test Result
```bash
curl -X POST http://localhost:8080/api/test-results \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "test-123",
    "event_id": 1,
    "user_id": 1,
    "answers": [
      {"question_id": 1, "answer": "A", "answered_at": "2024-01-17T06:00:00Z"}
    ],
    "completed_at": "2024-01-17T06:05:00Z"
  }'
```

### 4. Upload Recording
```bash
curl -X POST http://localhost:8080/api/recordings \
  -F "session_id=test-123" \
  -F "recording_type=camera" \
  -F "video=@/path/to/video.webm"
```

## Postman Collection

Import file: `ELING_API.postman_collection.json`

## Environment Variables

```
base_url = http://localhost:8080
```

For production, change to your admin server IP:
```
base_url = http://192.168.1.100:8080
```
