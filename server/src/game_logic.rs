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
            nickname: "".to_string(), // Will be populated by main.rs on join/rejoin
            avatar: "".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_types::{Rank, Suit};

    // 輔助函式：快速建立一個測試用的 GameState
    fn setup_test_game() -> (GameState, String, String) {
        let p1_id = "player1".to_string();
        let p2_id = "player2".to_string();
        let game = init_game("room_test".to_string(), vec![p1_id.clone(), p2_id.clone()]);
        
        // 強制設定當前回合為 player1，方便測試
        let mut game = game;
        game.phase = GamePhase::Playing; // 跳過 Mulligan 直接開始
        game.current_turn = p1_id.clone();
        
        (game, p1_id, p2_id)
    }

    // 輔助函式：在玩家手牌中加入一張特定卡片方便測試
    fn add_card_to_hand(game: &mut GameState, player_id: &String, rank: Rank, ability: CardAbility, power: u8) -> String {
        let card = Card {
            id: Uuid::new_v4().to_string(),
            suit: Suit::Heart, // 花色不影響邏輯
            rank,
            base_power: power,
            current_power: power,
            ability,
            owner_id: player_id.clone(),
        };
        let card_id = card.id.clone();
        game.players.get_mut(player_id).unwrap().hand.push(card);
        card_id
    }

    #[test]
    fn test_create_deck_count() {
        let deck = create_deck();
        // 4 花色 * 13 張 (2-10, J, Q, K, A) + 2 張鬼牌 = 54 張
        assert_eq!(deck.len(), 54);
    }

    #[test]
    fn test_iron_guard_bonding() {
        let (mut game, p1, _) = setup_test_game();
        
        // 1. 給玩家兩張 2 (IronGuard)
        let c1_id = add_card_to_hand(&mut game, &p1, Rank::Number(2), CardAbility::IronGuard, 2);
        let c2_id = add_card_to_hand(&mut game, &p1, Rank::Number(2), CardAbility::IronGuard, 2);

        // 2. 打出第一張 2
        play_card(&mut game, &p1, &c1_id, None).expect("Should play card 1");
        
        // 檢查：只有一張時，戰力應為 2
        let p1_board = &game.players.get(&p1).unwrap().board;
        assert_eq!(p1_board[0].current_power, 2);

        // --- 修正重點開始 ---
        // 因為 play_card 會自動換回合，我們為了測試連續出牌效果，強制把回合切回給 p1
        game.current_turn = p1.clone();
        // --- 修正重點結束 ---

        // 3. 打出第二張 2
        play_card(&mut game, &p1, &c2_id, None).expect("Should play card 2");

        // 檢查：兩張都在場上時，戰力都應變為 6 (Bonding)
        let p1_board = &game.players.get(&p1).unwrap().board;
        assert_eq!(p1_board[0].current_power, 6);
        assert_eq!(p1_board[1].current_power, 6);
        
        // 檢查總分：6 + 6 = 12
        assert_eq!(game.players.get(&p1).unwrap().current_score, 12);
    }

    #[test]
    fn test_spy_mechanic() {
        let (mut game, p1, p2) = setup_test_game();

        // 1. 給 P1 一張間諜牌 (Rank J, Power 10)
        let spy_id = add_card_to_hand(&mut game, &p1, Rank::Jack, CardAbility::Intel, 10);
        
        // 記錄出牌前的手牌數量
        let initial_hand_size = game.players.get(&p1).unwrap().hand.len();

        // 2. P1 打出間諜
        play_card(&mut game, &p1, &spy_id, None).expect("Should play spy");

        // 檢查 A：間諜應該出現在 P2 (對手) 的場上
        let p2_board = &game.players.get(&p2).unwrap().board;
        assert!(p2_board.iter().any(|c| c.id == spy_id));

        // 檢查 B：P1 應該抽 2 張牌 (手牌數：原本 - 1 張打出 + 2 張抽牌 = +1)
        let final_hand_size = game.players.get(&p1).unwrap().hand.len();
        assert_eq!(final_hand_size, initial_hand_size + 1);
    }

    #[test]
    fn test_scorch_destroys_strongest() {
        let (mut game, p1, p2) = setup_test_game();

        // 1. 佈局：讓 P1 場上有一張 10，P2 場上有一張 10 和一張 5
        let c1 = Card { id: "p1_10".to_string(), suit: Suit::Spade, rank: Rank::Number(10), base_power: 10, current_power: 10, ability: CardAbility::None, owner_id: p1.clone() };
        game.players.get_mut(&p1).unwrap().board.push(c1);

        let c2 = Card { id: "p2_10".to_string(), suit: Suit::Heart, rank: Rank::Number(10), base_power: 10, current_power: 10, ability: CardAbility::None, owner_id: p2.clone() };
        let c3 = Card { id: "p2_5".to_string(), suit: Suit::Heart, rank: Rank::Number(5), base_power: 5, current_power: 5, ability: CardAbility::None, owner_id: p2.clone() };
        game.players.get_mut(&p2).unwrap().board.push(c2);
        game.players.get_mut(&p2).unwrap().board.push(c3);

        // 2. 給 P1 一張灼燒 (Scorch/Ace) 並打出
        let scorch_id = add_card_to_hand(&mut game, &p1, Rank::Ace, CardAbility::Burn, 0);
        play_card(&mut game, &p1, &scorch_id, None).expect("Should play scorch");

        // 3. 檢查：所有戰力為 10 的非英雄單位都應該被銷毀
        let p1_board = &game.players.get(&p1).unwrap().board;
        let p2_board = &game.players.get(&p2).unwrap().board;

        // P1 的 10 應該消失，只剩剛剛打出的 Scorch (0分)
        assert!(!p1_board.iter().any(|c| c.id == "p1_10"));
        
        // P2 的 10 應該消失，只剩 5
        assert!(!p2_board.iter().any(|c| c.id == "p2_10"));
        assert!(p2_board.iter().any(|c| c.id == "p2_5"));
    }

    #[test]
    fn test_scorch_does_not_destroy_hero() {
        let (mut game, p1, _) = setup_test_game();

        // 1. P1 場上有一張 15 分的英雄
        let hero = Card { id: "hero".to_string(), suit: Suit::Club, rank: Rank::King, base_power: 15, current_power: 15, ability: CardAbility::Hero, owner_id: p1.clone() };
        game.players.get_mut(&p1).unwrap().board.push(hero);

        // 2. 打出灼燒
        let scorch_id = add_card_to_hand(&mut game, &p1, Rank::Ace, CardAbility::Burn, 0);
        play_card(&mut game, &p1, &scorch_id, None).expect("Should play scorch");

        // 3. 檢查：英雄應該還在場上 (因為 Hero Immune)
        let p1_board = &game.players.get(&p1).unwrap().board;
        assert!(p1_board.iter().any(|c| c.id == "hero"));
    }

    #[test]
    fn test_medic_revive() {
        let (mut game, p1, _) = setup_test_game();

        // 1. 預先放一張牌在棄牌堆
        let dead_card = Card { id: "dead".to_string(), suit: Suit::Club, rank: Rank::Number(5), base_power: 5, current_power: 5, ability: CardAbility::None, owner_id: p1.clone() };
        game.players.get_mut(&p1).unwrap().discard_pile.push(dead_card);

        // 2. 給 P1 醫生 (Medic/Queen)
        let medic_id = add_card_to_hand(&mut game, &p1, Rank::Queen, CardAbility::Medic, 5);

        // 3. 打出醫生，指定復活 "dead"
        play_card(&mut game, &p1, &medic_id, Some("dead".to_string())).expect("Should play medic");

        // 4. 檢查：棄牌堆應該空了，該卡應該回到場上
        let p1_state = game.players.get(&p1).unwrap();
        assert_eq!(p1_state.discard_pile.len(), 0);
        assert!(p1_state.board.iter().any(|c| c.id == "dead"));
    }
    
    #[test]
    fn test_decoy_mechanic() {
        let (mut game, p1, _) = setup_test_game();
        
        // 1. 場上先有一張牌
        let board_card_id = "target_card";
        let board_card = Card { 
            id: board_card_id.to_string(), 
            suit: Suit::Spade, rank: Rank::Number(8), base_power: 8, current_power: 8, ability: CardAbility::None, owner_id: p1.clone() 
        };
        game.players.get_mut(&p1).unwrap().board.push(board_card);
        
        // 2. 手牌加入 Decoy (Joker)
        let decoy_id = add_card_to_hand(&mut game, &p1, Rank::Joker, CardAbility::Decoy, 0);
        
        // 3. 打出 Decoy，指定回收 target_card
        play_card(&mut game, &p1, &decoy_id, Some(board_card_id.to_string())).expect("Should play decoy");
        
        let p1_state = game.players.get(&p1).unwrap();
        
        // 4. 檢查：
        // - target_card 應該回到手牌
        assert!(p1_state.hand.iter().any(|c| c.id == board_card_id));
        // - board 上應該沒有 target_card
        assert!(!p1_state.board.iter().any(|c| c.id == board_card_id));
        // - board 上應該有 Decoy
        assert!(p1_state.board.iter().any(|c| c.id == decoy_id));
    }
}