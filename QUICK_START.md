# ELING - Quick Start Guide

## 🚀 Untuk ADMIN (Server)

### 1. Jalankan Aplikasi
```bash
npm run dev
```

### 2. Login
- Username: `admin`
- Password: (password Anda)

### 3. Start API Server
1. Buka menu **Settings** (⚙️)
2. Klik **Server Control**
3. Klik **"Start Server"**
4. Copy server URL (contoh: `http://192.168.1.100:8080`)

### 4. Buat Event
1. Buka **Events**
2. Klik **"Create Event"**
3. Isi nama event
4. Klik **"Generate Code"**
5. Share **Event Code** dan **Server URL** ke kandidat

### 5. Monitor
- Lihat **Active Sessions** untuk real-time monitoring
- Lihat **Test Results** untuk hasil test

---

## 👤 Untuk CANDIDATE (Client)

### 1. Jalankan Aplikasi
```bash
npm run dev
```

### 2. Login/Register
- Buat akun baru atau login
- Role: `candidate`

### 3. Configure Server
1. Buka menu **Settings** (⚙️)
2. Klik **Server Connection**
3. Masukkan **Server URL** dari admin
   - Contoh: `http://192.168.1.100:8080`
4. Klik **"Test Connection"**
5. Klik **"Save Configuration"**

### 4. Join Event
1. Kembali ke **Dashboard**
2. Klik **"Join Event"**
3. Masukkan **Event Code** (6 karakter)
4. Klik **"Join"**

### 5. Ikuti Test
1. Pilih event
2. Klik **"Start Test"**
3. Selesaikan soal
4. Data auto-sync ke server

---

## 🌐 Network Setup

### Admin PC:
1. Check IP address:
   ```bash
   ifconfig  # macOS/Linux
   ipconfig  # Windows
   ```

2. Open port 8080:
   ```bash
   # macOS
   sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add /path/to/eling
   
   # Windows (Run as Admin)
   netsh advfirewall firewall add rule name="ELING" dir=in action=allow protocol=TCP localport=8080
   ```

### Candidate PC:
1. Pastikan **same network** dengan admin
2. Test connection:
   ```bash
   ping <admin-ip>
   curl http://<admin-ip>:8080/api/health
   ```

---

## ❓ Troubleshooting

### Connection Failed
1. Check firewall
2. Verify IP address
3. Ping admin server
4. Check server is running

### Data Not Syncing
1. Check sync queue status
2. Retry manual sync
3. Check server logs

### Server Won't Start
1. Check port 8080 not in use
2. Kill existing process:
   ```bash
   lsof -i :8080
   kill -9 <PID>
   ```

---

## 📞 Need Help?

Check full documentation: `DEPLOYMENT_GUIDE.md`
