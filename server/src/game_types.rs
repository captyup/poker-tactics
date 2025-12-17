use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
    Joker,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardAbility {
    None,      // Normal cards
    IronGuard, // For Rank 2: pairs get stronger
    Intel,     // For Jack: spy logic
    Medic,     // For Queen: revive unit
    Hero,      // For King: immune to effects
    Burn,      // For Ace: scorch strongest unit
    Decoy,     // For Joker: return card to hand
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    Waiting,
    Mulligan,
    Playing,
    RoundEnd,
    GameEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub suit: Suit,
    pub rank: Rank,
    pub base_power: u8,
    pub current_power: u8, // Can change (e.g., IronGuard)
    pub ability: CardAbility,
    pub owner_id: String, // The player who "controls" this card on the board (for scoring)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub hand: Vec<Card>,
    pub board: Vec<Card>,         // Cards currently on this player's side of the field
    pub discard_pile: Vec<Card>,  // Cards in the graveyard
    pub current_score: u32,       // Current round score
    pub rounds_won: u8,
    pub passed: bool,             // If true, player cannot play more cards this round
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub room_id: String,
    pub phase: GamePhase,
    pub players: HashMap<String, Player>, // Key: Player ID
    pub current_turn: String,             // Player ID
    pub round_count: u32,
    pub deck: Vec<Card>,
    pub winner: Option<String>,           // Player ID of the match winner
    pub last_update: u64, // Timestamp of last update
}
