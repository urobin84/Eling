# Testing di Laptop yang Sama

## 🎯 Overview

Anda bisa menjalankan **Admin** dan **Candidate** di laptop yang sama dengan 2 cara:
1. **2 Terminal** - Run 2 instance dev server
2. **2 Browser** - Satu admin, satu candidate (jika web-based)

---

## 🚀 Method 1: Dual Terminal (Recommended)

### Setup

**Terminal 1 - Admin:**
```bash
# Terminal pertama
cd /Users/robin/Documents/mqr-pro/Rust/Eling
npm run dev
```

**Terminal 2 - Candidate:**
```bash
# Terminal kedua (buka tab/window baru)
cd /Users/robin/Documents/mqr-pro/Rust/Eling
npm run dev
```

### Workflow

**1. Di Terminal 1 (Admin):**
- Login sebagai `admin`
- Settings → Server Control → Start Server
- Server akan running di `http://localhost:8080`
- Copy URL: `http://localhost:8080`
- Buat event dan generate code

**2. Di Terminal 2 (Candidate):**
- Login/Register sebagai candidate
- Settings → Server Connection
- Masukkan URL: `http://localhost:8080`
- Test Connection → Save
- Join event dengan code dari admin

**3. Testing:**
- Admin: Monitor di Active Sessions
- Candidate: Start test dan submit
- Admin: Lihat hasil masuk di Test Results

---

## 🔧 Method 2: Different Ports

Jika ingin run 2 instance sekaligus dengan port berbeda:

**Terminal 1 - Admin (Port 1420):**
```bash
npm run dev
# Default port 1420
```

**Terminal 2 - Candidate (Port 1421):**
```bash
# Edit vite.config.ts untuk port berbeda
npm run dev -- --port 1421
```

**Server URL untuk Candidate:**
```
http://localhost:8080
```

---

## 🌐 Network Configuration

### Localhost Testing

**Admin Server URL:**
- `http://localhost:8080` ✅
- `http://127.0.0.1:8080` ✅

**Candidate Configuration:**
```
Server URL: http://localhost:8080
```

### Testing dengan IP Lokal

Jika ingin test dengan IP (simulasi network):

**1. Check IP Anda:**
```bash
ifconfig | grep "inet " | grep -v 127.0.0.1
# Contoh output: 192.168.1.105
```

**2. Admin Server:**
- Server otomatis listen di `0.0.0.0:8080`
- Accessible via `http://192.168.1.105:8080`

**3. Candidate:**
```
Server URL: http://192.168.1.105:8080
```

---

## 📊 Database Isolation

Karena menggunakan SQLite, kedua instance akan **share database** yang sama di:
```
~/.eling/admin.db
~/.eling/candidate.db
```

**Untuk isolasi penuh:**

**Option 1: Different User Accounts**
```bash
# Terminal 1 - Admin
# Login sebagai: admin

# Terminal 2 - Candidate  
# Login sebagai: candidate1
```

**Option 2: Different Database Paths**
```bash
# Set environment variable
export ELING_DB_PATH=~/.eling/test_candidate.db
npm run dev
```

---

## 🎓 Complete Testing Workflow

### Step-by-Step

**1. Start Admin (Terminal 1):**
```bash
cd /Users/robin/Documents/mqr-pro/Rust/Eling
npm run dev
```
- Browser akan open di `http://localhost:1420`
- Login: `admin` / `password`
- Go to Settings → Server Control
- Click "Start Server"
- Note: Server running di `http://localhost:8080`

**2. Create Event (Admin):**
- Go to Events
- Click "Create Event"
- Name: "Test Event 1"
- Click "Generate Code"
- Note the code: `ABC123`

**3. Start Candidate (Terminal 2):**
```bash
# Buka terminal baru
cd /Users/robin/Documents/mqr-pro/Rust/Eling
npm run dev -- --port 1421
```
- Browser akan open di `http://localhost:1421`
- Register new account: `candidate1`
- Login dengan account baru

