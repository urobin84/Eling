<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import CandidateLayout from '@/components/templates/CandidateLayout.vue';
import BaseCard from '@/components/atoms/BaseCard.vue';

const router = useRouter();
const user = ref<any>(null);

onMounted(() => {
  const sessionStr = localStorage.getItem('user_session');
  if (sessionStr) {
    try {
      user.value = JSON.parse(sessionStr);
    } catch (e) {
      router.push('/login');
    }
  } else {
    router.push('/login');
  }
});

function goBack() {
  router.push('/dashboard');
}

function logout() {
  localStorage.removeItem('user_session');
  window.location.href = '/login';
}

function goToProfile() {
  router.push('/profile');
}

function goToInfo() {
  // Already on info page, do nothing or refresh
  router.push('/info');
}
</script>

<template>
  <CandidateLayout :username="user?.username || 'Candidate'" @logout="logout" @profile="goToProfile" @info="goToInfo">
    <div class="max-w-5xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <button @click="goBack" class="flex items-center gap-2 text-gray-600 dark:text-gray-400 hover:text-eling-emerald transition-colors mb-4">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
          Back to Dashboard
        </button>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">Informasi Aplikasi</h1>
        <p class="text-gray-600 dark:text-gray-300">Panduan lengkap penggunaan platform ELING Psychotest</p>
      </div>

      <!-- Overview -->
      <BaseCard padding="lg" class="mb-6">
        <div class="flex items-start gap-4">
          <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-eling-emerald to-emerald-600 flex items-center justify-center flex-shrink-0">
            <svg class="w-6 h-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <div>
            <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-2">Tentang ELING</h2>
            <p class="text-gray-600 dark:text-gray-300 leading-relaxed">
              ELING (Evaluasi Lingkungan Intelegensi) adalah platform psychotest modern yang dilengkapi dengan sistem monitoring dan analisis AI untuk memastikan integritas dan akurasi hasil tes psikologi.
            </p>
          </div>
        </div>
      </BaseCard>

      <!-- Test Flow -->
      <BaseCard padding="lg" class="mb-6">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
          <svg class="w-6 h-6 text-eling-emerald" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          Alur Pengerjaan Tes
        </h2>
        
        <div class="space-y-4">
          <div class="flex gap-4">
            <div class="flex-shrink-0 w-8 h-8 rounded-full bg-eling-emerald text-white flex items-center justify-center font-bold text-sm">1</div>
            <div>
              <h3 class="font-semibold text-gray-900 dark:text-white mb-1">Join Event</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Masukkan kode event 6 karakter yang diberikan oleh administrator untuk mendaftar ke sesi tes.</p>
            </div>
          </div>

          <div class="flex gap-4">
            <div class="flex-shrink-0 w-8 h-8 rounded-full bg-eling-emerald text-white flex items-center justify-center font-bold text-sm">2</div>
            <div>
              <h3 class="font-semibold text-gray-900 dark:text-white mb-1">Mulai Assessment</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Klik "START ASSESSMENT" pada event card untuk memulai tes. Anda akan diarahkan ke halaman instruksi.</p>
            </div>
          </div>

          <div class="flex gap-4">
            <div class="flex-shrink-0 w-8 h-8 rounded-full bg-eling-emerald text-white flex items-center justify-center font-bold text-sm">3</div>
            <div>
              <h3 class="font-semibold text-gray-900 dark:text-white mb-1">Baca Instruksi</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Pahami instruksi dengan baik sebelum memulai. Setiap jenis tes memiliki instruksi yang berbeda.</p>
            </div>
          </div>

          <div class="flex gap-4">
            <div class="flex-shrink-0 w-8 h-8 rounded-full bg-eling-emerald text-white flex items-center justify-center font-bold text-sm">4</div>
            <div>
              <h3 class="font-semibold text-gray-900 dark:text-white mb-1">Kerjakan Tes</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Jawab semua pertanyaan dengan jujur dan sesuai kondisi Anda. Perhatikan waktu yang tersedia.</p>
            </div>
          </div>

          <div class="flex gap-4">
            <div class="flex-shrink-0 w-8 h-8 rounded-full bg-eling-emerald text-white flex items-center justify-center font-bold text-sm">5</div>
            <div>
              <h3 class="font-semibold text-gray-900 dark:text-white mb-1">Submit & Selesai</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Setelah selesai, jawaban akan otomatis tersimpan dan Anda akan melihat halaman konfirmasi.</p>
            </div>
          </div>
        </div>
      </BaseCard>

      <!-- Monitoring System -->
      <BaseCard padding="lg" class="mb-6">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
          <svg class="w-6 h-6 text-eling-emerald" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
          </svg>
          Sistem Monitoring
        </h2>

        <div class="space-y-4">
          <div class="bg-blue-50 dark:bg-blue-500/10 border border-blue-200 dark:border-blue-500/20 rounded-lg p-4">
            <h3 class="font-semibold text-blue-900 dark:text-blue-300 mb-2 flex items-center gap-2">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              Camera Monitoring
            </h3>
            <p class="text-sm text-blue-800 dark:text-blue-200">
              Kamera Anda akan diaktifkan selama tes untuk memastikan integritas pengerjaan. Administrator dapat melihat live feed untuk monitoring real-time.
            </p>
          </div>

          <div class="bg-purple-50 dark:bg-purple-500/10 border border-purple-200 dark:border-purple-500/20 rounded-lg p-4">
            <h3 class="font-semibold text-purple-900 dark:text-purple-300 mb-2 flex items-center gap-2">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
              </svg>
              Screen Recording
            </h3>
            <p class="text-sm text-purple-800 dark:text-purple-200">
              Layar Anda akan direkam selama tes berlangsung. Rekaman ini digunakan untuk verifikasi dan analisis perilaku pengerjaan.
            </p>
          </div>

          <div class="bg-amber-50 dark:bg-amber-500/10 border border-amber-200 dark:border-amber-500/20 rounded-lg p-4">
            <h3 class="font-semibold text-amber-900 dark:text-amber-300 mb-2 flex items-center gap-2">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
              Penting!
            </h3>
            <ul class="text-sm text-amber-800 dark:text-amber-200 space-y-1 list-disc list-inside">
              <li>Pastikan kamera dan mikrofon berfungsi dengan baik</li>
              <li>Jangan menutup atau minimize browser selama tes</li>
              <li>Hindari membuka tab atau aplikasi lain</li>
              <li>Kerjakan di ruangan yang terang dan tenang</li>
            </ul>
          </div>
        </div>
      </BaseCard>

      <!-- AI Analysis -->
      <BaseCard padding="lg" class="mb-6">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
          <svg class="w-6 h-6 text-eling-emerald" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
          </svg>
          Analisis AI (Coming Soon)
        </h2>

        <div class="space-y-4">
          <p class="text-gray-600 dark:text-gray-300">
            Platform ELING akan dilengkapi dengan sistem analisis berbasis AI yang dapat:
          </p>

          <div class="grid md:grid-cols-2 gap-4">
            <div class="bg-gradient-to-br from-emerald-50 to-cyan-50 dark:from-emerald-900/20 dark:to-cyan-900/20 rounded-lg p-4 border border-emerald-200 dark:border-emerald-500/20">
              <h3 class="font-semibold text-gray-900 dark:text-white mb-2">📊 Analisis Pola Jawaban</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Mendeteksi pola jawaban yang tidak konsisten atau mencurigakan</p>
            </div>

            <div class="bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-900/20 dark:to-indigo-900/20 rounded-lg p-4 border border-blue-200 dark:border-blue-500/20">
              <h3 class="font-semibold text-gray-900 dark:text-white mb-2">🎥 Analisis Video</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Mendeteksi gerakan mata, ekspresi wajah, dan perilaku mencurigakan</p>
            </div>

            <div class="bg-gradient-to-br from-purple-50 to-pink-50 dark:from-purple-900/20 dark:to-pink-900/20 rounded-lg p-4 border border-purple-200 dark:border-purple-500/20">
              <h3 class="font-semibold text-gray-900 dark:text-white mb-2">⏱️ Analisis Waktu</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Menganalisis kecepatan menjawab dan pola waktu pengerjaan</p>
            </div>

            <div class="bg-gradient-to-br from-amber-50 to-orange-50 dark:from-amber-900/20 dark:to-orange-900/20 rounded-lg p-4 border border-amber-200 dark:border-amber-500/20">
              <h3 class="font-semibold text-gray-900 dark:text-white mb-2">📈 Rekomendasi</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">Memberikan insight dan rekomendasi berdasarkan hasil analisis</p>
            </div>
          </div>
        </div>
      </BaseCard>

      <!-- Tips -->
      <BaseCard padding="lg">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
          <svg class="w-6 h-6 text-eling-emerald" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
          Tips Pengerjaan
        </h2>

        <div class="grid md:grid-cols-2 gap-4">
          <div class="flex gap-3">
            <div class="flex-shrink-0 w-6 h-6 rounded-full bg-green-500 text-white flex items-center justify-center text-xs">✓</div>
            <p class="text-sm text-gray-600 dark:text-gray-300">Jawab dengan jujur sesuai kondisi Anda</p>
          </div>
          <div class="flex gap-3">
            <div class="flex-shrink-0 w-6 h-6 rounded-full bg-green-500 text-white flex items-center justify-center text-xs">✓</div>
            <p class="text-sm text-gray-600 dark:text-gray-300">Baca instruksi dengan teliti</p>
          </div>
          <div class="flex gap-3">
            <div class="flex-shrink-0 w-6 h-6 rounded-full bg-green-500 text-white flex items-center justify-center text-xs">✓</div>
            <p class="text-sm text-gray-600 dark:text-gray-300">Kerjakan di tempat yang nyaman</p>
          </div>
          <div class="flex gap-3">
            <div class="flex-shrink-0 w-6 h-6 rounded-full bg-green-500 text-white flex items-center justify-center text-xs">✓</div>
            <p class="text-sm text-gray-600 dark:text-gray-300">Pastikan koneksi internet stabil</p>
          </div>
          <div class="flex gap-3">
            <div class="flex-shrink-0 w-6 h-6 rounded-full bg-red-500 text-white flex items-center justify-center text-xs">✗</div>
            <p class="text-sm text-gray-600 dark:text-gray-300">Jangan membuka tab atau aplikasi lain</p>
          </div>
          <div class="flex gap-3">
            <div class="flex-shrink-0 w-6 h-6 rounded-full bg-red-500 text-white flex items-center justify-center text-xs">✗</div>
            <p class="text-sm text-gray-600 dark:text-gray-300">Jangan meminta bantuan orang lain</p>
          </div>
        </div>
      </BaseCard>
    </div>
  </CandidateLayout>
</template>
