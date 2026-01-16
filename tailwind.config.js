/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class', // Manual toggle support
  theme: {
    extend: {
      colors: {
        eling: {
          // Light Mode Colors (Siang/Fokus - Kejujuran Batin)
          light: {
            bg: '#F8FAFD',      // Putih Kebiruan - Bersih & Fokus
            card: '#FFFFFF',    // Putih Murni - Fokus Konten
            surface: '#EDF2FA', // Biru Pucat - Pemisah Area
            text: '#1F1F1F',    // Dark Gray - Readability
          },
          // Dark Mode Colors (Malam/Tenang - Refleksi Diri)
          dark: {
            bg: '#131314',      // Hitam Google - Ketenangan
            card: '#1E1F20',    // Deep Gray - Fokus Konten
            surface: '#2C2D2E', // Abu Gelap - Pemisah Area
            text: '#E3E3E3',    // Off-White - Readability
          },
          // Accent Colors (Identitas Robin)
          emerald: '#10B981',   // Hijau Robin - Kesadaran
          blue: '#4285F4',      // Biru AI - Teknologi
          purple: '#A142F4',    // Purple - Spiritual

          // Legacy support (for gradual migration)
          accent: '#10B981',
          danger: '#FFC107',
        }
      },
      fontFamily: {
        sans: ['Inter', 'Google Sans', 'Plus Jakarta Sans', 'sans-serif'],
        body: ['Inter', 'Roboto', 'Outfit', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      backgroundImage: {
        // Gemini-style gradients (The "Eling Glow")
        'gemini-gradient': 'linear-gradient(135deg, #10B981 0%, #4285F4 50%, #A142F4 100%)',
        'eling-gradient': 'linear-gradient(135deg, #10B981 0%, #4285F4 100%)',
        'eling-glow': 'linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(66, 133, 244, 0.1) 100%)',

        // Light mode backgrounds
        'light-gradient': 'linear-gradient(135deg, #F8FAFD 0%, #EDF2FA 100%)',

        // Dark mode backgrounds
        'dark-gradient': 'linear-gradient(135deg, #131314 0%, #1E1F20 100%)',
      },
      borderRadius: {
        '3xl': '1.5rem', // Bento Box style
        '4xl': '2rem',
      },
      backdropBlur: {
        'glass': '12px',
      }
    },
  },
  plugins: [],
}
