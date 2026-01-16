# Deployment Plan - Eling Platform

> [!NOTE]
> Production deployment strategy untuk distribusi aplikasi ke end users.

---

## 1. Build Configuration

### 1.1 Tauri Build Settings

```json
// tauri.conf.json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Eling",
    "version": "1.0.0"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.eling.psychotest",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      },
      "macOS": {
        "minimumSystemVersion": "10.13",
        "signing Identity": null
      }
    }
  }
}
```

---

### 1.2 Build Commands

```bash
# Development build
cargo tauri dev

# Production build (all platforms)
cargo tauri build

# Platform-specific builds
cargo tauri build --target x86_64-pc-windows-msvc  # Windows
cargo tauri build --target x86_64-apple-darwin     # macOS Intel
cargo tauri build --target aarch64-apple-darwin    # macOS Apple Silicon
cargo tauri build --target x86_64-unknown-linux-gnu # Linux
```

---

## 2. Release Artifacts

### 2.1 Windows

**Output:**
```
src-tauri/target/release/bundle/
├── msi/
│   └── Eling_1.0.0_x64_en-US.msi   (Installer)
└── nsis/
    └── Eling_1.0.0_x64-setup.exe   (Alternative installer)
```

**Installer Features:**
- Silent install option: `Eling_setup.exe /S`
- Custom install location
- Desktop shortcut creation
- Start menu entry
- Auto-update capability (if implemented)

**System Requirements:**
- Windows 10 (build 1809) or later
- 64-bit processor
- 4 GB RAM minimum
- 500 MB disk space
- Webcam (for surveillance features)

---

### 2.2 macOS

**Output:**
```
src-tauri/target/release/bundle/
├── dmg/
│   ├── Eling_1.0.0_x64.dmg          (Intel)
│   └── Eling_1.0.0_aarch64.dmg      (Apple Silicon)
└── macos/
    └── Eling.app
```

**Distribution:**
- DMG for drag-and-drop installation
- Code signing required for Gatekeeper
- Notarization for macOS 10.15+

**System Requirements:**
- macOS 12 (Monterey) or later
- 4 GB RAM minimum
- 500 MB disk space

---

### 2.3 Linux

**Output:**
```
src-tauri/target/release/bundle/
├── appimage/
│   └── eling_1.0.0_amd64.AppImage
└── deb/
    └── eling_1.0.0_amd64.deb
```

**Installation:**
```bash
# AppImage (universal)
chmod +x eling_1.0.0_amd64.AppImage
./eling_1.0.0_amd64.AppImage

# Debian/Ubuntu
sudo dpkg -i eling_1.0.0_amd64.deb
sudo apt-get install -f  # Fix dependencies
```

**System Requirements:**
- Ubuntu 22.04 or equivalent
- 4 GB RAM minimum
- 500 MB disk space

---

## 3. Code Signing

### 3.1 Windows Code Signing

```bash
# Using signtool (Windows SDK)
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com Eling_setup.exe

# Verify signature
signtool verify /pa Eling_setup.exe
```

**Certificate Requirements:**
- Code Signing Certificate from trusted CA
- Valid for at least 1 year
- Organization validation (OV) recommended

---

### 3.2 macOS Code Signing

```bash
# Sign the app
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" Eling.app

# Create signed DMG
hdiutil create -volname "Eling" -srcfolder Eling.app -ov -format UDZO Eling_signed.dmg

# Notarize (required for macOS 10.15+)
xcrun notarytool submit Eling_signed.dmg --apple-id your@email.com --password app-specific-password --team-id TEAMID

# Staple ticket
xcrun stapler staple Eling_signed.dmg
```

---

## 4. Distribution Channels

### 4.1 Direct Download

**Setup:**
- Host on CDN (Cloudflare, AWS S3)
- SSL required
- Download verification (SHA256 checksums)

```
https://download.eling.app/v1.0.0/
├── Eling_1.0.0_x64_setup.exe
├── Eling_1.0.0_x64_setup.exe.sha256
├── Eling_1.0.0_x64.dmg
├── Eling_1.0.0_x64.dmg.sha256
├── Eling_1.0.0_aarch64.dmg
├── Eling_1.0.0_aarch64.dmg.sha256
├── eling_1.0.0_amd64.AppImage
└── eling_1.0.0_amd64.AppImage.sha256
```

**Download Page:**
```html
<!-- checksums.txt -->
# Eling v1.0.0 Checksums (SHA256)
abc123... Eling_1.0.0_x64_setup.exe
def456... Eling_1.0.0_x64.dmg
ghi789... eling_1.0.0_amd64.AppImage
```

---

### 4.2 Enterprise Deployment

**MSI Silent Install (Windows):**
```batch
REM Install for all users
msiexec /i Eling_1.0.0_x64.msi /qn ALLUSERS=1

REM Custom install location
msiexec /i Eling_1.0.0_x64.msi /qn INSTALLDIR="C:\Program Files\Eling"
```

**Group Policy Deployment:**
- Create GPO for software installation
- Assign MSI to computer/user
- Configure installation options

---

## 5. Auto-Update System (Optional)

### 5.1 Update Server

```json
// update-manifest.json
{
  "version": "1.0.1",
  "notes": "Bug fixes and performance improvements",
  "pub_date": "2026-02-01T00:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "url": "https://download.eling.app/v1.0.1/Eling_1.0.1_x64_setup.exe",
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNl..."
    },
    "darwin-x86_64": {
      "url": "https://download.eling.app/v1.0.1/Eling_1.0.1_x64.dmg",
      "signature": "..."
    }
  }
}
```

