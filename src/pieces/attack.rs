use crate::grid::isometric_grid::{Coord, IGrid};
use std::io::Error;

// WWOT this is where we implement the search for targets and the attack logic

use super::{
    traits::{Attack, Attackable, Piece},
    tree::{populate_tree, ThreeProngedTree},
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
    // let mut tree = populate_attack_tree(attacker, grid, previous);
    unimplemented!();
}

//LII this is the tree building function for the search for targets but we could take the tree
// building system in movment and isolate it so it can be used with attack and with movement
// it would return a tree that we can analyse independently of the other modules

pub fn in_range<'a, T: 'static>(attacker: &'a T, grid: &'a IGrid) -> Vec<&'a Coord>
where
    T: Attack + Copy,
{
    // create a new tree with the attacker's coord as the root and the range as the depth
    let coords = attacker.get_coord();
    let mut previous = vec![*attacker.get_coord()];
    let tree = populate_attack_tree(attacker, grid, &mut previous);
    let in_range = tree.get_list_of_children();

    // to not have to clone the pieces we can reference the coords inside the grid

    println!("in range: {:?}", in_range);
    let mut grid_refs: Vec<Option<&Coord>> =
        in_range.iter().map(|&c| grid.get_coord_ref(c)).collect(); // FIXME this is broken we gotta return an option from get_coor_ref
                                                                   // then we remove the garbage
    grid_refs.retain(|&c| c.is_some());
    let mut grid_refs: Vec<&Coord> = grid_refs.iter().map(|&c| c.unwrap()).collect();
    grid_refs.retain(|&c| c != coords);
    grid_refs.retain(|c| grid.get_piece(c).is_some());

    grid_refs.retain(|c| grid.get_piece(c).unwrap().can_be_attacked());
    // we return the coords of the pieces that can be attacked
    grid_refs
}

fn populate_attack_tree<T: 'static>(
    attacker: &T,
    grid: &IGrid,
    previous: &mut Vec<Coord>,
) -> ThreeProngedTree
where
    T: Attack + Copy,
{
    let coord = attacker.get_coord();
    let depth = attacker.get_range();
    let attacker = Box::new(*attacker) as Box<dyn Piece>;
    let mut tree = populate_tree(&attacker, grid, 1);
    tree
}
