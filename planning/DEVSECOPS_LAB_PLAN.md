# DevSecOps Lab Plan V3 (100% Free & Open Source Tools)

> [!IMPORTANT]
> Dokumen ini mendefinisikan strategi lengkap DevSecOps untuk Eling - Modular Psychotest Platform, mencakup testing automation, security hardening, compliance validation, dan continuous monitoring menggunakan tools open source.

---

## 1. Performance Testing & Optimization

### 1.1 Kraepelin Stress Test (High-Volume Input)
**Objective:** Memastikan aplikasi tetap responsif saat user menginput ribuan angka dalam waktu singkat dengan kamera aktif.

**Tools:**
- `criterion.rs` - Rust benchmarking framework
- `flamegraph` - CPU profiling visualization
- `heaptrack` - Memory allocation profiling

**Test Scenarios:**
```rust
// Benchmark target: < 16ms per input (60 FPS)
#[bench]
fn kraepelin_input_latency() {
    // Simulate 100 inputs/second for 10 minutes
    // Monitor: CPU usage, RAM, frame drops
}
```

**Acceptance Criteria:**
- ✅ Input latency < 16ms (60 FPS maintained)
- ✅ Memory growth < 50MB during entire test
- ✅ No camera frame drops
- ✅ UI remains responsive (no frozen frames)

**Optimization Strategy:**
- Virtual scrolling untuk render kolom angka
- Debouncing untuk auto-save
- Web Workers untuk background processing
- Canvas rendering optimization

---

### 1.2 Multi-Tab Rendering Performance
**Test:** Render 9 IST subtests dengan 100+ soal masing-masing.

**Metrics:**
- Initial load time < 2 seconds
- Tab switching < 100ms
- Total memory footprint < 200MB

---

## 2. Security Testing (Zero Trust Validation)

### 2.1 Bypass Attempt Simulation

#### A. OS Shortcut Blocking Test
**Tools:** `xdotool` (Linux), `AutoHotkey` (Windows), `cliclick` (macOS)

**Attack Scenarios:**
```bash
# Test Script: bypass_shortcuts.sh
- Cmd+Tab / Alt+Tab (Task Switching)
- Cmd+Q / Alt+F4 (Force Quit)
- Cmd+H (Hide Window)
- Cmd+M (Minimize)
- F11 (Fullscreen Toggle)
- PrintScreen (Screenshot)
- Cmd+Space (Spotlight/Search)
```

**Expected Result:** Semua shortcut di-reject, violation counter bertambah.

---

#### B. Surveillance Bypass Attack
**Attack Vectors:**
1. **Photo Spoofing:** Menampilkan foto wajah di layar HP
2. **Video Playback:** Memutar video wajah di background
3. **Multiple Faces:** Menghadirkan orang lain di frame

**Defense Strategy:**
- Face liveness detection (blink detection, micro-movements)
- Face consistency check (same person validation)
- Alert jika > 1 wajah terdeteksi dalam 5 detik berturut-turut

**Tools:**
- `opencv-rust` untuk face detection
- Liveness detection algorithm (challenge-response)

**Test Plan:**
```
Scenario 1: Photo Attack
- Expected: Deteksi "No face movement" dalam 3 detik
- Action: Log violation, pause test

Scenario 2: Multiple Faces
- Expected: Deteksi dalam 2 detik
- Action: Violation warning, countdown 10 detik untuk perbaiki

Scenario 3: Face Swap Mid-Test
- Expected: Face consistency mismatch alert
- Action: Test invalidated
```

---

#### C. Memory Forensics Attack
**Tools:** `GDB`, `radare2`, `volatility`

**Attack Goal:** Extract AES encryption key dari RAM.

**Defense Implementation:**
```rust
// Ephemeral key dengan memory zeroing
use zeroize::Zeroize;

struct EncryptionKey([u8; 32]);

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        self.0.zeroize(); // Overwrite memory on drop
    }
}
```

**Security Test:**
1. Dump process memory menggunakan GDB
2. Search untuk encryption artifacts
3. Verify key tidak ditemukan dalam plaintext

**Mitigation:**
- Key derivation menggunakan session-specific salt
- Key rotation setiap sub-test
- Memory locking (`mlock`) untuk sensitive data
- Anti-debugging checks

---

### 2.2 Process Monitoring Test

**Tools:** `sysmon` (Windows), `auditd` (Linux), `fs_events` (macOS)

