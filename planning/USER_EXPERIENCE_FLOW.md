# User Experience Flow - Eling Platform "Lab Mode"

> [!NOTE]
> Dokumen ini menjelaskan alur pengalaman pengguna (UX) untuk dua peran utama: **Admin (Psikolog)** dan **Test Taker (Calon Karyawan)**. Fokus utama adalah pada aspek edukasi ("Lab Mode") di mana setiap alat uji dijelaskan tujuan dan cara kerjanya sebelum tes dimulai.

---

## 1. High-Level User Journey

```mermaid
graph TD
    subgraph "Admin (Psychologist)"
        A1[Login] --> A2[Dashboard]
        A2 --> A3[Create Event/Package]
        A3 --> A4[Monitor Session]
        A4 --> A5[View/Export Report]
    end
    
    subgraph "Test Taker (Candidate)"
        U1[Start Session] --> U2[Identity Verification]
        U2 --> U3[Lab Introduction<br/>(Educational Context)]
        U3 --> U4[Test Sequence Loop]
        U4 --> U5[Completion]
    end
```

---

## 2. Admin (Psychologist) Experience

Tujuan Admin adalah mengatur konfigurasi tes ("meracik obat") dan memantau hasil ("diagnosa").

### 2.1 Alur Membuat Paket Tes (The Checkup Recipe)

1.  **Dashboard Overview**
    *   Admin melihat kalender event tes yang akan datang.
    *   Statistik singkat: Jumlah peserta aktif, completion rate, alerts.

2.  **Create New Event**
    *   Admin memilih **"Create New Lab Session"**.
    *   Input Nama Event (misal: *"Management Trainee Batch 5"*).
    *   **Educational Context Setting**: Admin dapat menuliskan deskripsi singkat kenapa rangkaian tes ini dipilih (misal: *"Untuk mengukur potensi kepemimpinan dan ketahanan stres"*).

3.  **Package Configuration (The Mix)**
    *   Admin memilih alat uji dari **Psychological Tools Library**.
    *   UI Drag-and-drop untuk menyusun urutan:
        1.  *TIU* (untuk IQ)
        2.  *DISC* (untuk Kepribadian)
        3.  *Kraepelin* (untuk Ketahanan Kerja)
    *   **Custom Instruction**: Admin bisa menambahkan catatan khusus untuk peserta sebelum tiap subtes (opsional).

4.  **Security Settings**
    *   Toggle fitur keamanan: "Strict Lockdown", "Face Surveillance", "Offline Mode Only".

5.  **Publish & Generate Token**
    *   Event disimpan dan menghasilkan **Event ID/Token** untuk didistribusikan ke peserta.

### 2.2 Alur Monitoring & Reporting

1.  **Live Dashboard**
    *   Melihat status peserta secara *real-time*: "Sedang Mengerjakan IST - Subtes 3".
    *   **Violation Feed**: Notifikasi jika ada peserta yang wajahnya hilang atau terdeteksi kecurangan.

2.  **Data Analysis (The Lab Result)**
    *   Klik peserta untuk melihat **Individual Report**.
    *   **Scoring Breakdown**: Nilai mentah vs Norma.
    *   **Behavioral Note**: Catatan sistem (misal: *"Sering melihat ke arah lain pada menit ke-15"*).
    *   Export PDF/Excel.

---

## 3. Test Taker (Calon Karyawan) Experience

Tujuan Peserta adalah menyelesaikan tes dengan pemahaman yang baik tentang apa yang diukur ("Lab Experience").

### 3.1 Onboarding & "Lab" Entry

1.  **Welcome Screen (The Lobby)**
    *   User memasukkan **Event ID** dan **Participant ID** (NIK/No KTP).
    *   Sistem melakukan *System Check* (Kamera, Resolusi Layar, Battery).

2.  **Identity Verification (Security Check)**
    *   Instruksi: *"Posisikan wajah Anda di dalam bingkai"*.
    *   Camera Preview muncul.
    *   Sistem mengambil **Baseline Face Data**.

