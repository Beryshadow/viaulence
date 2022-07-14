use std::io::Error;

use std::fmt::Debug;
use uuid::Uuid;

use crate::grid::isometric_grid::{Coord, IGrid};

// use super::tokens::Piece;

pub trait BuyablePiece: Piece + Buyable {
    fn get_cost(&self) -> i32;
    fn get_id(&self) -> Uuid;
    fn set_id(&mut self, id: Uuid);
}

impl Debug for dyn BuyablePiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BuyablePiece {{ id: {}, cost: {} }}",
            self.get_id(),
            self.get_cost()
        )
    }
}

pub trait Piece {
    //: Clone + std::fmt::Debug
    // can occupy a slot has coords,
    fn get_coord(&self) -> &Coord;
    fn get_uuid(&self) -> &Uuid;
    fn change_immune_state(&mut self, immune: bool);
    fn is_immune(&self) -> bool;
    fn get_on_base(&self) -> bool;
    fn set_on_base(&mut self, on_base: bool);
    fn get_on_pot(&self) -> bool;
    fn set_on_pot(&mut self, on_pot: bool);
    fn can_host_piece(&self) -> bool;
    fn get_name(&self) -> &str;
    fn movable(&self) -> bool;
    fn can_attack(&self) -> bool;
    fn can_be_attacked(&self) -> bool;
}

impl Debug for dyn Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Piece {{ coord: {:#?} }}", self.get_coord())
    }
}

impl PartialEq for dyn Piece {
    fn eq(&self, other: &Self) -> bool {
        self.get_uuid() == other.get_uuid()
    }
}

pub trait Attack: Piece {
    // can attack if it has range, has a damage value, and is not on a base,
    fn attack(&self, attacked: &mut dyn Attackable, grid: &IGrid) -> Result<(), Error>;
    fn can_attack(&self, grid: &IGrid) -> bool;
    fn get_range(&self) -> i8;
    fn get_damage(&self) -> i8;

    // WWOT
    // This is important we need to take the piece and look
    // in its range for any pieces that are not of the same team
}

pub trait Attackable {
    // it has a health attribute and is not currently immune to attacks
    fn can_be_attacked(&self) -> bool;
    fn get_uuid(&self) -> Uuid;
}

pub trait Move: Piece {
    // we take a piece and a grid and return a Ok(()) if the move was successful or Err(()) if it was not
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        // we get the current coord of the piece self (maybe we can have coords in the piece?)
        // using the tree to put the piece in the new coord
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(0)
    }
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