**Test Scenarios:**
```yaml
Blacklisted Processes to Detect:
  - TeamViewer / AnyDesk (Remote Desktop)
  - Chrome / Firefox (Browser)
  - Discord / Slack (Communication)
  - Screenshot tools (Snagit, Lightshot)
  - Screen recording (OBS, Camtasia)
```

**Implementation:**
```rust
// Rust process scanner
fn detect_forbidden_processes() -> Vec<String> {
    let blacklist = ["chrome", "firefox", "teamviewer", "anydesk"];
    let running = get_running_processes();
    running.iter().filter(|p| blacklist.contains(&p.name)).collect()
}
```

**Test:** Jalankan aplikasi terlarang saat tes berlangsung.
**Expected:** Deteksi dalam < 5 detik, violation logged, test paused.

---

## 3. Automated Testing Pipeline

### 3.1 Static Analysis

#### A. Rust Code Security
**Tools:**
- `cargo-audit` - Dependency vulnerability scanning
- `cargo-clippy` - Linting & best practices
- `cargo-deny` - License & security policy enforcement

```bash
# CI Pipeline Step
cargo audit --deny warnings
cargo clippy -- -D warnings
cargo deny check
```

---

#### B. Secret Detection
**Tools:** `gitleaks`, `trufflehog`

```bash
# Pre-commit hook
gitleaks detect --source . --verbose
```

**Scan for:**
- API keys
- Database credentials
- Encryption keys in code
- Hardcoded passwords

---

#### C. SAST (Static Application Security Testing)
**Tools:** `semgrep`

```yaml
# .semgrep.yml
rules:
  - id: unsafe-deserialization
    pattern: serde_json::from_str($INPUT)
    message: "Validate input before deserialization"
    
  - id: sql-injection-risk
    pattern: format!("SELECT * FROM {}", $VAR)
    message: "Use parameterized queries"
```

---

### 3.2 Container Security

**Tools:** `Trivy`, `Grype`

```bash
# Scan Tauri Docker build image
trivy image --severity HIGH,CRITICAL eling-builder:latest
```

**Scan untuk:**
- OS vulnerabilities
- Dependency vulnerabilities
- Misconfigurations
- Exposed secrets

---

### 3.3 Virtual Desktop Testing

**Environment:** `Linux Webtop` (Docker-based virtual desktop)

**Purpose:** Test kiosk mode behavior di isolated environment.

```dockerfile
# Dockerfile.test-env
FROM linuxserver/webtop:ubuntu-kde
RUN apt-get update && apt-get install -y \
    x11vnc \
    xdotool \
    scrot
COPY eling-test.AppImage /app/
```

**Test Cases:**
1. Always-on-Top enforcement
2. Desktop escape attempts
3. Multi-monitor behavior
4. Screen resolution changes

---

## 4. Compliance & Privacy Validation

### 4.1 Data Privacy Audit

**Standards:** GDPR, PDPA (Indonesia)

**Compliance Checks:**
```yaml
Privacy Requirements:
  - ✅ Data minimization (only necessary data collected)
  - ✅ Encryption at rest (AES-256-GCM)
  - ✅ Secure deletion after test completion
  - ✅ No data transmission (offline-first)
  - ✅ User consent mechanism
  - ✅ Data export capability (JSON format)
```

**Validation Tests:**
```bash
# Test: Secure deletion after test
1. Complete a test session
2. Check SQLite file for leftover data
3. Verify encryption keys zeroed in memory
4. Attempt data recovery using forensic tools
```

**Expected:** Data tidak dapat di-recover setelah deletion.

---

### 4.2 Accessibility Compliance

**Standards:** WCAG 2.1 Level AA

**Tools:** `axe-core`, `pa11y`

```bash
# Accessibility testing
pa11y-ci --sitemap https://localhost:1420/sitemap.xml
```

**Checks:**
- Keyboard navigation support
- Screen reader compatibility
- Color contrast ratios
- Focus indicators
- Alt text untuk images

---

## 5. Continuous Integration Pipeline

### 5.1 GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline

on: [push, pull_request]

jobs:
  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Cargo Audit
        run: cargo audit
      
      - name: Secret Scan
        run: gitleaks detect
      
      - name: SAST
        run: semgrep --config=auto .

  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - name: Run Tests
        run: cargo test --all-features
      
      - name: Benchmarks
        run: cargo bench

  build:
    needs: [security-scan, test]
    runs-on: ubuntu-latest
    steps:
      - name: Build Tauri
        run: cargo tauri build
      
      - name: Container Scan
        run: trivy fs --scanners vuln,config .
