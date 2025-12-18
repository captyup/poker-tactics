export type Suit = 'Heart' | 'Diamond' | 'Spade' | 'Club' | 'Joker';

export type Rank = { Number: number } | 'Jack' | 'Queen' | 'King' | 'Ace' | 'Joker';

export type CardAbility = 'None' | 'IronGuard' | 'Intel' | 'Medic' | 'Hero' | 'Burn' | 'Decoy';

export type GamePhase = 'Waiting' | 'Mulligan' | 'Playing' | 'RoundEnd' | 'GameEnd';

export interface Card {
    id: string;
    suit: Suit;
    rank: Rank;
    base_power: number;
    current_power: number;
    ability: CardAbility;
    owner_id: string;
}

export interface Player {
    id: string;
    nickname: string;
    avatar: string;
    hand: Card[];
    board: Card[];
    discard_pile: Card[];
    current_score: number;
    rounds_won: number;
    passed: boolean;
}

export interface GameState {
    room_id: string;
    phase: GamePhase;
    players: Record<string, Player>;
    current_turn: string;
    round_count: number;
    deck: Card[]; 
    winner?: string | null;
}