### 5.2 Client Update Check

```rust
// In Tauri app
use tauri::updater::UpdaterBuilder;

#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<String, String> {
    let updater = UpdaterBuilder::new()
        .url("https://update.eling.app/update-manifest.json")
        .build(&app)?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            // Update available
            Ok(format!("Update available: {}", update.version))
        }
        Ok(None) => Ok("Already up to date".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
```

---

## 6. Installation Verification

### 6.1 Post-Install Checks

```bash
# Windows
eling.exe --version
eling.exe --verify-install

# macOS/Linux
./eling --version
./eling --verify-install
```

**Verification Script:**
```rust
#[tauri::command]
fn verify_install() -> Result<InstallStatus, String> {
    let checks = vec![
        check_database_writable(),
        check_camera_access(),
        check_permissions(),
        check_config_files(),
    ];
    
    if checks.iter().all(|c| *c) {
        Ok(InstallStatus::Ok)
    } else {
        Err("Installation verification failed".to_string())
    }
}
```

---

## 7. Rollback Strategy

### 7.1 Version Management

```
Installation Directory:
C:\Program Files\Eling\
├── v1.0.0\          (Previous version, kept)
├── v1.0.1\          (Current version)
└── current -> v1.0.1  (Symlink)
```

### 7.2 Rollback Procedure

```bash
# Windows (PowerShell)
Stop-Process -Name eling
Remove-Item "C:\Program Files\Eling\current"
New-Item -ItemType SymbolicLink -Path "C:\Program Files\Eling\current" -Target "C:\Program Files\Eling\v1.0.0"
Start-Process "C:\Program Files\Eling\current\eling.exe"
```

---

## 8. Monitoring & Analytics

### 8.1 Installation Tracking

```rust
// Optional: Anonymous installation tracking (with user consent)
#[tauri::command]
async fn report_installation(
    version: String,
    platform: String,
    install_id: String,
) -> Result<(), String> {
    // Send to analytics server (anonymized)
    Ok(())
}
```

### 8.2 Crash Reporting

**Tool:** `sentry` (optional)

```rust
// Initialize Sentry
let _guard = sentry::init(("https://your-dsn@sentry.io/project", sentry::ClientOptions {
    release: sentry::release_name!(),
    ..Default::default()
}));

// Capture errors
sentry::capture_error(&error);
```

---

## 9. Uninstallation

### 9.1 Clean Uninstall

**Windows:**
- Standard uninstaller via "Add/Remove Programs"
- Optionally remove user data

**macOS:**
```bash
# Remove app
rm -rf /Applications/Eling.app

# Remove user data (optional)
rm -rf ~/Library/Application\ Support/com.eling.psychotest
```

**Linux:**
```bash
# Debian
sudo apt-get remove eling

# AppImage
rm ~/Applications/eling_1.0.0_amd64.AppImage
```

---

### 9.2 Data Cleanup Script

```rust
#[tauri::command]
async fn uninstall_cleanup(remove_user_data: bool) -> Result<(), String> {
    if remove_user_data {
        // Remove database
        fs::remove_file(get_db_path())?;
        
        // Remove logs
        fs::remove_dir_all(get_log_dir())?;
        
        // Remove config
        fs::remove_file(get_config_path())?;
    }
    
    Ok(())
}
```

---

## 10. Deployment Checklist

### Pre-Release
- [ ] All tests passing (unit, integration, E2E)
- [ ] Security scan completed (0 CRITICAL)
- [ ] Performance benchmarks met
- [ ] Code signed (all platforms)
- [ ] Release notes prepared
- [ ] Documentation updated

### Build & Package
- [ ] Version bumped in all files
- [ ] Production build successful (all platforms)
- [ ] Installers tested on clean VMs
- [ ] SHA256 checksums generated
- [ ] Notarization completed (macOS)

### Distribution
- [ ] Artifacts uploaded to CDN
- [ ] Download page updated
- [ ] Changelog published
- [ ] Email notification sent (if applicable)

### Post-Release
- [ ] Monitor crash reports
- [ ] Track installation metrics
- [ ] Monitor user feedback
- [ ] Prepare hotfix process if needed

---

## 11. Emergency Hotfix Process

**Timeline:** Within 24 hours for CRITICAL issues

```bash
# 1. Create hotfix branch
git checkout -b hotfix/v1.0.1 main

# 2. Fix the issue
# ... code changes ...

# 3. Fast-track testing (critical path only)
cargo test
cargo tauri dev  # Manual verification

# 4. Build and deploy
cargo tauri build
# Upload to CDN
# Notify users

# 5. Merge back
git checkout main
git merge hotfix/v1.0.1
git tag v1.0.1
```

---

## Summary

**Deployment Platforms:**
- Windows 10/11 (MSI/NSIS)
- macOS 12+ (DMG, signed & notarized)
- Linux (AppImage, .deb)

**Distribution:**
- Direct download from CDN
- Enterprise silent installation
- Optional auto-update

**Security:**
- All binaries code-signed
- SHA256 verification
- Secure update server

> [!IMPORTANT]
> Never skip code signing for production releases. Unsigned apps will trigger security warnings dan mengurangi user trust.
