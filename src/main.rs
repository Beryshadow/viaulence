// use crate::pieces::movement::populate_tree;
// use crate::pieces::tokens::Piece;
// use crate::pieces::tokens::*;
use grid::isometric_grid::{Coord, IGrid};
use pieces::traits::Consumable;

use crate::{
    game::gamestate::start_new_game,
    pieces::{movement::*, tokens::*},
};

mod game;
mod grid;
mod pieces;
mod player;
#[cfg(test)]
mod tests;

fn main() {
    // create a set of pieces
    let mut pieces: Vec<Box<(dyn Consumable + 'static)>> = Vec::new();
    let mut scout1 = Scout::new();

    // create a new game
    let mut game = start_new_game(2, pieces);
    println!()
}
