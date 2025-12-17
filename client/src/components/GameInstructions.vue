<template>
  <div class="fixed inset-0 bg-black bg-opacity-80 z-50 flex items-center justify-center p-4 backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-gray-800 border border-gray-600 rounded-lg shadow-2xl w-full max-w-4xl h-[80vh] flex flex-col overflow-hidden relative" role="dialog" aria-modal="true">
      
      <!-- Header -->
      <div class="flex justify-between items-center p-6 border-b border-gray-700 bg-gray-900">
        <h2 class="text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-yellow-600">
          {{ $t('instructions.title') }}
        </h2>
        <button @click="$emit('close')" class="text-gray-400 hover:text-white transition-colors p-2 rounded-full hover:bg-gray-700">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Tabs -->
      <div class="flex border-b border-gray-700 bg-gray-800">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          @click="currentTab = tab.id"
          class="flex-1 py-4 px-6 text-sm font-bold uppercase tracking-wider transition-all duration-200 border-b-2"
          :class="currentTab === tab.id ? 'border-yellow-500 text-yellow-500 bg-gray-700/50' : 'border-transparent text-gray-400 hover:text-gray-200 hover:bg-gray-700/30'"
        >
          {{ $t(`instructions.tabs.${tab.id}`) }}
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-8 bg-gray-800 text-gray-300 leading-relaxed custom-scrollbar">
        
        <!-- Introduction Tab -->
        <div v-show="currentTab === 'intro'" class="space-y-6">
          <section>
            <h3 class="text-xl font-bold text-white mb-3">{{ $t('instructions.intro.goal.title') }}</h3>
            <p>{{ $t('instructions.intro.goal.desc') }}</p>
          </section>
          <section>
            <h3 class="text-xl font-bold text-white mb-3">{{ $t('instructions.intro.core_mechanics.title') }}</h3>
            <ul class="list-disc list-inside space-y-2 ml-2">
                <li v-for="(item, index) in coreMechanics" :key="index">{{ item }}</li>
            </ul>
          </section>
        </div>

        <!-- Cards Tab -->
        <div v-show="currentTab === 'cards'" class="space-y-8">
            
            <!-- Unit Cards -->
            <section>
                <h3 class="text-xl font-bold text-blue-400 mb-4 border-b border-gray-700 pb-2">{{ $t('instructions.cards.units.title') }}</h3>
                <div class="grid gap-6 md:grid-cols-2">
                    <div class="bg-gray-700/50 p-4 rounded-lg border border-gray-600">
                        <h4 class="font-bold text-white text-lg mb-2 flex items-center">
                            <span class="inline-block w-8 h-8 bg-gray-600 rounded-full text-center leading-8 mr-2 text-sm border border-gray-500">2</span>
                            {{ $t('instructions.cards.units.bond.name') }}
                        </h4>
                        <p class="text-sm">{{ $t('instructions.cards.units.bond.desc') }}</p>
                    </div>
                    <div class="bg-gray-700/50 p-4 rounded-lg border border-gray-600">
                        <h4 class="font-bold text-white text-lg mb-2 flex items-center">
                            <span class="inline-block w-8 h-8 bg-gray-600 rounded-full text-center leading-8 mr-2 text-sm border border-gray-500">3-9</span>
                            {{ $t('instructions.cards.units.soldier.name') }}
                        </h4>
                        <p class="text-sm">{{ $t('instructions.cards.units.soldier.desc') }}</p>
                    </div>
                    <div class="bg-gray-700/50 p-4 rounded-lg border border-gray-600">
                        <h4 class="font-bold text-white text-lg mb-2 flex items-center">
                            <span class="inline-block w-8 h-8 bg-gray-600 rounded-full text-center leading-8 mr-2 text-sm border border-gray-500">10</span>
                            {{ $t('instructions.cards.units.heavy.name') }}
                        </h4>
                        <p class="text-sm">{{ $t('instructions.cards.units.heavy.desc') }}</p>
                    </div>
                </div>
            </section>

             <!-- Special Cards -->
             <section>
                <h3 class="text-xl font-bold text-purple-400 mb-4 border-b border-gray-700 pb-2">{{ $t('instructions.cards.special.title') }}</h3>
                <div class="space-y-4">
                    <div v-for="card in specialCards" :key="card.key" class="bg-gray-700/30 p-4 rounded-lg border border-gray-600 flex flex-col md:flex-row gap-4">
                        <div class="flex-shrink-0">
                            <div class="w-12 h-16 bg-gray-200 rounded text-black font-bold flex items-center justify-center text-2xl shadow-md border-2" :class="card.borderColor">
                                {{ card.symbol }}
                            </div>
                        </div>
                        <div>
                            <h4 class="font-bold text-white text-lg" :class="card.textColor">{{ $t(`instructions.cards.special.${card.key}.name`) }}</h4>
                            <p class="text-sm mt-1">{{ $t(`instructions.cards.special.${card.key}.desc`) }}</p>
                            <p class="text-xs text-gray-400 mt-2 italic">{{ $t(`instructions.cards.special.${card.key}.note`) }}</p>
                        </div>
                    </div>
                </div>
            </section>
        </div>

        <!-- Gameplay Tab -->
        <div v-show="currentTab === 'gameplay'" class="space-y-8">
            <div class="relative pl-8 border-l-2 border-gray-600 space-y-8">
                <div class="relative">
                    <div class="absolute -left-[41px] bg-gray-800 border-2 border-blue-500 text-blue-500 rounded-full w-8 h-8 flex items-center justify-center font-bold">1</div>
                    <h3 class="text-xl font-bold text-white mb-2">{{ $t('instructions.gameplay.setup.title') }}</h3>
                    <p>{{ $t('instructions.gameplay.setup.desc') }}</p>
                </div>
                <div class="relative">
                    <div class="absolute -left-[41px] bg-gray-800 border-2 border-blue-500 text-blue-500 rounded-full w-8 h-8 flex items-center justify-center font-bold">2</div>
                    <h3 class="text-xl font-bold text-white mb-2">{{ $t('instructions.gameplay.mulligan.title') }}</h3>
                    <p>{{ $t('instructions.gameplay.mulligan.desc') }}</p>
                </div>
                <div class="relative">
                    <div class="absolute -left-[41px] bg-gray-800 border-2 border-blue-500 text-blue-500 rounded-full w-8 h-8 flex items-center justify-center font-bold">3</div>
                    <h3 class="text-xl font-bold text-white mb-2">{{ $t('instructions.gameplay.round.title') }}</h3>
                    <ul class="list-disc list-inside space-y-1 text-gray-400 ml-2 mt-2">
                        <li>{{ $t('instructions.gameplay.round.play') }}</li>
                        <li>{{ $t('instructions.gameplay.round.pass') }}</li>
                        <li>{{ $t('instructions.gameplay.round.resolve') }}</li>
                    </ul>
                </div>
                <div class="relative">
                    <div class="absolute -left-[41px] bg-gray-800 border-2 border-blue-500 text-blue-500 rounded-full w-8 h-8 flex items-center justify-center font-bold">4</div>
                    <h3 class="text-xl font-bold text-white mb-2">{{ $t('instructions.gameplay.winning.title') }}</h3>
                    <p class="text-yellow-400 font-bold">{{ $t('instructions.gameplay.winning.desc') }}</p>
                </div>
            </div>
        </div>

      </div>

      <!-- Footer -->
      <div class="p-6 border-t border-gray-700 bg-gray-900 flex justify-end">
        <button 
          @click="$emit('close')" 
          class="bg-gray-700 hover:bg-gray-600 text-white font-bold py-2 px-6 rounded transition-colors"
        >
          {{ $t('instructions.close') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';

defineEmits(['close']);
const { t } = useI18n();

const currentTab = ref('intro');

const tabs = [
  { id: 'intro' },
  { id: 'cards' },
  { id: 'gameplay' }
];

const coreMechanics = computed(() => [
    t('instructions.intro.core_mechanics.resource'),
    t('instructions.intro.core_mechanics.bluff'),
    t('instructions.intro.core_mechanics.combo')
]);

const specialCards = [
    { key: 'spy', symbol: 'J', borderColor: 'border-red-500', textColor: 'text-red-400' },
    { key: 'medic', symbol: 'Q', borderColor: 'border-green-500', textColor: 'text-green-400' },
    { key: 'hero', symbol: 'K', borderColor: 'border-yellow-500', textColor: 'text-yellow-400' },
    { key: 'scorch', symbol: 'A', borderColor: 'border-orange-500', textColor: 'text-orange-400' },
    { key: 'decoy', symbol: '★', borderColor: 'border-purple-500', textColor: 'text-purple-400' },
];
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 8px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(31, 41, 55, 0.5);
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(75, 85, 99, 0.8);
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(107, 114, 128, 0.8);
}
</style>
