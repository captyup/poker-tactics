import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useUserStore = defineStore('user', () => {
    const nickname = ref(localStorage.getItem('poker_tactics_nickname') || '');
    const avatar = ref(localStorage.getItem('poker_tactics_avatar') || '👤');

    function setProfile(name: string, av: string) {
        nickname.value = name;
        avatar.value = av;
        localStorage.setItem('poker_tactics_nickname', name);
        localStorage.setItem('poker_tactics_avatar', av);
    }

    return {
        nickname,
        avatar,
        setProfile
    };
});
