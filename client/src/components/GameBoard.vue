<template>
  <div class="game-board w-full h-full flex flex-col bg-green-800 p-4 text-white overflow-hidden" v-if="game.gameState">
    <!-- Opponent Area -->
    <div class="opponent-area flex-1 flex flex-col items-center justify-start border-b border-green-700/50 pb-2 mt-8 md:mt-0">
      <div class="flex flex-wrap items-center justify-center gap-2 mb-2 px-2">
        <h2 class="text-xl font-bold mr-2">{{ $t('game.opponent') }}</h2>
        <div class="badge bg-blue-600 px-2 py-1 rounded text-sm">{{ $t('game.score') }}: {{ opponent?.current_score }}</div>
        <div class="badge bg-yellow-600 px-2 py-1 rounded text-sm">{{ $t('game.rounds') }}: {{ opponent?.rounds_won }}</div>
        <div v-if="opponent?.passed" class="badge bg-gray-500 px-2 py-1 rounded text-sm">{{ $t('game.passed') }}</div>
        <div class="badge bg-purple-600 px-2 py-1 rounded text-sm">{{ $t('game.hand') }}: {{ opponent?.hand.length }}</div>
      </div>
      
      <!-- Opponent Board -->
      <div class="flex gap-2 flex-wrap justify-center min-h-[140px] overflow-y-auto max-h-[30vh]">
        <CardComponent 
          v-for="card in opponent?.board" 
          :key="card.id" 
          :card="card" 
          class="scale-75 origin-top"
        />
      </div>
    </div>

    <!-- Center Info -->
    <div class="center-area min-h-12 py-1 flex flex-wrap items-center justify-between px-4 bg-green-900/30 gap-2">
      <div class="text-sm md:text-lg">{{ $t('game.round') }}: {{ game.gameState.round_count }} / 3</div>
      <div class="text-lg md:text-2xl font-bold text-yellow-400 text-center flex-1 mx-2">
        {{ message }}
      </div>
      <div class="text-sm md:text-lg">{{ $t('game.phase') }}: {{ game.gameState.phase }}</div>
    </div>

    <!-- My Area -->
    <div class="my-area flex-1 flex flex-col items-center justify-end pt-2 border-t border-green-700/50 overflow-hidden">
       <!-- My Board -->
      <div class="flex gap-2 flex-wrap justify-center min-h-[100px] md:min-h-[140px] mb-2 overflow-y-auto max-h-[30vh]">
        <CardComponent 
          v-for="card in me?.board" 
          :key="card.id" 
          :card="card" 
          class="scale-75 origin-bottom"
          @click="handleBoardClick(card)"
        />
      </div>

      <!-- My Info -->
      <div class="flex flex-wrap items-center justify-center gap-2 mb-2 px-2">
        <h2 class="text-xl font-bold text-green-300 mr-2">{{ $t('game.you') }}</h2>
        <div class="badge bg-blue-600 px-2 py-1 rounded text-sm">{{ $t('game.score') }}: {{ me?.current_score }}</div>
        <div class="badge bg-yellow-600 px-2 py-1 rounded text-sm">{{ $t('game.rounds') }}: {{ me?.rounds_won }}</div>
        <div v-if="me?.passed" class="badge bg-gray-500 px-2 py-1 rounded text-sm">{{ $t('game.passed') }}</div>
        
        <button 
          v-if="canAction && !me?.passed && game.gameState.phase === 'Playing'"
          class="bg-red-600 hover:bg-red-700 text-white font-bold py-1 px-4 rounded text-sm"
          @click="game.passTurn()"
        >
          {{ $t('game.pass') }}
        </button>

        <button 
            v-if="game.gameState.phase === 'Mulligan' && !me?.passed"
            class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-4 rounded text-sm"
            @click="confirmMulligan"
        >
            {{ selectedHandCards.length > 0 ? $t('game.confirmSwap', { count: selectedHandCards.length }) : $t('game.confirmSwap_no_args') }}
        </button>
      </div>

      <!-- Hint Area -->
      <div v-if="selectedCardHint" class="mb-1 h-6 flex items-center justify-center w-full px-2">
          <div class="bg-black/50 px-2 py-1 rounded text-yellow-300 font-bold animate-pulse text-xs md:text-sm truncate max-w-full text-center">
              {{ selectedCardHint }} <span class="text-white/70 font-normal text-[10px] ml-1 hidden md:inline">{{ $t('game.tapToConfirm') }}</span>
          </div>
      </div>

      <!-- My Hand (Scrollable) -->
      <div class="w-full overflow-x-auto pb-4 pt-4 px-4 scrollbar-thin scrollbar-thumb-gray-400 scrollbar-track-transparent">
        <div class="flex flex-nowrap min-w-min justify-center -space-x-4 md:-space-x-6">
            <CardComponent 
            v-for="card in sortedHand" 
            :key="card.id" 
            :card="card" 
            :isSelected="selectedHandCards.includes(card.id) || pendingCardId === card.id"
            @click="handleHandClick(card)"
            class="transition-all hover:z-10 hover:-translate-y-4 hover:scale-105 flex-shrink-0"
            />
        </div>
      </div>
    </div>

    <!-- Medic Modal -->
    <div v-if="showMedicModal" class="fixed inset-0 bg-black/80 flex items-center justify-center z-50">
      <div class="bg-white text-black p-6 rounded-lg max-w-2xl w-full">
        <h3 class="text-xl font-bold mb-4">{{ $t('medic.title') }}</h3>
        <div class="flex gap-2 flex-wrap justify-center max-h-[60vh] overflow-y-auto bg-gray-100 p-4 rounded">
            <div v-if="me?.discard_pile.length === 0" class="text-gray-500">{{ $t('medic.empty') }}</div>
            <CardComponent 
                v-for="card in validMedicTargets" 
                :key="card.id" 
                :card="card" 
                @click="confirmMedic(card.id)"
            />
        </div>
        <button class="mt-4 bg-gray-500 text-white px-4 py-2 rounded" @click="cancelMedic">{{ $t('medic.cancel') }}</button>
      </div>
    </div>

     <!-- Error/Status Overlay -->
    <div v-if="game.error" class="fixed top-4 right-4 bg-red-500 text-white p-4 rounded shadow-lg z-50 animate-bounce">
        {{ game.error }}
    </div>
    
    <div v-if="game.gameState.winner" class="fixed inset-0 bg-black/90 flex flex-col items-center justify-center z-50 text-white">
        <h1 class="text-6xl font-bold mb-8">
            {{ game.gameState.winner === game.playerId ? $t('message.victory') : (game.gameState.winner === 'Draw' ? $t('message.draw') : $t('message.defeat')) }}
        </h1>
        <button class="bg-white text-black text-xl px-8 py-3 rounded hover:bg-gray-200" @click="reload">{{ $t('message.playAgain') }}</button>
    </div>

  </div>
  <div v-else class="w-full h-full flex items-center justify-center text-white text-2xl">
      {{ $t('game.waiting') }}
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useGameStore } from '@/stores/game';
import CardComponent from './CardComponent.vue';
import type { Card } from '@/types/poker';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const game = useGameStore();

