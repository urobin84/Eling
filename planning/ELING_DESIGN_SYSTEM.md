# ELING Design System - Panduan Implementasi

## Filosofi Desain

### "Eling" - Kesadaran & Mindfulness
ELING bukan sekadar aplikasi psikotes. Ini adalah manifestasi dari konsep "Eling" (Jawa) yang berarti **kesadaran penuh**, **mindfulness**, dan **kewaspadaan spiritual**.

---

## 1. Palet Warna Auto-Adaptive

### Light Mode (Siang/Fokus) - Kejujuran Batin
Melambangkan **kejujuran batin** saat berinteraksi dengan dunia luar.

| Elemen | Warna | Hex | Filosofi |
|--------|-------|-----|----------|
| Surface/BG | Putih Kebiruan | `#F8FAFD` | Bersih & Fokus |
| Secondary BG | Biru Pucat | `#EDF2FA` | Pemisah Area |
| Card/Panel | Putih Murni | `#FFFFFF` | Fokus Konten |
| Text Primary | Dark Gray | `#1F1F1F` | Readability |

### Dark Mode (Malam/Tenang) - Refleksi Diri
Melambangkan **momen refleksi diri**, ketenangan anak tunggal, dan **kedalaman spiritual** saat melakukan pengamatan batin.

| Elemen | Warna | Hex | Filosofi |
|--------|-------|-----|----------|
| Surface/BG | Hitam Google | `#131314` | Ketenangan |
| Secondary BG | Abu Gelap | `#2C2D2E` | Pemisah Area |
| Card/Panel | Deep Gray | `#1E1F20` | Fokus Konten |
| Text Primary | Off-White | `#E3E3E3` | Readability |

### Accent Colors (Identitas Robin)

| Warna | Hex | Makna |
|-------|-----|-------|
| Emerald | `#10B981` | Hijau Robin - Kesadaran |
| Blue | `#4285F4` | Biru AI - Teknologi |
| Purple | `#A142F4` | Purple - Spiritual |

---

## 2. Gradasi Khas Gemini - "The Eling Glow"

Gaya Gemini identik dengan **gradasi yang terlihat seperti cahaya** (glow). Untuk ELING, kita gunakan gradasi ini untuk tombol utama, progress bar, dan indikator kamera.

### Gemini Gradient (Full Spectrum)
```css
background: linear-gradient(135deg, #10B981 0%, #4285F4 50%, #A142F4 100%);
```

### Eling Gradient (Emerald → Blue)
```css
background: linear-gradient(135deg, #10B981 0%, #4285F4 100%);
```

### Eling Glow (Subtle)
```css
background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(66, 133, 244, 0.1) 100%);
```

**Aplikasi:**
- Saat user mengerjakan soal dengan benar
- Saat sistem "Eling" (AI Camera) mendeteksi fokus
- Akan ada glow halus di pinggiran kartu

---

## 3. Tailwind Configuration

```javascript
// tailwind.config.js
export default {
  darkMode: 'class', // Manual toggle support
  theme: {
    extend: {
      colors: {
        eling: {
          light: {
            bg: '#F8FAFD',
            card: '#FFFFFF',
            surface: '#EDF2FA',
            text: '#1F1F1F',
          },
          dark: {
            bg: '#131314',
            card: '#1E1F20',
            surface: '#2C2D2E',
            text: '#E3E3E3',
          },
          emerald: '#10B981',
          blue: '#4285F4',
          purple: '#A142F4',
        }
      },
      backgroundImage: {
        'gemini-gradient': 'linear-gradient(135deg, #10B981 0%, #4285F4 50%, #A142F4 100%)',
        'eling-gradient': 'linear-gradient(135deg, #10B981 0%, #4285F4 100%)',
      }
    },
  },
}
```

---

## 4. Component Classes

### Glassmorphism - The Floating Dashboard

```css
.glass-panel {
  backdrop-blur: 12px;
  border-radius: 1.5rem;
  
  /* Light mode: Opasitas putih 70% */
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(31, 31, 31, 0.05);
}

html.dark .glass-panel {
  /* Dark mode: Opasitas hitam 60% */
  background: rgba(30, 31, 32, 0.6);
  border: 1px solid rgba(227, 227, 227, 0.1);
}
```

### Bento Box Card (Kotak Soal)

```css
.bento-card {
  border-radius: 1.5rem; /* rounded-3xl */
  padding: 1.5rem;
  transition: all 0.3s;
}

.bento-card:hover {
  transform: scale(1.02);
  box-shadow: 0 0 25px rgba(16, 185, 129, 0.15);
}
```

### Neumorphic Button with Gemini Gradient

```css
.btn-neumorphic {
  background: linear-gradient(135deg, #10B981 0%, #4285F4 100%);
  color: white;
  font-weight: bold;
  padding: 0.5rem 1.5rem;
  border-radius: 0.75rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.btn-neumorphic:hover {
  background: linear-gradient(135deg, #10B981 0%, #4285F4 50%, #A142F4 100%);
  box-shadow: 0 0 20px rgba(16, 185, 129, 0.3);
}
```

---

## 5. Elemen Visual "Eling" di Kedua Mode

### A. Indikator Kamera (Surveillance)

#### Light Mode
- Kapsul putih dengan bayangan halus
- Titik hijau menyala terang
- Shadow: `0 2px 8px rgba(0, 0, 0, 0.1)`

