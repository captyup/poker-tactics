use crate::game_types::{Card, CardAbility, GamePhase, GameState, Player, Rank, Suit};
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom; 
use rand::rng;
use std::collections::HashMap;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    let suits = [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club];
    
    // Add standard cards
    for suit in &suits {
        // Numbers 2-10
        for num in 2..=10 {
            let rank = Rank::Number(num);
            let ability = if num == 2 {
                CardAbility::IronGuard
            } else {
                CardAbility::None
            };
            
            deck.push(Card {
                id: Uuid::new_v4().to_string(),
                suit: suit.clone(),
                rank,
                base_power: num,
                current_power: num,
                ability,
                owner_id: "deck".to_string(),
            });
        }

        // Face cards
        deck.push(Card {
            id: Uuid::new_v4().to_string(),
            suit: suit.clone(),
            rank: Rank::Jack,
            base_power: 10, 
            current_power: 10,
            ability: CardAbility::Intel,
            owner_id: "deck".to_string(),
        });

        deck.push(Card {
            id: Uuid::new_v4().to_string(),
            suit: suit.clone(),
            rank: Rank::Queen,
            base_power: 5,
            current_power: 5,
            ability: CardAbility::Medic,
            owner_id: "deck".to_string(),
        });

        deck.push(Card {
            id: Uuid::new_v4().to_string(),
            suit: suit.clone(),
            rank: Rank::King,
            base_power: 15,
            current_power: 15,
            ability: CardAbility::Hero,
            owner_id: "deck".to_string(),
        });

        deck.push(Card {
            id: Uuid::new_v4().to_string(),
            suit: suit.clone(),
            rank: Rank::Ace,
            base_power: 0,
            current_power: 0,
            ability: CardAbility::Burn,
            owner_id: "deck".to_string(),
        });
    }

    // Add 2 Jokers
    for _ in 0..2 {
        deck.push(Card {
            id: Uuid::new_v4().to_string(),
            suit: Suit::Joker,
            rank: Rank::Joker,
            base_power: 0,
            current_power: 0,
            ability: CardAbility::Decoy,
            owner_id: "deck".to_string(),
        });
    }

    deck
}

pub fn init_game(room_id: String, player_ids: Vec<String>) -> GameState {
    let mut deck = create_deck();
    let mut rng = rng();
    deck.shuffle(&mut rng);

    let mut players = HashMap::new();
    
    for id in &player_ids {
        let hand: Vec<Card> = deck.drain(0..10).map(|mut c| {
            c.owner_id = id.clone();
            c
        }).collect();

        players.insert(id.clone(), Player {
            id: id.clone(),
            hand,
            board: Vec::new(),
            discard_pile: Vec::new(),
            current_score: 0,
            rounds_won: 0,
            passed: false,
        });
    }

    let current_turn = player_ids.choose(&mut rng).unwrap().clone();

    GameState {
        room_id,
        phase: GamePhase::Mulligan,
        players,
        current_turn,
        round_count: 1,
        deck,
        winner: None,
        last_update: get_timestamp(),
    }
}

pub fn handle_mulligan(game: &mut GameState, player_id: &String, card_ids_to_replace: Vec<String>) -> Result<(), String> {
    if game.phase != GamePhase::Mulligan {
        return Err("Not in Mulligan phase".to_string());
    }

    let player = game.players.get_mut(player_id).ok_or("Player not found")?;

    if card_ids_to_replace.len() > 2 {
        return Err("Cannot replace more than 2 cards".to_string());
    }

    let (to_keep, mut to_replace): (Vec<Card>, Vec<Card>) = player.hand.drain(..)
        .partition(|c| !card_ids_to_replace.contains(&c.id));

    if to_replace.len() != card_ids_to_replace.len() {
         player.hand = to_keep;
         player.hand.extend(to_replace);
         return Err("Invalid card IDs for mulligan".to_string());
    }
    
    player.hand = to_keep;

    for mut card in to_replace {
        card.owner_id = "deck".to_string();
        game.deck.push(card);
    }

    for _ in 0..card_ids_to_replace.len() {
        if let Some(mut card) = game.deck.first().cloned() {
             game.deck.remove(0);
             card.owner_id = player_id.clone();
             player.hand.push(card);
        }
    }

    game.last_update = get_timestamp();
    Ok(())
}

