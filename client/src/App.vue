<template>
  <div class="h-screen w-screen bg-gray-900 text-white overflow-hidden font-sans relative">
    <LanguageSwitcher />
    <div v-if="!game.roomId" class="h-full flex flex-col items-center justify-center gap-4">
      <h1 class="text-6xl font-bold mb-8 text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-red-600 drop-shadow-lg">
        {{ $t('app.title') }}
      </h1>
      <div class="bg-gray-800 p-8 rounded-lg shadow-2xl flex flex-col gap-4 w-96 border border-gray-700">
        <div>
           <label class="block text-sm font-bold mb-2 text-gray-300">{{ $t('app.roomId') }}</label>
           <input v-model="roomInput" class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-white focus:outline-none focus:border-blue-500" placeholder="e.g. room1" @keyup.enter="join" />
        </div>
        <div>
           <label class="block text-sm font-bold mb-2 text-gray-300">{{ $t('app.playerId') }}</label>
           <input v-model="playerInput" class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-white focus:outline-none focus:border-blue-500" placeholder="e.g. Geralt" @keyup.enter="join" />
        </div>
        <button 
          @click="join" 
          class="w-full bg-gradient-to-r from-blue-600 to-blue-800 hover:from-blue-500 hover:to-blue-700 text-white font-bold py-3 rounded mt-4 transform transition hover:scale-105"
          :disabled="!roomInput || !playerInput"
        >
          {{ $t('app.enter') }}
        </button>
      </div>
    </div>
    
    <GameBoard v-else />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useGameStore } from '@/stores/game';
import GameBoard from '@/components/GameBoard.vue';
import LanguageSwitcher from '@/components/LanguageSwitcher.vue';

const game = useGameStore();
const roomInput = ref('');
const playerInput = ref('');

function join() {
  if (roomInput.value && playerInput.value) {
    game.joinGame(roomInput.value, playerInput.value);
  }
}
</script>

<style>
body {
    margin: 0;
    padding: 0;
}
</style>
