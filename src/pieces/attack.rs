use crate::{
    game::{self, gamestate::Game},
    grid::isometric_grid::{Coord, IGrid},
};
use std::io::{Error, ErrorKind};

// WWOT this is where we implement the search for targets and the attack logic

use super::{
    traits::{Attack, Consumable, Piece},
    tree::{populate_tree, ThreeProngedTree},
};

/// This function removes health from the target piece. Make sure to check if the target is available first with in_range()
pub fn attack<A: 'static, T: 'static>(
    attacker: &A,
    attacked: &mut T,
    game: &mut Game,
) -> Result<(), Error>
where
    A: Attack + Copy,
    T: Consumable + Copy,
{
    // check if the target is immune
    if attacked.is_immune() {
        return Err(Error::new(ErrorKind::InvalidInput, "Target is immune"));
    }
    // check if the target is in range
    if !in_range(attacker, game.get_grid_ref()).contains(&attacked.get_coord()) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Target is not in range",
        ));
    }
    // apply the damage
    let damage = attacker.get_damage();
    attacked.remove_health(damage);

    // if the target is dead, remove it from the game
    if attacked.get_health() <= 0 {
        let attacked = Box::new(*attacked) as Box<dyn Consumable>;
        game.remove_piece(attacked);
    }

    Ok(())
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
    let mut grid_refs: Vec<Option<&Coord>> =
        in_range.iter().map(|&c| grid.get_coord_ref(c)).collect();
    grid_refs.retain(|&c| c.is_some());
    let mut grid_refs: Vec<&Coord> = grid_refs.iter().map(|&c| c.unwrap()).collect();
    grid_refs.retain(|&c| c != coords);
    grid_refs.retain(|c| grid.get_piece(c).is_some());
    grid_refs.retain(|c| grid.get_piece(c).unwrap().can_be_attacked());
    // we only keep the ones without the same team as the attacker
    grid_refs.retain(|c| grid.get_piece(c).unwrap().get_team_uuid().is_some());
    grid_refs.retain(|c| {
        grid.get_piece(c).unwrap().get_team_uuid().unwrap() != Attack::get_team_uuid(attacker)
    });
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
    let mut tree = populate_tree(&attacker, grid, depth.into());
    tree
}

pub fn in_range_with_dyn<'a>(attacker: &dyn Attack, grid: &'a IGrid) -> Vec<&'a Coord> {
    // create a new tree with the attacker's coord as the root and the range as the depth
    let coords = attacker.get_coord();
    let mut previous = vec![*attacker.get_coord()];
    let tree = populate_attack_tree_with_dyn(attacker, grid, &mut previous);
    let in_range = tree.get_list_of_children();

    // to not have to clone the pieces we can reference the coords inside the grid
    let mut grid_refs: Vec<Option<&Coord>> =
        in_range.iter().map(|&c| grid.get_coord_ref(c)).collect();
    grid_refs.retain(|&c| c.is_some());
    let mut grid_refs: Vec<&Coord> = grid_refs.iter().map(|&c| c.unwrap()).collect();
    grid_refs.retain(|&c| c != coords);
    grid_refs.retain(|c| grid.get_piece(c).is_some());
    grid_refs.retain(|c| grid.get_piece(c).unwrap().can_be_attacked());
    // we only keep the ones without the same team as the attacker
    grid_refs.retain(|c| grid.get_piece(c).unwrap().get_team_uuid().is_some());
    grid_refs.retain(|c| {
        grid.get_piece(c).unwrap().get_team_uuid().unwrap() != Attack::get_team_uuid(attacker)
    });
    // we return the coords of the pieces that can be attacked
    grid_refs
}

fn populate_attack_tree_with_dyn(
    attacker: &dyn Attack,
    grid: &IGrid,
    previous: &mut Vec<Coord>,
) -> ThreeProngedTree {
    let coord = attacker.get_coord();
    let depth = attacker.get_range();
    let attacker = grid.get_piece(attacker.get_coord()).unwrap();
    let mut tree = populate_tree(&attacker, grid, depth.into());
    tree
}
