// fn a player is a struct with a vec of pieces he owns, an amount of gold and an amount of coins
// a player will exist so long as the grid also exists

// might need to remake player

use crate::{
    game::gamestate::Game,
    grid::isometric_grid::IGrid,
    pieces::{
        attack::{in_range, in_range_with_dyn},
        movement::can_move,
        traits::{Attack, Move, Piece},
    },
};

#[derive(Debug)]
pub struct Player {
    uuid: uuid::Uuid,
    pieces: Vec<Box<dyn Piece>>,
    gold: i32,
    coins: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            uuid: uuid::Uuid::new_v4(),
            pieces: Vec::new(),
            gold: 0,
            coins: 1000,
        }
    }
    pub fn add_piece(&mut self, piece: Box<dyn Piece>) {
        self.pieces.push(piece);
    }
    pub fn pieces(&self) -> &Vec<Box<dyn Piece>> {
        &self.pieces
    }
    pub fn can_play(&self, game: &Game) -> bool {
        // a player can play if he can buy a piece, move a piece, or attack a piece
        self.can_buy(game)
            || self.can_move(game.get_grid_ref())
            || self.can_attack(game.get_grid_ref())
    }
    pub fn can_buy(&self, game: &Game) -> bool {
        // a player can buy if he has enough coins to buy any piece
        // get the lowest costing piece and check if he has enough coins to buy it
        if game.lowest_costing_piece().is_none() {
            return false;
        } else if game.lowest_costing_piece().unwrap().get_cost() > self.coins {
            return false;
        } else {
            return true;
        }
    }
    pub fn can_move(&self, grid: &IGrid) -> bool {
        // a player can move if he has a piece that can move
        // the can move method is in the movement trait
        for piece in self.pieces.iter() {
            if piece.movable() {
                // here we know we have a piece that can move
                // we need to check if it has access to one of the three spaces around it
                if can_move(piece, grid) {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn can_attack(&self, grid: &IGrid) -> bool {
        // a player can attack if he has a piece that can attack
        // the can attack method is in the movement trait
        for piece in self.pieces.iter() {
            if piece.can_attack() {
                let attack_piece = piece.to_attack().unwrap();
                if in_range_with_dyn(attack_piece, grid).len() > 0 {
                    return true;
                }
            }
        }
        return false;
    }
}
