// fn a player is a struct with a vec of pieces he owns, an amount of gold and an amount of coins
// a player will exist so long as the grid also exists

// might need to remake player

use crate::{
    game::gamestate::Game,
    grid::isometric_grid::IGrid,
    pieces:: traits::Piece,
};

#[derive(Debug)]
pub struct Player<'a> {
    uuid: uuid::Uuid,
    pieces: Vec<'a Box<& dyn Piece>>,
    gold: i32,
    coins: i32,
}

impl Player<'_> {
    pub fn new() -> Player<'static> {
        Player {
            uuid: uuid::Uuid::new_v4(),
            pieces: Vec::new(),
            gold: 0,
            coins: 1000,
        }
    }
    pub fn add_piece<T>(&mut self, piece: &dyn Piece)
    where
        T: Piece,
    {
        self.pieces.push(Box::from(piece));
        // self.pieces.push(piece);
    }
    pub fn pieces(&self) -> &Vec<Box<dyn Piece>> {
        &self.pieces
    }
    pub fn can_play(&self, game: Game) -> bool {
        // a player can play if he can buy a piece, move a piece, or attack a piece
        self.can_buy(&game)
            || self.can_move(game.get_grid_ref())
            || self.can_attack(game.get_grid_ref())
    }
    pub fn can_buy(&self, game: &Game) -> bool {
        // a player can buy if he has enough coins to buy any piece
        // get the lowest costing piece and check if he has enough coins to buy it
        if game.lowest_costing_piece().is_none() {
            return false;
        } else if game.lowest_costing_piece().unwrap().get_cost() > Some(self.coins) {
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
                if piece.can_move(grid) {
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
            if piece.can_attack(grid) {
                //LII we need to propagate through all pieces and check if they can attack
                // it would be better to optimize this by checking the longest range pieces first
            }
        }
        return false;
    }
}
