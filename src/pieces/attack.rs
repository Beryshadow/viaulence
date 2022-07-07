use crate::{grid::isometric_grid::IGrid, Piece};
use std::io::Error;

pub fn attack(attacker: &Piece, attacked: &Piece, grid: &IGrid) -> Result<(), Error> {
    unimplemented!();
}

fn get_all_targets(attacker: &Piece, grid: &IGrid) -> Vec<Piece> {
    unimplemented!();
}