```

---

### 5.2 Performance Monitoring

**Tools:** `hyperfine` (CLI benchmarking)

```bash
# Benchmark startup time
hyperfine --warmup 3 './eling-app'

# Expected: < 2 seconds cold start
```

---

## 6. Chaos Engineering

### 6.1 Failure Injection Tests

**Scenarios:**
```yaml
Camera Failures:
  - Camera suddenly disconnected
  - Permission revoked mid-test
  - Multiple camera sources
  
Resource Constraints:
  - Low memory (< 100MB available)
  - CPU throttling (50% max)
  - Disk full during save
  
Network (if applicable):
  - Intermittent connectivity
  - Proxy interference
```

**Expected Behaviors:**
- Graceful degradation
- Clear error messages
- State preservation
- Recovery mechanisms

---

### 6.2 Stress Testing

**Tools:** `stress-ng`

```bash
# Simulate resource pressure
stress-ng --cpu 4 --vm 2 --vm-bytes 1G --timeout 60s

# Run Eling during stress
./eling-app --test-mode
```

**Metrics:**
- App remains functional
- No crashes or data loss
- Performance degradation < 20%

---

## 7. Reporting & Documentation

### 7.1 Test Reports

**Format:** HTML + JSON

**Tools:** `cargo-tarpaulin` (coverage), `junit` (CI integration)

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/
```

**Metrics to Track:**
- Code coverage (target: >80%)
- Vulnerability count (target: 0 HIGH/CRITICAL)
- Performance regression (baseline comparison)
- Test pass rate (target: 100%)

---

### 7.2 Security Scorecard

**Automated using:** `scorecard` (OSSF tool)

```bash
# Check security best practices
scorecard --repo=github.com/eling/psychotest
```

**Checks:**
- Branch protection
- Code review enforcement
- Dependency update policy
- Security policy (SECURITY.md)
- Signed releases

---

## 8. Deployment Validation

### 8.1 Installation Testing

**Platforms:**
- Windows 10/11 (x64)
- macOS 12+ (Intel & Apple Silicon)
- Ubuntu 22.04+ (x64)

**Test Matrix:**
```yaml
Scenarios:
  - Fresh installation
  - Upgrade from previous version
  - Offline installation (no internet)
  - Installation to custom directory
  - Multi-user environment
```

---

### 8.2 Post-Deployment Monitoring

**Tools:** Built-in telemetry (optional, user consent)

**Metrics:**
- Crash rate
- Performance metrics (startup time, memory usage)
- Feature usage statistics
- Error logs (anonymized)

---

## 9. Continuous Security Monitoring

### 9.1 Dependency Monitoring

**Tools:** `Dependabot` (GitHub)

**Configuration:**
```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: cargo
    directory: "/"
    schedule:
      interval: weekly
    open-pull-requests-limit: 10
```

---

### 9.2 Vulnerability Alerts

**Integration:** GitHub Security Advisories

**Response SLA:**
- CRITICAL: Patch within 24 hours
- HIGH: Patch within 7 days
- MEDIUM: Patch within 30 days

---

## 10. Lab Environment Setup

### 10.1 Docker Compose Stack

```yaml
# docker-compose.lab.yml
version: '3.8'
services:
  test-runner:
    image: rust:latest
    volumes:
      - .:/workspace
    command: cargo test
  
  security-scanner:
    image: aquasec/trivy
    volumes:
      - .:/scan
    command: fs /scan
  
  webtop:
    image: linuxserver/webtop:ubuntu-kde
    ports:
      - "3000:3000"
    environment:
      - PUID=1000
      - PGID=1000
```

**Usage:**
```bash
# Start entire test lab
docker-compose -f docker-compose.lab.yml up

# Run specific test suite
docker-compose run test-runner cargo test --test security_tests
```

---

## Summary: DevSecOps Metrics Dashboard

| Category | Tool | Target | Automation |
|----------|------|--------|------------|
| Code Coverage | Tarpaulin | >80% | ✅ CI |
| Vulnerabilities | Cargo Audit | 0 Critical | ✅ CI |
| SAST Issues | Semgrep | 0 High | ✅ CI |
| Container Vulns | Trivy | 0 Critical | ✅ CI |
| Performance | Criterion | <16ms latency | ✅ Benchmark |
| Security Tests | Custom | 100% pass | ✅ E2E |
| Compliance | Manual | 100% checklist | 🔄 Quarterly |

---

> [!CAUTION]
> **Red Team Exercise:** Jadwalkan pentesting manual setiap 6 bulan untuk validasi security posture secara comprehensive.