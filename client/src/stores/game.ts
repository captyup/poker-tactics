import { defineStore } from 'pinia';
import { io, type Socket } from 'socket.io-client';
import type { GameState } from '@/types/poker';
import { ref } from 'vue';
import { useUserStore } from './user';
import { soundManager } from '@/utils/sound';

export const useGameStore = defineStore('game', () => {
    const socket = ref<Socket | null>(null);
    const gameState = ref<GameState | null>(null);
    const availableRooms = ref<any[]>([]); // Using any[] for now, or define RoomInfo interface
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

            // Simple sound triggers based on state changes (naive approach)
            // Ideally we'd compare old state vs new state to detect events
            if (gameState.value) {
                // If it was my turn and now it's not (played a card)
                if (gameState.value.current_turn === playerId.value && state.current_turn !== playerId.value) {
                     soundManager.play('play');
                }

                // If round count changed
                if (gameState.value.round_count !== state.round_count) {
                    soundManager.play('round_end');
                }

                // If winner
                if (!gameState.value.winner && state.winner) {
                    soundManager.play(state.winner === playerId.value ? 'win' : 'lose');
                }
            }

            gameState.value = state;
            error.value = '';
        });

        socket.value.on('rooms_list', (rooms: any[]) => {
            availableRooms.value = rooms;
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

        const userStore = useUserStore();

        socket.value?.emit('join_game', {
            room_id: room,
            player_id: player,
            nickname: userStore.nickname,
            avatar: userStore.avatar
        });
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

    function restartGame() {
        socket.value?.emit('restart_game', {
            room_id: roomId.value,
            player_id: playerId.value
        });
    }

    function fetchRooms() {
        connect();
        socket.value?.emit('list_rooms');
    }

    return {
        socket,
        gameState,
        availableRooms,
        playerId,
        roomId,
        error,
        joinGame,
        mulligan,
        playCard,
        passTurn,
        restartGame,
        fetchRooms
    };
});