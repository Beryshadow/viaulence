use crate::{grid::isometric_grid::IGrid, Piece};
use std::io::Error;

use super::traits::{Attack, CanBeAttacked};

pub fn attack<A, T>(attacker: &A, attacked: &T, grid: &IGrid) -> Result<(), Error>
where
    A: Attack,
    T: CanBeAttacked,
{
    unimplemented!();
}

fn get_all_targets<A>(attacker: &A, grid: &IGrid) -> Vec<Piece>
where
    A: Attack,
{
    unimplemented!();
}