const me = computed(() => {
    if (!game.gameState || !game.playerId) return null;
    return game.gameState.players[game.playerId];
});

const opponent = computed(() => {
    if (!game.gameState || !game.playerId) return null;
    const opponentId = Object.keys(game.gameState.players).find(id => id !== game.playerId);
    return opponentId ? game.gameState.players[opponentId] : null;
});

const sortedHand = computed(() => {
    if (!me.value?.hand) return [];
    
    return [...me.value.hand].sort((a, b) => {
        const getPriority = (card: Card) => {
            // Function cards priority > 100
            // Number cards priority = base_power
            
            switch (card.ability) {
                case 'Hero': return 500;
                case 'Burn': return 400;
                case 'Medic': return 300;
                case 'Intel': return 200;
                case 'Decoy': return 100;
                case 'IronGuard': return 2; // Treat as number 2
                case 'None': 
                default:
                    return card.base_power;
            }
        };

        const pA = getPriority(a);
        const pB = getPriority(b);

        if (pA !== pB) {
            return pB - pA; // Descending
        }
        
        return 0;
    });
});

const canAction = computed(() => {
    return game.gameState?.current_turn === game.playerId;
});

const message = computed(() => {
    if (game.gameState?.winner) return t('message.gameOver');
    if (game.gameState?.phase === 'Mulligan') return t('message.mulliganPhase');
    if (canAction.value) return t('message.yourTurn');
    return t('message.opponentTurn');
});

