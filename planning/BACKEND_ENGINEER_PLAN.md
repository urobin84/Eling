# Backend Engineering Plan V2 (Rust/Tauri)

## 1. Modular SQLite Schema
- **Table `tools`**: Definisi alat uji (JSON schema soal).
- **Table `events`**: Urutan paket (Contoh: Event "Rekrutmen Manager" = IST -> MSDT -> PAPI).
- **Table `responses`**: Jawaban terenkripsi per sub-tes.

## 2. High-Precision Sequencer (Rust Core)
- Menggunakan `tokio::time` untuk mengelola timer sub-tes yang sangat presisi (terutama Kraepelin).
- Fungsi `next_stage()`: Mengunci data sub-tes sebelumnya secara permanen dan memvalidasi integritas file sebelum lanjut.

## 3. Advanced Lockdown Implementation
- **rdev Hooks**: Memblokir seluruh input sistem kecuali yang dibutuhkan aplikasi.
- **Always-on-Top Enforcement**: Memaksa window Tauri tetap di depan.
- **Surveillance Worker**: Thread khusus untuk memproses stream kamera (Face Detection) menggunakan `nokhwa`.

## 4. Data Security
- Implementasi `AES-256-GCM` untuk mengenkripsi jawaban di SQLite.
- Key enkripsi bersifat ephemeral (hanya ada di RAM selama ujian berlangsung).