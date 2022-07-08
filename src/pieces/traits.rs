use std::io::Error;

use crate::grid::isometric_grid::{Coord, IGrid};

use super::tokens::Piece;

pub trait Attack {
    // can attack if it has range, has a damage value, and is not on a base,
    fn attack(&self, attacked: &Piece, grid: &IGrid) -> Result<(), Error>;
}

pub trait CanBeAttacked {
    // it has a health attribute and is not currently immune to attacks
    fn can_be_attacked(&self, attacker: &Piece) -> bool;
}

pub trait Move {
    // we take a piece and a grid and return a Ok(()) if the move was successful or Err(()) if it was not
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error>;
    fn get_moves(&self) -> Option<i8> {
        Some(0)
    }
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

pub trait Placeable {
    fn placeable(&self, grid: &IGrid) -> bool;
}
