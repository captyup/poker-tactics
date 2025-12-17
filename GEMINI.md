# Poker Tactics - GEMINI Project Context

## Project Overview

**Poker Tactics** is a web-based card game implementation of "Poker Gwent" (a simplified version of Gwent using standard poker cards). It focuses on strategy, resource management, and psychological play.

The project is a monorepo containing:
*   **Client**: A Vue 3 application (Vite + TypeScript + Pinia + Tailwind CSS).
*   **Server**: A Rust application (Axum + Socketioxide).

### Key Features
*   **Multi-language Support**: English, Traditional Chinese (繁體中文), Simplified Chinese (简体中文).
*   **Real-time Gameplay**: Socket.IO for state synchronization.
*   **Game Logic**:
    *   **Unit Cards**: Numbers 2-10 (IronGuard/Bond mechanic for 2s).
    *   **Special Cards**: Spy (J), Medic (Q), Hero (K), Scorch (A), Decoy (Joker).
    *   **Mechanics**: Mulligan, Round-based scoring (Best of 3), Passing.
*   **UI/UX**:
    *   **Card Sorting**: Hand is sorted by type (Special > Number) and power.
    *   **Interaction**: "Tap to Select, Tap again to Play" pattern with dynamic hints.
    *   **Visuals**: Tailwind CSS styling with card-specific visual indicators.

## Directory Structure

*   `game-rules.md`: The definitive ruleset for Poker Gwent.
*   `client/`: Frontend source code.
    *   `src/components/`: UI components (`GameBoard.vue`, `CardComponent.vue`, `LanguageSwitcher.vue`).
    *   `src/stores/`: Pinia state management (`game.ts`).
    *   `src/locales/`: i18n translation files (`en.json`, `zh-TW.json`, `zh-CN.json`).
    *   `src/types/`: TypeScript definitions shared with backend logic.
*   `server/`: Backend source code.
    *   `.gitignore`: Specifies files and directories to be ignored by Git (e.g., build artifacts).
    *   `src/main.rs`: Entry point, Socket.IO setup, event handlers, and static file serving.
    *   `src/game_logic.rs`: Core game mechanics (deck creation, turn resolution, ability logic).
    *   `src/game_types.rs`: Data structures (Card, Player, GameState).
*   `Dockerfile`: Multi-stage build configuration for creating a single deployable image.
*   `docker-compose.yml`: Deployment configuration for Raspberry Pi with Cloudflare Tunnel.

## Getting Started

### Prerequisites
*   Node.js (v20+)
*   Rust (latest stable)
*   Docker & Docker Compose

### Development

1.  **Start the Server:**
    ```bash
    cd server
    cargo run
    ```
    *   Runs on `http://0.0.0.0:3000`.

2.  **Start the Client:**
    ```bash
    cd client
    npm install
    npm run dev
    ```
    *   Runs on `http://localhost:5173`.

### Deployment (Raspberry Pi + Cloudflare Tunnel)

1.  **Prepare Environment:**
    Create a `.env` file in the root directory and add your Cloudflare Tunnel token:
    ```env
    TUNNEL_TOKEN=your_token_here
    ```

2.  **Deploy:**
    ```bash
    docker-compose up -d --build
    ```
    *   This builds the image (supporting ARM64 for RPi) and starts the game server along with the Cloudflare Tunnel agent.
    *   Access the game via your configured Cloudflare domain.

## Development Conventions

*   **Communication**: Socket.IO events (`join_game`, `game_state_update`, `play_card`, `mulligan`, `pass`).
*   **State Management**: The server is the source of truth. The client renders based on the broadcasted `GameState`.
*   **i18n**: Use `vue-i18n`. All user-facing text must be in `client/src/locales/`.
*   **Card Logic**: All card abilities are enforced by `server/src/game_logic.rs`. Client-side checks are visual only.

## Troubleshooting

*   **Connection Refused**: Ensure the server is running on port 3000.
*   **WebSocket Error**: Check CORS settings in `server/src/main.rs`. Currently set to `permissive`.
*   **Game State Not Updating**: Ensure the server logic awaits `socket.emit` futures (e.g., `socket.within(...).emit(...).await`).