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
          <button
            @click="enterBrowseMode"
            class="w-full bg-gradient-to-r from-purple-600 to-purple-800 hover:from-purple-500 hover:to-purple-700 text-white font-bold py-4 rounded-lg transform transition hover:scale-105 shadow-xl text-xl"
          >
            {{ $t('app.browseRooms') || 'Browse Rooms' }}
          </button>
          <button 
            @click="showInstructions = true"
            class="w-full bg-gradient-to-r from-yellow-600 to-yellow-800 hover:from-yellow-500 hover:to-yellow-700 text-white font-bold py-4 rounded-lg transform transition hover:scale-105 shadow-xl text-xl"
          >
            {{ $t('instructions.title') }}
          </button>

          <a href="https://github.com/captyup/poker-tactics" target="_blank" rel="noopener noreferrer" 
             class="mt-4 mx-auto flex items-center gap-2 text-gray-400 hover:text-white transition-colors opacity-70 hover:opacity-100">
              <svg viewBox="0 0 24 24" class="w-6 h-6 fill-current">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              <span class="text-sm font-medium">View on GitHub</span>
          </a>
      </div>
      


      <div v-else class="bg-gray-800 p-8 rounded-lg shadow-2xl flex flex-col gap-4 w-96 border border-gray-700 relative">
        <button @click="mode = null" class="absolute top-2 right-4 text-gray-400 hover:text-white text-xl">
            ✕
        </button>
        <h2 class="text-2xl font-bold text-center mb-4 text-blue-400">
            {{ mode === 'start' ? $t('app.startNewGame') : (mode === 'join' ? $t('app.joinGame') : 'Browse Rooms') }}
        </h2>

        <!-- Profile Settings (Only show in start/join/browse if not joined) -->
        <div class="mb-4 border-b border-gray-700 pb-4">
            <label class="block text-sm font-bold mb-2 text-gray-300">Nickname</label>
            <input v-model="nicknameInput" class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-white focus:outline-none focus:border-blue-500 mb-2" placeholder="Your Name" />

            <label class="block text-sm font-bold mb-2 text-gray-300">Avatar</label>
            <div class="flex gap-2 overflow-x-auto pb-2 scrollbar-thin scrollbar-thumb-gray-600">
                <button
                    v-for="av in avatars"
                    :key="av"
                    @click="avatarInput = av"
                    class="text-2xl hover:bg-gray-700 rounded p-1 transition-colors"
                    :class="{ 'bg-blue-600 bg-opacity-50': avatarInput === av }"
                >
                    {{ av }}
                </button>
            </div>
        </div>

        <div v-if="mode === 'join'">
           <label class="block text-sm font-bold mb-2 text-gray-300">{{ $t('app.roomId') }}</label>
           <input v-model="roomInput" class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-white focus:outline-none focus:border-blue-500 mb-4" placeholder="e.g. 123456" @keyup.enter="join" />

           <button
              @click="join"
              class="w-full bg-gradient-to-r from-blue-600 to-blue-800 hover:from-blue-500 hover:to-blue-700 text-white font-bold py-3 rounded mt-4 transform transition hover:scale-105"
              :disabled="!roomInput || !nicknameInput"
            >
              {{ $t('app.enter') }}
            </button>
        </div>
        
        <div v-else-if="mode === 'start'">
             <label class="block text-sm font-bold mb-2 text-gray-300">{{ $t('app.roomId') }}</label>
             <div class="text-2xl font-mono font-bold text-green-400 text-center py-2 bg-gray-900 rounded border border-gray-700 mb-4">
                 {{ roomInput }}
             </div>

             <button
              @click="join"
              class="w-full bg-gradient-to-r from-blue-600 to-blue-800 hover:from-blue-500 hover:to-blue-700 text-white font-bold py-3 rounded mt-4 transform transition hover:scale-105"
              :disabled="!roomInput || !nicknameInput"
            >
              {{ $t('app.enter') }}
            </button>
        </div>

        <div v-else-if="mode === 'browse'">
            <div class="flex flex-col gap-2 max-h-60 overflow-y-auto">
                <div v-if="game.availableRooms.length === 0" class="text-center text-gray-500 py-4">
                    No active rooms found.
                </div>
                <div
                    v-for="room in game.availableRooms"
                    :key="room.id"
                    class="bg-gray-700 p-3 rounded flex justify-between items-center hover:bg-gray-600 cursor-pointer transition-colors"
                    @click="selectRoom(room.id)"
                >
                    <span class="font-mono font-bold">{{ room.id }}</span>
                    <span class="text-sm px-2 py-1 rounded" :class="room.player_count < 2 ? 'bg-green-600' : 'bg-red-600'">
                        {{ room.player_count }}/2
                    </span>
                </div>
            </div>
            <button @click="game.fetchRooms()" class="mt-4 text-blue-400 hover:text-white text-sm underline w-full text-center">Refresh List</button>
        </div>

      </div>
    </div>
    
    <GameBoard v-else />
    
    <GameInstructions v-if="showInstructions" @close="showInstructions = false" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useGameStore } from '@/stores/game';