**4. Configure Server (Candidate):**
- Go to Settings → Server Connection
- Server URL: `http://localhost:8080`
- Click "Test Connection" (should show success)
- Click "Save Configuration"

**5. Join Event (Candidate):**
- Go to Dashboard
- Click "Join Event"
- Enter code: `ABC123`
- Click "Join"

**6. Take Test (Candidate):**
- Select event
- Click "Start Test"
- Answer questions
- Submit test

**7. Monitor (Admin):**
- Switch ke Terminal 1 browser
- Go to Active Sessions (should see candidate)
- Go to Test Results (after submit)
- Verify data received

---

## 🔍 Debugging

### Check Server Status

**Terminal 3 (Optional):**
```bash
# Test API health
curl http://localhost:8080/api/health

# Expected response:
# {"status":"ok","version":"0.1.0","timestamp":"..."}
```

### Check Database

```bash
# View admin database
sqlite3 ~/.eling/admin.db "SELECT * FROM sync_log;"

# View candidate database  
sqlite3 ~/.eling/candidate.db "SELECT * FROM sync_queue;"
```

### View Logs

```bash
# Admin logs
tail -f ~/.eling/logs/server.log

# Candidate logs
tail -f ~/.eling/logs/sync.log
```

---

## ⚠️ Common Issues

### Port Already in Use

**Error:** `Port 1420 is already in use`

**Solution:**
```bash
# Kill existing process
lsof -i :1420
kill -9 <PID>

# Or use different port
npm run dev -- --port 1421
```

### Connection Refused

**Error:** `Connection refused to localhost:8080`

**Solutions:**
1. Verify admin server is running
2. Check server status di admin UI
3. Test dengan curl:
   ```bash
   curl http://localhost:8080/api/health
   ```

### Database Locked

**Error:** `Database is locked`

**Solution:**
```bash
# Close all instances
# Delete lock file
rm ~/.eling/*.db-shm
rm ~/.eling/*.db-wal
```

---

## 💡 Tips

**1. Use Different Browsers:**
- Admin: Chrome
- Candidate: Firefox/Safari
- Avoid session conflicts

**2. Use Incognito/Private:**
- Separate browser sessions
- Clear cookies/storage

**3. Monitor Both:**
- Split screen: Admin (left) | Candidate (right)
- Watch real-time sync

**4. Test Offline:**
```bash
# Candidate: Disconnect network
# Take test offline
# Reconnect → Auto-sync
```

**5. Test Multiple Candidates:**
```bash
# Terminal 2: Candidate 1 (port 1421)
npm run dev -- --port 1421

# Terminal 3: Candidate 2 (port 1422)  
npm run dev -- --port 1422

# Terminal 4: Candidate 3 (port 1423)
npm run dev -- --port 1423
```

---

## 📝 Quick Reference

### URLs
```
Admin UI:     http://localhost:1420
Candidate UI: http://localhost:1421
API Server:   http://localhost:8080
Health Check: http://localhost:8080/api/health
```

### Commands
```bash
# Start admin
npm run dev

# Start candidate (different port)
npm run dev -- --port 1421

# Test API
curl http://localhost:8080/api/health

# View logs
tail -f ~/.eling/logs/server.log
```

### Database
```bash
# Admin DB
sqlite3 ~/.eling/admin.db

# Candidate DB
sqlite3 ~/.eling/candidate.db

# View tables
.tables

# View sync queue
SELECT * FROM sync_queue;
```

---

## ✅ Testing Checklist

- [ ] Terminal 1: Admin running
- [ ] Terminal 2: Candidate running
- [ ] Admin: Server started (port 8080)
- [ ] Admin: Event created with code
- [ ] Candidate: Server configured
- [ ] Candidate: Connection tested
- [ ] Candidate: Event joined
- [ ] Candidate: Test completed
- [ ] Admin: Results received
- [ ] Sync queue: Empty (all synced)

---

**Happy Testing!** 🎉