3.  **Lab Context (The Briefing)**
    *   Tampilan **"Welcome to Psychotest Lab"**.
    *   Penjelasan Event: *"Anda mengikuti seleksi Management Trainee. Rangkaian tes ini akan mengukur 3 aspek utama Anda: Kognitif, Kepribadian, dan Performa Kerja."*
    *   Security Reminder: *"Selama proses ini, kamera akan aktif untuk memastikan integritas sesi."*

### 3.2 Test Loop (The Experiment)

Untuk setiap alat uji (Example: Kraepelin), alurnya adalah:

#### Phase A: Educational Overview (The "Why")
*   **Screen**: "Module: Performance Test (Kraepelin)"
*   **What is this?**: *"Tes ini bukan tentang matematika canggih. Ini adalah simulasi beban kerja rutin."*
*   **What it measures?**: *"Kami mengukur kecepatan, ketelitian, keajegan (konsistensi), dan ketahanan stres Anda saat bekerja di bawah tekanan waktu."*
*   **Tips**: *"Jangan berhenti. Jika salah, biarkan saja dan lanjut ke angka berikutnya."*

#### Phase B: Technical Tutorial (The "How")
*   **Interactive Demo**: Peserta mencoba input angka dummy.
*   **Key Controls**:
    *   *"Ketik angka satuan dari hasil penjumlahan"*
    *   *"Tekan Enter/Spasi untuk pindah baris (jika manual)"* atau otomatis pindah.
*   Konfirmasi: *"Saya Paham & Siap Mulai"*.

#### Phase C: Execution (The Test)
*   User mengerjakan tes.
*   **Surveillance UI**: Terdapat indikator "Recording" kecil di pojok (memberi efek psikologis diawasi).
*   **Timer**: Countdown timer yang jelas tapi tidak mengganggu.
*   **Blocker**: Tombol shortcut dimatikan. Jika user mencoba Alt+Tab, muncul peringatan *"Fokus pada tes! Percobaan keluar tercatat."*

#### Phase D: Transition (The Break)
*   Selesai satu modul.
*   **Cool Down**: *"Modul Performance selesai. Data tersimpan aman."*
*   **Next Up**: *"Selanjutnya: Tes Kepribadian (DISC). Tidak ada jawaban benar/salah. Jadilah diri sendiri."*

### 3.3 Completion (Exit Lab)

1.  **Final Submission**
    *   Sistem mengenkripsi semua data.
    *   Status upload (jika online) atau instruksi export (jika offline).

2.  **Closing Statement**
    *   *"Terima kasih telah menyelesaikan Lab Session."*
    *   Info kapan hasil akan diumumkan (custom text dari Admin).
    *   Aplikasi menutup diri secara otomatis.

---

## 4. Visual Flow Diagram (Test Taker)

```mermaid
sequenceDiagram
    actor U as User
    participant Sys as System
    participant Ed as Edu-Module
    participant T as Test-Engine
    
    U->>Sys: Input ID
    Sys->>U: Verify Face
    
    rect rgb(240, 248, 255)
        Note over U, T: Test Loop 1: KRAEPELIN
        
        Sys->>Ed: Load Kraepelin Context
        Ed->>U: Show "Why & How" (Lab Context)
        U->>Ed: Try Practice Mode
        Ed->>Sys: Ready
        
        Sys->>T: Start Test (Lockdown Mode)
        activate T
        T->>U: Show Questions
        U->>T: Input Answers
        T->>Sys: Save Encrypted Data
        deactivate T
        
        Sys->>U: Show Transition/Break
    end
    
    rect rgb(255, 240, 245)
        Note over U, T: Test Loop 2: DISC
        Sys->>Ed: Load DISC Context
        Ed->>U: Show "Personality Logic"
        ...
    end
    
    Sys->>U: Show Completion Screen
```

---

## 5. Key UX Principles

*   **Transparency**: Peserta diberitahu apa yang diukur, mengurangi kecemasan akan "magic metrics".
*   **Focus**: UI minimalis saat tes berlangsung (Zen Mode) untuk akurasi data maksimal.
*   **Feedback Loop**: Indikator visual saat data tersimpan memberikan rasa aman.
*   **Human Tone**: Instruksi menggunakan bahasa manusia yang memberdayakan, bukan bahasa mesin yang kaku.
