// Placeholder for SoundManager
// In a real implementation, this would load Audio objects.

class SoundManager {
    private sounds: Map<string, HTMLAudioElement> = new Map();
    private enabled: boolean = true;

    constructor() {
        // Preload sounds if we had URLs
        // this.load('play', '/sfx/play.mp3');
    }

    play(name: string) {
        if (!this.enabled) return;
        console.log(`[SoundManager] Playing: ${name}`);

        // Simulating sound for now since we don't have assets
        // In the future:
        // const audio = this.sounds.get(name);
        // if (audio) {
        //    audio.currentTime = 0;
        //    audio.play().catch(e => console.warn('Audio play failed', e));
        // }
    }

    toggle(on: boolean) {
        this.enabled = on;
    }
}

export const soundManager = new SoundManager();