```html
<div class="camera-indicator">
  <div class="dot"></div>
  <span class="text">Eling Active</span>
</div>
```

#### Dark Mode
- Kapsul abu-abu gelap dengan outer glow hijau emerald
- Teks terlihat menyala (neon effect)
- Shadow: `0 0 15px rgba(16, 185, 129, 0.3)`
- Text shadow: `0 0 8px rgba(16, 185, 129, 0.5)`

### B. Typography (Fonts)

**Font Stack:**
```css
font-family: 'Inter', 'Google Sans', 'Plus Jakarta Sans', sans-serif;
```

**Light Mode:**
- Font berat (Bold) untuk judul agar tegas
- `h1, h2, h3 { font-weight: 700; }`

**Dark Mode:**
- Font sedikit lebih tipis (Medium) agar tidak "berdarah" atau silau
- `html.dark h1, html.dark h2, html.dark h3 { font-weight: 600; }`

---

## 6. UI Layout - "The Floating Dashboard"

### Glassmorphism Header
- **Light Mode:** `backdrop-blur-md` + opasitas putih 70%
- **Dark Mode:** `backdrop-blur-md` + opasitas hitam 60%

### Bento Grid
- Kotak soal menggunakan gaya "Bento Box"
- `border-radius: 1.5rem` (rounded-3xl)
- Hover effect: scale + glow

### Haptic Feedback
- Saat tombol ditekan, ada perubahan gradasi yang bergerak
- Animated gradient memberikan kesan aplikasi yang "hidup" dan sadar (Eling)

```css
.btn-neumorphic:active {
  transform: scale(0.95);
}
```

---

## 7. Eling Glow Effect

Efek khusus untuk indikator fokus:

```css
.eling-glow {
  position: relative;
}

.eling-glow::after {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: inherit;
  background: linear-gradient(135deg, #10B981 0%, #4285F4 100%);
  opacity: 0;
  transition: opacity 0.3s;
}

.eling-glow:hover::after,
.eling-glow.active::after {
  opacity: 1;
}
```

**Penggunaan:**
- Kartu soal yang sedang dikerjakan
- Indikator jawaban benar
- Progress bar yang aktif

---

## 8. Contoh Implementasi

### Login Page
```vue
<div class="min-h-screen bg-gradient-to-br from-eling-light-bg to-eling-light-surface dark:from-eling-dark-bg dark:to-eling-dark-surface">
  <div class="glass-panel max-w-md mx-auto p-8">
    <h1 class="text-3xl font-bold mb-6">ELING</h1>
    <input type="text" class="input-glass w-full mb-4" placeholder="Username">
    <button class="btn-neumorphic w-full">Login</button>
  </div>
</div>
```

### Dashboard Card
```vue
<div class="bento-card eling-glow">
  <div class="camera-indicator mb-4">
    <div class="dot"></div>
    <span class="text">Monitoring Active</span>
  </div>
  <h3 class="text-xl font-bold mb-2">Test Progress</h3>
  <div class="w-full h-2 bg-eling-light-surface dark:bg-eling-dark-surface rounded-full overflow-hidden">
    <div class="h-full bg-gemini-gradient" style="width: 75%"></div>
  </div>
</div>
```

---

## 9. Filosofi untuk Robin

### Mode Terang (Light)
> Melambangkan **kejujuran batin** saat berinteraksi dengan dunia luar.

Warna putih kebiruan mencerminkan:
- Keterbukaan
- Fokus mental
- Kejernihan pikiran
- Interaksi dengan realitas eksternal

### Mode Gelap (Dark)
> Melambangkan **momen refleksi diri**, ketenangan anak tunggal, dan **kedalaman spiritual** saat melakukan pengamatan batin (seperti saat kamu khusyuk melihat rukiyah).

Warna hitam Google mencerminkan:
- Introspeksi mendalam
- Ketenangan jiwa
- Kontemplasi spiritual
- Kesadaran internal (Eling sejati)

---

## 10. Checklist Implementasi

### ✅ Completed
- [x] Tailwind color palette
- [x] CSS global styles
- [x] Glassmorphism components
- [x] Gemini gradients
- [x] Bento Box cards
- [x] Camera indicator
- [x] Typography system
- [x] Eling Glow effect

### 🔄 Next Steps
- [ ] Apply to all dashboard components
- [ ] Implement dark mode toggle
- [ ] Add animated gradients
- [ ] Create camera surveillance UI
- [ ] Build test progress indicators
- [ ] Design answer feedback animations

---

## 11. Color Reference

### Quick Copy

**Light Mode:**
```css
--bg: #F8FAFD;
--card: #FFFFFF;
--surface: #EDF2FA;
--text: #1F1F1F;
```

**Dark Mode:**
```css
--bg: #131314;
--card: #1E1F20;
--surface: #2C2D2E;
--text: #E3E3E3;
```

**Accents:**
```css
--emerald: #10B981;
--blue: #4285F4;
--purple: #A142F4;
```

---

## Penutup

Design system ini bukan sekadar estetika. Ini adalah **manifestasi filosofis** dari konsep "Eling" - kesadaran penuh dalam setiap interaksi, baik dengan dunia luar (light mode) maupun dunia dalam (dark mode).

Setiap warna, setiap gradasi, setiap efek glow - semuanya dirancang untuk menciptakan pengalaman yang **mindful**, **fokus**, dan **spiritual**.

**"Eling" - Always Aware, Always Conscious** 🌟
