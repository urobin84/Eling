# Frontend Engineering Plan V2 (Vue 3/Pinia)

## 1. Component Strategy: The "Dynamic Bridge"
Menggunakan `<component :is="currentEngine" />` untuk berganti tampilan secara instan berdasarkan data dari Rust.

## 2. Pinia State Management (`testStore`)
- `currentTool`: Data soal saat ini.
- `packageQueue`: Daftar antrian alat uji selanjutnya.
- `violationLog`: Counter percobaan kecurangan (blur, shortcut).

## 3. Special UI Implementations
- **Kraepelin Module**: Optimalisasi render kolom angka dalam jumlah besar agar tetap ringan (Virtual Scrolling).
- **Canvas Module**: Fitur menggambar (Wartegg/DAP) dengan dukungan undo/redo dan ekspor ke Base64 terenkripsi.
- **Instruction Overlay**: Halaman transisi wajib baca sebelum memulai sub-tes baru.

## 4. Surveillance UI
- Picture-in-Picture (PiP) mini untuk monitor wajah sendiri (efek psikologis).
- Peringatan visual jika wajah tidak terdeteksi atau sistem mendeteksi orang lain.