import { useUserStore } from '@/stores/user';
import GameBoard from '@/components/GameBoard.vue';
import LanguageSwitcher from '@/components/LanguageSwitcher.vue';
import GameInstructions from '@/components/GameInstructions.vue';

const game = useGameStore();
const user = useUserStore();
const roomInput = ref('');
const playerIdInput = ref('');
const nicknameInput = ref(user.nickname);
const avatarInput = ref(user.avatar);
const mode = ref<'start' | 'join' | 'browse' | null>(null);
const showInstructions = ref(false);

const avatars = ['👤', '🤖', '🦊', '🐱', '🐶', '🦁', '🐯', '🐸', '🐵', '🐔', '🐧', '🦄', '👻', '👽', '💀', '🤡'];

onMounted(() => {
    // Check URL query params
    const params = new URLSearchParams(window.location.search);
    const roomParam = params.get('room');
    
    // Check localStorage
    const savedRoomId = localStorage.getItem('poker_tactics_roomId');
    const savedPlayerId = localStorage.getItem('poker_tactics_playerId');

    if (roomParam) {
        roomInput.value = roomParam;
        mode.value = 'join';
        if (savedPlayerId) {
             playerIdInput.value = savedPlayerId;
        }
    } else if (savedRoomId && savedPlayerId) {
        // If no URL param, but we have saved state, we can pre-fill
        roomInput.value = savedRoomId;
        playerIdInput.value = savedPlayerId;
    }
    
    // If no player ID yet, generate one random default
    if (!playerIdInput.value) {
        playerIdInput.value = Math.floor(100000 + Math.random() * 900000).toString();
    }

    // Sync inputs if store has data
    nicknameInput.value = user.nickname;
    avatarInput.value = user.avatar;
});

function startNewGame() {
    // Generate 6-digit random number
    const randomId = Math.floor(100000 + Math.random() * 900000).toString();
    roomInput.value = randomId;
    // Generate formatted player ID if empty
    if (!playerIdInput.value) {
        playerIdInput.value = Math.floor(100000 + Math.random() * 900000).toString();
    }
    mode.value = 'start';
}

function enterJoinMode() {
    mode.value = 'join';
    // Keep existing roomInput if any (from localStorage or URL)
    if (!roomInput.value) {
        roomInput.value = '';
    }
     // Keep existing playerInput if any
    if (!playerIdInput.value) {
        playerIdInput.value = Math.floor(100000 + Math.random() * 900000).toString();
    }
}

function enterBrowseMode() {
    mode.value = 'browse';
    game.fetchRooms();
     // Keep existing playerInput if any
    if (!playerIdInput.value) {
        playerIdInput.value = Math.floor(100000 + Math.random() * 900000).toString();
    }
}

function selectRoom(id: string) {
    roomInput.value = id;
    join();
}

function join() {
  if (roomInput.value && nicknameInput.value) {
    const playerId = playerIdInput.value;

    // Save Profile
    user.setProfile(nicknameInput.value, avatarInput.value);

    if (mode.value === 'start' || (mode.value === 'join' && !window.location.search.includes('room='))) {
        // Update URL without reloading
        const url = new URL(window.location.href);
        url.searchParams.set('room', roomInput.value);
        window.history.pushState({}, '', url);
    }
    
    // Save to localStorage
    localStorage.setItem('poker_tactics_roomId', roomInput.value);
    localStorage.setItem('poker_tactics_playerId', playerId);

    game.joinGame(roomInput.value, playerId);
  }
}
</script>

<style>
body {
    margin: 0;
    padding: 0;
}
</style>
