use crate::grid::isometric_grid::{Coord, IGrid};
use std::io::Error;

// WWOT this is where we implement the search for targets and the attack logic

use super::{
    traits::{Attack, Attackable, Piece},
    tree::ThreeProngedTree,
};

use crate::pieces::tree::Slot::Available;

pub fn attack<A, T>(attacker: &A, attacked: &T, grid: &IGrid) -> Result<(), Error>
where
    A: Attack,
    T: Attackable,
{
    // we get all the targets and in CanBeAttack we return self and check if coords match up with the available targets in get_all_targets
    unimplemented!();
}

fn get_all_targets<A>(attacker: &A, grid: &IGrid) -> Vec<Box<dyn Piece>>
where
    A: Attack,
{
    // here the attack trait would have the build tree function and wed use it to search all available targets and return them
    unimplemented!();
}

//LII this is the tree building function for the search for targets but we could take the tree
// building system in movment and isolate it so it can be used with attack and with movement
// it would return a tree that we can analyse independently of the other modules

// pub fn in_range<'a, T>(attacker: &'a T, grid: &'a IGrid) -> &'a Vec<Coord>
// where
//     T: Attack,
// {
//     // create a new tree with the attacker's coord as the root and the range as the depth
//     let mut previous = vec![*attacker.get_coord()];
//     let tree = populate_attack_tree(attacker, grid, &mut previous);
//     let mut in_range = tree.get_list_of_children();
//     // in_range.retain(|&c| c != *coords);
//     // in_range.retain(|c| grid.get_piece(c).is_some());
//     // in_range.retain(|c| grid.get_piece(c).unwrap().can_be_attacked(grid));
//     in_range
// }

fn populate_attack_tree<'a, T>(
    attacker: &T,
    grid: &'a IGrid,
    previous: &mut Vec<Coord>,
) -> ThreeProngedTree
where
    T: Attack,
{
    let coord = attacker.get_coord();
    let mut tree = ThreeProngedTree::from(Available(*coord));
    let depth = attacker.get_range();
    tree.populate_for_depth(grid, previous, depth as i32);
    tree.set_list_of_children(previous.clone());
    tree
}
