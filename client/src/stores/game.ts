import { defineStore } from 'pinia';
import { io, type Socket } from 'socket.io-client';
import type { GameState } from '@/types/poker';
import { ref } from 'vue';

export const useGameStore = defineStore('game', () => {
    const socket = ref<Socket | null>(null);
    const gameState = ref<GameState | null>(null);
    const playerId = ref('');
    const roomId = ref('');
    const error = ref('');

    function connect() {
        if (socket.value) return;
        
        const socketUrl = import.meta.env.VITE_APP_SOCKET_URL || 'http://localhost:3000';
        socket.value = io(socketUrl);
        
        socket.value.on('connect', () => {
            console.log('Connected to', socketUrl);
        });

        socket.value.on('game_state_update', (state: GameState) => {
            console.log('Game State Updated:', state);
            gameState.value = state;
            error.value = '';
        });

        socket.value.on('error', (msg: string) => {
            console.error('Game Error:', msg);
            error.value = msg;
        });
    }

    function joinGame(room: string, player: string) {
        connect();
        roomId.value = room;
        playerId.value = player;
        // Delay emit slightly to ensure connection? No, socket.io buffers.
        socket.value?.emit('join_game', { room_id: room, player_id: player });
    }

    function mulligan(cardIds: string[]) {
        socket.value?.emit('mulligan', { 
            room_id: roomId.value, 
            player_id: playerId.value,
            card_ids: cardIds 
        });
    }

    function playCard(cardId: string, targetId?: string) {
        socket.value?.emit('play_card', { 
            room_id: roomId.value, 
            player_id: playerId.value,
            card_id: cardId,
            target_id: targetId
        });
    }

    function passTurn() {
         socket.value?.emit('pass', { 
            room_id: roomId.value, 
            player_id: playerId.value
        });
    }

    return {
        socket,
        gameState,
        playerId,
        roomId,
        error,
        joinGame,
        mulligan,
        playCard,
        passTurn
    };
});