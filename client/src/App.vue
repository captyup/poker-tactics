<template>
  <div class="h-screen w-screen bg-gray-900 text-white overflow-hidden font-sans relative">
    <LanguageSwitcher />
    <div v-if="!game.roomId" class="h-full flex flex-col items-center justify-center gap-4">
      <h1 class="text-6xl font-bold mb-8 text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-red-600 drop-shadow-lg">
        {{ $t('app.title') }}
      </h1>
      
      <div v-if="!mode" class="flex flex-col gap-4 w-96">
          <button 
            @click="startNewGame"
            class="w-full bg-gradient-to-r from-green-600 to-green-800 hover:from-green-500 hover:to-green-700 text-white font-bold py-4 rounded-lg transform transition hover:scale-105 shadow-xl text-xl"
          >
            {{ $t('app.startNewGame') }}
          </button>
          <button 
            @click="enterJoinMode"
            class="w-full bg-gradient-to-r from-blue-600 to-blue-800 hover:from-blue-500 hover:to-blue-700 text-white font-bold py-4 rounded-lg transform transition hover:scale-105 shadow-xl text-xl"
          >
            {{ $t('app.joinGame') }}
          </button>
      </div>

      <div v-else class="bg-gray-800 p-8 rounded-lg shadow-2xl flex flex-col gap-4 w-96 border border-gray-700 relative">
        <button @click="mode = null" class="absolute top-2 right-4 text-gray-400 hover:text-white text-xl">
            ✕
        </button>
        <h2 class="text-2xl font-bold text-center mb-4 text-blue-400">
            {{ mode === 'start' ? $t('app.startNewGame') : $t('app.joinGame') }}
        </h2>

        <div v-if="mode === 'join'">
           <label class="block text-sm font-bold mb-2 text-gray-300">{{ $t('app.roomId') }}</label>
           <input v-model="roomInput" class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-white focus:outline-none focus:border-blue-500" placeholder="e.g. 123456" @keyup.enter="join" />
        </div>
        
        <div v-else>
             <label class="block text-sm font-bold mb-2 text-gray-300">{{ $t('app.roomId') }}</label>
             <div class="text-2xl font-mono font-bold text-green-400 text-center py-2 bg-gray-900 rounded border border-gray-700 mb-2">
                 {{ roomInput }}
             </div>
        </div>

        <button 
          @click="join" 
          class="w-full bg-gradient-to-r from-blue-600 to-blue-800 hover:from-blue-500 hover:to-blue-700 text-white font-bold py-3 rounded mt-4 transform transition hover:scale-105"
          :disabled="!roomInput"
        >
          {{ $t('app.enter') }}
        </button>
      </div>
    </div>
    
    <GameBoard v-else />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useGameStore } from '@/stores/game';
import GameBoard from '@/components/GameBoard.vue';
import LanguageSwitcher from '@/components/LanguageSwitcher.vue';

const game = useGameStore();
const roomInput = ref('');
const mode = ref<'start' | 'join' | null>(null);

onMounted(() => {
    // Check URL query params
    const params = new URLSearchParams(window.location.search);
    const roomParam = params.get('room');
    if (roomParam) {
        roomInput.value = roomParam;
        mode.value = 'join';
    }
});

function startNewGame() {
    // Generate 6-digit random number
    const randomId = Math.floor(100000 + Math.random() * 900000).toString();
    roomInput.value = randomId;
    mode.value = 'start';
}

function enterJoinMode() {
    mode.value = 'join';
    roomInput.value = '';
}

function join() {
  if (roomInput.value) {
    // Generate random 6-digit Player ID
    const randomPlayerId = Math.floor(100000 + Math.random() * 900000).toString();

    if (mode.value === 'start' || (mode.value === 'join' && !window.location.search.includes('room='))) {
        // Update URL without reloading
        const url = new URL(window.location.href);
        url.searchParams.set('room', roomInput.value);
        window.history.pushState({}, '', url);
    }
    game.joinGame(roomInput.value, randomPlayerId);
  }
}
</script>

<style>
body {
    margin: 0;
    padding: 0;
}
</style>
