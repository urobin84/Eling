/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        eling: {
          // --- GEMINI LIGHT MODE ---
          light: {
            bg: '#F0F4F9',      // Gemini Light Grayish Blue (Lebih fresh)
            card: '#FFFFFF',    // White
            surface: '#E9EEF6', // Gemini Light Hover/Active
            text: '#1F1F1F',    // Google Standard Text
            subtext: '#444746', // Secondary Text
          },
          // --- GEMINI DARK MODE ---
          dark: {
            bg: '#0E0E10',      // Deepest Obsidian (Gemini 2026 Standard)
            card: '#131314',    // Elevated Surface (Pengganti #1E1F20)
            surface: '#1E1F20', // Higher Elevation (Hover/Secondary)
            text: '#E3E3E3',    // Soft White
            subtext: '#C4C7C5', // Muted Gray
          },
          // --- GEMINI ACCENTS ---
          emerald: '#10B981',   // Tetap Hijau Robin (Identity)
          blue: '#4285F4',      // Google Blue
          purple: '#A142F4',    // Gemini Purple

          // Common
          border: 'rgba(255, 255, 255, 0.1)',
        }
      },
      fontFamily: {
        // Mengutamakan font Gemini
        sans: ['Google Sans', 'Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      backgroundImage: {
        // Animasi Gradasi Gemini yang ikonik
        'gemini-gradient': 'linear-gradient(70deg, #4285F4 0%, #10B981 50%, #A142F4 100%)',

        // Background Mesh untuk kesan spiritual Robin
        'eling-mesh-dark': `
          radial-gradient(at 0% 0%, rgba(16, 185, 129, 0.08) 0px, transparent 40%),
          radial-gradient(at 100% 100%, rgba(66, 133, 244, 0.08) 0px, transparent 40%)
        `,
        'eling-mesh-light': `
          radial-gradient(at 0% 0%, rgba(16, 185, 129, 0.05) 0px, transparent 35%),
          radial-gradient(at 100% 100%, rgba(66, 133, 244, 0.05) 0px, transparent 35%)
        `,
      },
      boxShadow: {
        // Shadow halus khas Gemini
        'gemini-soft': '0 1px 3px rgba(0,0,0,0.1), 0 1px 2px rgba(0,0,0,0.06)',
        'gemini-elevated': '0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -2px rgba(0,0,0,0.05)',
      }
    },
  },
  plugins: [],
}