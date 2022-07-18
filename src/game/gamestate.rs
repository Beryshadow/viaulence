// lets create a new game we might have to remake game state entirely

use std::fmt::Debug;
use std::io::Error;

use crate::grid::isometric_grid::IGrid;
use crate::pieces;
use crate::pieces::traits::{Consumable, Piece};
use crate::player::player::Player;

pub fn start_new_game(player_count: i32, pieces_available: Vec<Box<dyn Consumable>>) -> Game {
    // create a new game
    let mut game = Game::from(player_count, pieces_available);

    // make the turn system

    // while game.get_gold() > 0 {
    //     game.turn();
    // }
    game
}

// a game is a struct that has a grid, a vec of players and a current player pointer
pub struct Game {
    grid: IGrid,
    players: Vec<Player>,
    player_count: i32,
    turn: usize,
    gold_in_game: i32,
    available_pieces: Vec<Box<dyn Consumable>>,
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {{ grid: {:#?}, players: {:#?}, player_count: {}, turn: {}, gold_in_game: {}, available_pieces: {:#?} }}", self.grid, self.players, self.player_count, self.turn, self.gold_in_game, self.available_pieces)
    }
}

impl<'a> Game {
    fn new() -> Game {
        Game {
            grid: IGrid::new(),
            players: Vec::new(),
            player_count: 0,
            turn: 0,
            gold_in_game: 0,
            available_pieces: Vec::new(),
        }
    }
    fn from(players: i32, pieces_available: Vec<Box<dyn Consumable>>) -> Game {
        let mut player_vec = Vec::new();
        for _ in 0..players {
            let player = Player::new();
            // add the player to the game
            player_vec.push(player);
        }
        Game {
            grid: IGrid::new(),
            players: player_vec,
            player_count: players,
            turn: 0,
            gold_in_game: players * 20 + 1,
            available_pieces: pieces_available,
        }
    }
    fn add_player(&mut self, player: Player) {
        self.player_count += 1;
        self.players.push(player);
    }
    fn add_gold(&mut self, gold: i32) {
        self.gold_in_game += gold;
    }
    fn get_gold(&self) -> i32 {
        self.gold_in_game
    }
    pub fn turn(&mut self) -> Result<(), Error> {
        // get the current player
        let current_player = self.players.get(self.turn).unwrap();
        // check if the player can place a piece with his available coins,
        if current_player.can_play(&self) {
            // if he can, place a piece
            if current_player.can_buy(&self) {
                println!("player can buy");
            }
            if current_player.can_move(&self.grid) {
                println!("player can move");
            }
            if current_player.can_attack(&self.grid) {
                println!("player can attack");
            }
            println!("player can play");
        }
        Ok(())
    }
    fn do_action_phase(&mut self) {
        for _ in 0..=3 {
            if let Err(_) = self.turn() {
                // remove the player from the game
            }
        }
        self.turn += 1;
    }
    pub fn get_grid_ref(&self) -> &IGrid {
        &self.grid
    }
    pub fn lowest_costing_piece(&self) -> Option<&Box<dyn Consumable>> {
        let mut lowest_costing_piece = None;
        for piece in self.available_pieces.iter() {
            if lowest_costing_piece.is_none() {
                lowest_costing_piece = Some(piece);
            } else if lowest_costing_piece.unwrap().get_cost() > piece.get_cost() {
                lowest_costing_piece = Some(piece);
            }
        }
        lowest_costing_piece
    }
    fn add_to_available_pieces(&mut self, piece: Box<dyn Consumable>) {
        //convert the piece to a buyable piece
        self.available_pieces.push(piece);
    }
    pub fn remove_piece(&mut self, piece: Box<dyn Consumable>) {
        self.grid.remove_piece(piece.get_coord());
        self.add_to_available_pieces(piece);
    }
}
