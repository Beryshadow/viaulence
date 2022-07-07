use std::io::Error;

use crate::grid::isometric_grid::{Coord, IGrid};

use super::tokens::Piece;

pub trait Attack {
    fn attack(&self, attacked: &Piece, grid: &IGrid) -> Result<(), Error>;
}

pub trait Move {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error>;
}

pub trait Die {
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
