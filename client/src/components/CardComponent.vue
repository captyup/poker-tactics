<template>
  <div 
    class="card border-2 rounded p-2 w-20 h-32 flex flex-col items-center justify-between cursor-pointer transition-transform hover:-translate-y-2 relative bg-white select-none"
    :class="{
      'border-yellow-500 ring-4 ring-yellow-300': isSelected,
      'border-gray-800': !isSelected,
      'bg-yellow-100': isHero,
      'text-red-600': isRed,
      'text-black': !isRed,
      'opacity-50': isDisabled
    }"
    @click="handleClick"
  >
    <div class="self-start font-bold">{{ rankDisplay }}</div>
    <div class="text-2xl">{{ suitDisplay }}</div>
    <div class="font-bold text-lg border-2 border-current rounded-full w-8 h-8 flex items-center justify-center bg-white/80 z-10">{{ card.current_power }}</div>
    <div class="text-xs absolute bottom-1 right-1" title="Ability">{{ abilityIcon }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Card } from '@/types/poker';

const props = defineProps<{
  card: Card;
  isSelected?: boolean;
  isDisabled?: boolean;
}>();

const emit = defineEmits(['click']);

function handleClick() {
  if (!props.isDisabled) {
    emit('click');
  }
}

const isRed = computed(() => props.card.suit === 'Heart' || props.card.suit === 'Diamond');
const isHero = computed(() => props.card.ability === 'Hero');

const rankDisplay = computed(() => {
  const r = props.card.rank;
  if (typeof r === 'object' && 'Number' in r) return r.Number;
  if (r === 'Joker') return 'JK';
  return (r as string)[0]; // J, Q, K, A
});

const suitDisplay = computed(() => {
  switch (props.card.suit) {
    case 'Heart': return '♥';
    case 'Diamond': return '♦';
    case 'Spade': return '♠';
    case 'Club': return '♣';
    case 'Joker': return '🃏';
    default: return '?';
  }
});

const abilityIcon = computed(() => {
  switch (props.card.ability) {
    case 'IronGuard': return '🛡️';
    case 'Intel': return '👁️';
    case 'Medic': return '⚕️';
    case 'Hero': return '👑';
    case 'Burn': return '🔥';
    case 'Decoy': return '🔄';
    default: return '';
  }
});
</script>

<style scoped>
.card {
  box-shadow: 2px 2px 5px rgba(0,0,0,0.2);
}
</style>