pub fn play_card(
    game: &mut GameState, 
    player_id: &String, 
    card_id: &String, 
    target_id: Option<String>
) -> Result<(), String> {
    if game.phase != GamePhase::Playing {
        return Err("Not in Playing phase".to_string());
    }
    if &game.current_turn != player_id {
        return Err("Not your turn".to_string());
    }

    let mut card_to_play = {
        let player = game.players.get_mut(player_id).ok_or("Player not found")?;
        let card_idx = player.hand.iter().position(|c| &c.id == card_id).ok_or("Card not in hand")?;
        player.hand.remove(card_idx)
    };

    match card_to_play.ability {
        CardAbility::Intel => { 
            let opponent_id = get_opponent_id(game, player_id)?;
            card_to_play.owner_id = opponent_id.clone();
            
            let opponent = game.players.get_mut(&opponent_id).unwrap();
            opponent.board.push(card_to_play);
            
            let cards_to_draw = 2;
            for _ in 0..cards_to_draw {
                if !game.deck.is_empty() {
                    let mut drawn = game.deck.remove(0);
                    drawn.owner_id = player_id.clone();
                    game.players.get_mut(player_id).unwrap().hand.push(drawn);
                }
            }
        },
        CardAbility::Medic => { 
            card_to_play.owner_id = player_id.clone();
            game.players.get_mut(player_id).unwrap().board.push(card_to_play);

            if let Some(tid) = target_id {
                let player = game.players.get_mut(player_id).unwrap();
                if let Some(idx) = player.discard_pile.iter().position(|c| c.id == tid) {
                    let mut revived = player.discard_pile.remove(idx);
                    
                    if revived.ability == CardAbility::Hero {
                        player.discard_pile.push(revived);
                        return Err("Cannot revive Hero".to_string());
                    }

                    revived.owner_id = player_id.clone();
                    
                    if revived.ability == CardAbility::Intel {
                         let opponent_id = get_opponent_id_from_keys(game.players.keys(), player_id);
                         revived.owner_id = opponent_id.clone();
                         game.players.get_mut(&opponent_id).unwrap().board.push(revived);
                         let cards_to_draw = 2;
                         for _ in 0..cards_to_draw {
                            if !game.deck.is_empty() {
                                let mut drawn = game.deck.remove(0);
                                drawn.owner_id = player_id.clone();
                                game.players.get_mut(player_id).unwrap().hand.push(drawn);
                            }
                         }
                    } else if revived.ability == CardAbility::Burn {
                         game.players.get_mut(player_id).unwrap().board.push(revived);
                         trigger_scorch(game);
                    } else {
                         game.players.get_mut(player_id).unwrap().board.push(revived);
                    }
                }
            }
        },
        CardAbility::Decoy => {
            if let Some(tid) = target_id {
                 let player = game.players.get_mut(player_id).unwrap();
                 if let Some(idx) = player.board.iter().position(|c| c.id == tid) {
                     let mut target_card = player.board.remove(idx);
                     
                     if target_card.ability == CardAbility::Hero {
                         player.board.push(target_card);
                         player.hand.push(card_to_play);
                         return Err("Cannot Decoy Hero".to_string());
                     }

                     target_card.owner_id = player_id.clone();
                     player.hand.push(target_card);
                     
                     card_to_play.owner_id = player_id.clone();
                     player.board.push(card_to_play);
                 } else {
                     game.players.get_mut(player_id).unwrap().hand.push(card_to_play);
                     return Err("Decoy target not found".to_string());
                 }
            } else {
                 game.players.get_mut(player_id).unwrap().hand.push(card_to_play);
                 return Err("Decoy requires a target".to_string());
            }
        },
        CardAbility::Burn => {
            card_to_play.owner_id = player_id.clone();
            game.players.get_mut(player_id).unwrap().board.push(card_to_play);
            trigger_scorch(game);
        },
        _ => {
             card_to_play.owner_id = player_id.clone();
             game.players.get_mut(player_id).unwrap().board.push(card_to_play);
        }
    }

    update_scores(game);
    
    let opponent_id = get_opponent_id(game, player_id)?;
    let opponent_passed = game.players.get(&opponent_id).unwrap().passed;
    
    if !opponent_passed {
        game.current_turn = opponent_id;
    }

    game.last_update = get_timestamp();

    Ok(())
}