// Interaction State
const selectedHandCards = ref<string[]>([]);
const targetMode = ref<'Decoy' | null>(null);
const showMedicModal = ref(false);
const pendingCardId = ref<string | null>(null); // Acts as "Selected Card" in Playing phase

const selectedCardHint = computed(() => {
    if (!pendingCardId.value || !me.value) return null;
    const card = me.value.hand.find(c => c.id === pendingCardId.value);
    if (!card) return null;

    if (card.ability === 'IronGuard') {
        const hasBond = me.value.board.some(c => c.ability === 'IronGuard');
        if (hasBond) {
            return t('card.hint.bondActive');
        }
        return t('card.hint.bondInactive');
    }

    switch (card.ability) {
        case 'Intel': return t('card.hint.spy');
        case 'Medic': return t('card.hint.medic');
        case 'Hero': return t('card.hint.hero');
        case 'Burn': return t('card.hint.scorch');
        case 'Decoy': return t('card.hint.decoy');
        default: return null;
    }
});

function handleHandClick(card: Card) {
    if (game.gameState?.phase === 'Mulligan') {
        // Toggle selection for mulligan
        if (selectedHandCards.value.includes(card.id)) {
            selectedHandCards.value = selectedHandCards.value.filter(id => id !== card.id);
        } else {
            if (selectedHandCards.value.length < 2) {
                selectedHandCards.value.push(card.id);
            }
        }
        return;
    }

    if (game.gameState?.phase === 'Playing' && canAction.value) {
        // Selection Logic: Click once to select, Click again to play
        
        // If clicking a different card, select it and reset other states
        if (pendingCardId.value !== card.id) {
            pendingCardId.value = card.id;
            targetMode.value = null; 
            showMedicModal.value = false;
            return;
        }

        // If clicking the SAME card (Double tap/Confirm) -> EXECUTE
        
        if (card.ability === 'Decoy') {
            targetMode.value = 'Decoy';
            // User now needs to click the board
            return;
        } 
        
        if (card.ability === 'Medic') {
             showMedicModal.value = true;
             return;
        } 
        
        // Normal Play (Standard, IronGuard, Spy, Hero, Scorch)
        game.playCard(card.id);
        pendingCardId.value = null; // Reset selection
    }
}

function handleBoardClick(card: Card) {
    if (targetMode.value === 'Decoy' && pendingCardId.value) {
        // Validate target (cannot be Hero)
        if (card.ability === 'Hero') {
             // Visual shake?
             return;
        }
        game.playCard(pendingCardId.value, card.id);
        cancelInteraction();
    }
}

function confirmMulligan() {
    game.mulligan(selectedHandCards.value);
    selectedHandCards.value = [];
}

function confirmMedic(targetId: string) {
    if (pendingCardId.value) {
        game.playCard(pendingCardId.value, targetId);
    }
    cancelInteraction();
}

function cancelMedic() {
    cancelInteraction();
}

function cancelInteraction() {
    targetMode.value = null;
    showMedicModal.value = false;
    pendingCardId.value = null;
    selectedHandCards.value = [];
}

const validMedicTargets = computed(() => {
    if (!me.value) return [];
    // Medic can only revive non-Hero units. 
    // Simplified: Just filter out Heroes.
    return me.value.discard_pile.filter(c => c.ability !== 'Hero');
});

function reload() {
    window.location.reload();
}
</script>
