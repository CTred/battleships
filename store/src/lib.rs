pub mod camera;
pub mod game_objects;
pub mod map;

pub use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use game_objects::{GameObject, SHIPS};
use map::components::CubeCoords;

/// Struct for storing player related data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
}

#[derive(Resource)]
pub struct WhoAmI(pub PlayerId);

/// An event that progresses the GameState forward
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum GameEvent {
    SetupBoard,
    BeginGame {
        first_player: PlayerId,
    },
    EndGame {
        reason: EndGameReason,
    },
    PlayerJoined {
        player_id: PlayerId,
        player_details: Player,
    },
    PlayerDisconnected {
        player_id: PlayerId,
    },
    // PlayerSelects {
    //     player_id: PlayerId,
    //     select_box: SelectQuad,
    // },
    ShipMove {
        player_id: PlayerId,
        at: CubeCoords,
    },
    ShipPlaced {
        player_id: PlayerId,
        ship_type: GameObject,
        at: CubeCoords,
        rotation: i32,
    },
}

/// The different states a game can be in. (not to be confused with the entire "GameState")
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum GameStage {
    Lobby,
    PreGame,
    InGame,
    Paused,
    Ended,
}

/// This just makes it easier to dissern between a player id and any ol' u64
type PlayerId = u64;

/// A GameState object that is able to keep track of a game of TicTacTussle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Resource)]
pub struct GameState {
    pub stage: GameStage,
    pub players: HashMap<PlayerId, Player>,
    pub player_ships: HashMap<PlayerId, Vec<(GameObject, CubeCoords, i32)>>,
    pub history: Vec<GameEvent>,
    pub cur_player: Option<PlayerId>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stage: GameStage::Lobby,
            players: HashMap::new(),
            player_ships: HashMap::new(),
            history: Vec::new(),
            cur_player: None,
        }
    }
}

impl GameState {
    /// Determines whether an event is valid considering the current GameState
    pub fn validade(&self, event: &GameEvent) -> bool {
        use GameEvent::*;
        match event {
            BeginGame { first_player } => {
                if None == self.players.get(first_player) {
                    return false;
                }
                if self.players.len() != 2 {
                    return false;
                }
                if self
                    .player_ships
                    .iter()
                    .any(|(_, vec)| vec.len() < SHIPS.len())
                {
                    return false;
                }
            }
            EndGame { reason } => match reason {
                EndGameReason::PlayerWon { winner: _ } => {
                    if self.stage != GameStage::InGame {
                        return false;
                    }
                }
                _ => {}
            },
            PlayerJoined {
                player_id,
                player_details: _,
            } => {
                if self.players.contains_key(player_id) {
                    return false;
                }
            }
            PlayerDisconnected { player_id } => {
                if !self.players.contains_key(player_id) {
                    return false;
                }
            }
            ShipMove { player_id, at: _ } => return self.is_player_turn(player_id),
            ShipPlaced { player_id, .. } => {
                // check if game is in PreGame
                if self.stage != GameStage::PreGame {
                    return false;
                }

                // check if player is still allowed to place ships
                match self.player_ships.get(player_id) {
                    Some(garage) => {
                        if garage.len() == SHIPS.len() {
                            return false;
                        }
                        if garage.len() > SHIPS.len() {
                            panic!("{:?} has placed more ships than allowed", player_id);
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
            SetupBoard => {
                if self.stage != GameStage::Lobby {
                    return false;
                }
                if self.players.len() != 2 {
                    return false;
                }
            }
        }
        true
    }

    pub fn consume(&mut self, valid_event: &GameEvent) {
        use GameEvent::*;
        match valid_event {
            BeginGame { first_player } => {
                let player = self
                    .players
                    .iter()
                    .filter(|(p, _)| *p != first_player)
                    .next()
                    .unwrap();
                self.cur_player = Some(*player.0);
                trace!("First player: {:?}", *player.0);
                self.stage = GameStage::InGame;
            }
            EndGame { reason: _ } => self.stage = GameStage::Ended,
            PlayerDisconnected { player_id } => {
                self.players.remove(player_id);
            }
            PlayerJoined {
                player_id,
                player_details,
            } => {
                self.players.insert(*player_id, player_details.clone());
            }
            ShipMove {
                player_id: _,
                at: _,
            } => {
                self.cur_player = self.next_player();
            }
            ShipPlaced {
                player_id,
                at,
                rotation,
                ship_type,
            } => {
                let ship_vec = self.player_ships.get_mut(&player_id).unwrap();
                ship_vec.push((*ship_type, *at, *rotation));
            }
            SetupBoard => {
                self.stage = GameStage::PreGame;
                for p in &self.players {
                    self.player_ships.insert(*p.0, Vec::new());
                }
            }
        }

        self.history.push(valid_event.clone());
    }

    fn next_player(&self) -> Option<PlayerId> {
        if let Some(player_moved) = self.cur_player {
            for (key, _) in self.players.iter() {
                if player_moved != *key {
                    return Some(*key);
                }
            }
        }
        None
    }

    fn is_player_turn(&self, player_id: &PlayerId) -> bool {
        if let Some(p) = self.cur_player {
            if *player_id == p {
                return true;
            }
        }
        false
    }
}

/// The various reasons why a game could end
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Deserialize)]
pub enum EndGameReason {
    PlayerLeft { player_id: PlayerId },
    PlayerWon { winner: PlayerId },
}