pub fn pass_turn(game: &mut GameState, player_id: &String) -> Result<(), String> {
    if game.phase != GamePhase::Playing {
        return Err("Not in Playing phase".to_string());
    }
    if &game.current_turn != player_id {
        return Err("Not your turn".to_string());
    }

    let player = game.players.get_mut(player_id).unwrap();
    player.passed = true;

    let opponent_id = get_opponent_id_from_keys(game.players.keys(), player_id);
    let opponent = game.players.get(&opponent_id).unwrap();

    if opponent.passed {
        resolve_round(game);
    } else {
        game.current_turn = opponent_id;
    }

    game.last_update = get_timestamp();

    Ok(())
}

fn trigger_scorch(game: &mut GameState) {
    let mut max_power = 0;

    for (_pid, player) in &game.players {
        for card in &player.board {
            if card.ability != CardAbility::Hero {
                if card.current_power > max_power {
                    max_power = card.current_power;
                }
            }
        }
    }

    let mut ids_to_remove = Vec::new();

    for (_pid, player) in &game.players {
        for card in &player.board {
            if card.ability != CardAbility::Hero && card.current_power == max_power {
                ids_to_remove.push(card.id.clone());
            }
        }
    }

    for pid in game.players.keys().cloned().collect::<Vec<_>>() {
         let player = game.players.get_mut(&pid).unwrap();
         let mut i = 0;
         while i < player.board.len() {
             if ids_to_remove.contains(&player.board[i].id) {
                 let card = player.board.remove(i);
                 player.discard_pile.push(card);
             } else {
                 i += 1;
             }
         }
    }
}

fn update_scores(game: &mut GameState) {
    for player in game.players.values_mut() {
        let iron_guard_count = player.board.iter()
            .filter(|c| c.ability == CardAbility::IronGuard)
            .count();

        for card in &mut player.board {
            if card.ability == CardAbility::IronGuard {
                if iron_guard_count >= 2 {
                    card.current_power = 6;
                } else {
                    card.current_power = 2;
                }
            }
        }
        player.current_score = player.board.iter().map(|c| c.current_power as u32).sum();
    }
}

fn resolve_round(game: &mut GameState) {
    let ids: Vec<String> = game.players.keys().cloned().collect();
    let p1_id = &ids[0];
    let p2_id = &ids[1];

    let p1_score = game.players.get(p1_id).unwrap().current_score;
    let p2_score = game.players.get(p2_id).unwrap().current_score;

    if p1_score > p2_score {
        game.players.get_mut(p1_id).unwrap().rounds_won += 1;
    } else if p2_score > p1_score {
        game.players.get_mut(p2_id).unwrap().rounds_won += 1;
    } else {
        game.players.get_mut(p1_id).unwrap().rounds_won += 1;
        game.players.get_mut(p2_id).unwrap().rounds_won += 1;
    }

    let p1_wins = game.players.get(p1_id).unwrap().rounds_won;
    let p2_wins = game.players.get(p2_id).unwrap().rounds_won;

    if p1_wins >= 2 && p2_wins >= 2 {
        game.winner = Some("Draw".to_string()); 
        game.phase = GamePhase::GameEnd;
        return;
    } else if p1_wins >= 2 {
        game.winner = Some(p1_id.clone());
        game.phase = GamePhase::GameEnd;
        return;
    } else if p2_wins >= 2 {
        game.winner = Some(p2_id.clone());
        game.phase = GamePhase::GameEnd;
        return;
    }

    for player in game.players.values_mut() {
        player.discard_pile.append(&mut player.board);
        player.current_score = 0;
        player.passed = false;
    }
    
    game.round_count += 1;
    game.phase = GamePhase::Playing;
    if p1_score > p2_score {
        game.current_turn = p1_id.clone();
    } else if p2_score > p1_score {
        game.current_turn = p2_id.clone();
    }
}

fn get_opponent_id(game: &GameState, player_id: &String) -> Result<String, String> {
    for id in game.players.keys() {
        if id != player_id {
            return Ok(id.clone());
        }
    }
    Err("No opponent found".to_string())
}

fn get_opponent_id_from_keys<'a, I>(keys: I, player_id: &String) -> String
where
    I: Iterator<Item = &'a String>,
{
    for id in keys {
        if id != player_id {
            return id.clone();
        }
    }
    player_id.clone()
}
