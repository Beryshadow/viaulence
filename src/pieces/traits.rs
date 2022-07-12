use std::io::Error;

use uuid::Uuid;

use crate::grid::isometric_grid::{Coord, IGrid};

// use super::tokens::Piece;

pub trait Piece {
    // can occupy a slot has coords,
}

pub trait Attack {
    // can attack if it has range, has a damage value, and is not on a base,
    fn attack(&self, attacked: &mut dyn Attackable, grid: &IGrid) -> Result<(), Error>;
    fn can_attack(&self, grid: &IGrid) -> bool;
    fn get_range(&self) -> i8;
    fn get_damage(&self) -> i8;
    fn get_coord(&self) -> &Coord;

    // WWOT
    // This is important we need to take the piece and look
    // in its range for any pieces that are not of the same team
}

pub trait Attackable {
    // it has a health attribute and is not currently immune to attacks
    fn can_be_attacked(&self, attacker: &dyn Piece) -> bool;
    fn get_uuid(&self) -> Uuid;
}

pub trait Move {
    // we take a piece and a grid and return a Ok(()) if the move was successful or Err(()) if it was not
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        // we get the current coord of the piece self (maybe we can have coords in the piece?)
        // using the tree to put the piece in the new coord
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(0)
    }
    fn get_coord(&self) -> &Coord;
    fn not_blocked(&self, grid: &IGrid) -> bool;
}

pub trait Die {
    // we take a piece and a grid and then remove it from the grid and return a Ok(()) if the piece was successfully removed or Err(()) if it was not
    fn die(&self) -> Result<(), Error>;
}

pub trait CarryGold {
    fn carry_gold(&self) -> i32;
}

pub trait Heal {
    fn heal(&self) -> i8;
}

pub trait Buyable {
    fn cost(&self) -> i32;
}
