# Sistem Notifikasi Admin

## Overview
Sistem notifikasi real-time untuk memberitahu admin/psikolog ketika kandidat menyelesaikan tes mereka.

## Fitur

### 1. **Notifikasi Real-time**
- Polling otomatis setiap 30 detik
- Badge counter untuk notifikasi yang belum dibaca
- Dropdown panel dengan daftar notifikasi

### 2. **Jenis Notifikasi**
- `test_completed`: Kandidat selesai mengerjakan tes
- `session_started`: Sesi tes dimulai
- `alert`: Peringatan sistem
- `info`: Informasi umum

### 3. **Aksi Notifikasi**
- Mark as read (klik pada notifikasi)
- Mark all as read (tombol di header dropdown)
- Auto-refresh setelah aksi

## Cara Menggunakan

### Backend (Rust)

#### Membuat Notifikasi Ketika Test Selesai

```rust
// Di dalam fungsi yang menangani completion tes
use crate::db::Database;

async fn on_test_completed(
    session_id: i64,
    candidate_name: String,
    db: &Database
) -> Result<(), String> {
    // Create notification
    db.create_notification(
        None, // Broadcast ke semua admin
        "Test Completed",
        &format!("{} has completed their test session", candidate_name),
        "test_completed",
        Some(session_id),
        None,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
```

#### Atau menggunakan Tauri Command

```typescript
// Di frontend, setelah kandidat submit test
await invoke('create_test_completion_notification', {
    sessionId: currentSessionId,
    candidateName: 'John Doe'
});
```

### Frontend (Vue)

Komponen `NotificationBell` sudah terintegrasi di `AdminDashboard.vue`:

```vue
<NotificationBell />
```

## Database Schema

```sql
CREATE TABLE notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,                    -- NULL = broadcast to all admins
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    type TEXT NOT NULL,                 -- 'test_completed', 'session_started', etc.
    related_session_id INTEGER,
    related_user_id INTEGER,
    is_read BOOLEAN DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## API Commands

### `get_notifications()`
Mengambil semua notifikasi yang belum dibaca.

**Returns:** `Vec<Notification>`

### `mark_notification_read(notification_id: i64)`
Menandai satu notifikasi sebagai sudah dibaca.

### `mark_all_notifications_read()`
Menandai semua notifikasi sebagai sudah dibaca.

### `create_test_completion_notification(session_id: i64, candidate_name: String)`
Membuat notifikasi ketika kandidat selesai tes.

## Integrasi dengan Test Engine

Untuk mengintegrasikan dengan test engine, tambahkan kode berikut ketika kandidat menyelesaikan tes:

```typescript
// Di komponen test engine (misalnya KraepelinTest.vue)
async function submitTest() {
    try {
        // Submit answers...
        await submitAnswers();
        
        // Create notification
        await invoke('create_test_completion_notification', {
            sessionId: sessionId.value,
            candidateName: candidateName.value || 'Candidate'
        });
        
        // Navigate to completion page
        router.push('/test-complete');
    } catch (e) {
        console.error('Failed to submit test:', e);
    }
}
```

## Customization

### Mengubah Polling Interval

Edit `NotificationBell.vue`:

```typescript
// Default: 30 detik
pollInterval = window.setInterval(fetchNotifications, 30000);

// Ubah menjadi 10 detik untuk update lebih cepat
pollInterval = window.setInterval(fetchNotifications, 10000);
```

### Menambahkan Jenis Notifikasi Baru

1. Update enum di migration SQL
2. Tambahkan icon di `getNotificationIcon()` function
3. Buat command baru di `notifications.rs` jika diperlukan

## Future Enhancements

- [ ] WebSocket untuk real-time push notifications
- [ ] Sound notification
- [ ] Desktop notification (Tauri API)
- [ ] Filter notifikasi berdasarkan tipe
- [ ] Pagination untuk notifikasi lama
- [ ] Email notification integration
