use crate::{grid::isometric_grid::IGrid, pieces::tree::Slot::*};

use super::{
    traits::{Move, Piece},
    tree::ThreeProngedTree,
};

pub fn not_blocked<T>(movable: &T, grid: &IGrid) -> bool
where
    T: Move,
{
    let mut tree = ThreeProngedTree::from(Available(*movable.get_coord()));
    tree.populate_for_depth(grid, &mut vec![*movable.get_coord()], 1);
    // get the list of children
    let mut children = tree.get_list_of_children();
    // if the list is empty, then the piece can't move
    if children.is_empty() {
        return false;
    }
    // if the list is not empty, then the piece can move
    true
}

pub fn can_move(movable: &Box<dyn Piece>, grid: &IGrid) -> bool {
    let mut tree = ThreeProngedTree::from(Available(*movable.get_coord()));
    tree.populate_for_depth(grid, &mut vec![*movable.get_coord()], 1);
    // get the list of children
    let mut children = tree.get_list_of_children();
    // if the list is empty, then the piece can't move
    if children.is_empty() {
        return false;
    }
    // if the list is not empty, then the piece can move
    true
}
