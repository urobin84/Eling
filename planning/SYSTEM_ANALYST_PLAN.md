# System Analyst Plan V2 - Modular Psychotest Platform

## 1. System Vision
Membangun platform psikotes yang mampu menangani 20+ alat uji (TIU, IST, DISC, Kraepelin, dll) dengan urutan yang dapat dikustomisasi per sesi, dilindungi oleh Zero Trust Architecture.

## 2. Core Engine: The "Universal Test Player"
Alih-alih membuat satu halaman per tes, sistem menggunakan 5 Template Engine:
1. **ChoiceEngine**: (TIU, IST, CFIT, MBTI, DISC, 16PF, APM, SPM, WPT, RMIB, RIASEC).
2. **PairEngine**: (EPPS, PAPI Kostick).
3. **SpeedEngine**: (Kraepelin, Pauli, Klose).
4. **ProjectiveEngine**: (Wartegg, DAP, HTP menggunakan Canvas API).
5. **LeadershipEngine**: (MSDT).

## 3. Custom Event Sequencer
Admin dapat mendefinisikan "Test Package" dalam format JSON:
`Package_A = [TIU, DISC, Kraepelin]`.
Backend Rust akan mengontrol transisi otomatis, instruksi, dan perpindahan antar alat uji.

## 4. Security & ZTA
- **Identity Trust**: Verifikasi wajah via kamera setiap perpindahan sub-tes.
- **Process Trust**: Deteksi aplikasi terlarang (Browser, Remote Desktop).
- **Network Trust**: Aplikasi bekerja offline (SQLite) untuk mencegah kebocoran via network sniffing.