use crate::movement::Slot::*;

use crate::{
    isometric_grid::{Coord, IGrid},
    tokens::Piece,
};

// here is where we will check available moves, make a ternary tree with binary tree as branches, and then check if the move is valid
// if it is valid we will have the leaf as a possible valid move, if not we will have a leaf with a value of None
// we will also check if the tree has multiple leaf with the same value (excluding None) before adding it to the list of possible moves
pub fn populate_tree(coord: &Coord, piece: &Piece, grid: &IGrid) -> ThreeProngedTree {
    let mut tree = ThreeProngedTree::from(Some(*coord));
    let depth = piece.get_moves().unwrap();
    let initial_moves = AvailableMoves::from(coord, grid);
    let mut previous = vec![*coord];
    tree.populate_available_moves(initial_moves, grid, depth, &mut previous);
    println!("{:?}", previous);
    tree
}

#[derive(Debug, Clone)]
pub struct ThreeProngedTree {
    first: Option<Box<TwoProngedTree>>,
    second: Option<Box<TwoProngedTree>>,
    third: Option<Box<TwoProngedTree>>,
    value: Option<Coord>,
}

impl ThreeProngedTree {
    fn new() -> ThreeProngedTree {
        ThreeProngedTree {
            first: None,
            second: None,
            third: None,
            value: None,
        }
    }
    fn from(coord: Option<Coord>) -> ThreeProngedTree {
        ThreeProngedTree {
            first: None,
            second: None,
            third: None,
            value: coord,
        }
    }
    fn set_no(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Some(Box::new(tree)),
            2 => self.second = Some(Box::new(tree)),
            3 => self.third = Some(Box::new(tree)),
            _ => panic!("Invalid no for three pronged tree"),
        }
    }
    fn populate_available_moves(
        &mut self,
        moves: AvailableMoves,
        grid: &IGrid,
        depth: i8,
        previous: &mut Vec<Coord>,
    ) {
        let depth = depth - 1;
        if depth == 0 {
            return;
        }
        let mut counter = 0;
        for coord in moves {
            counter += 1;
            let mut tree = TwoProngedTree::from(Available(coord));
            previous.push(coord);
            let moves = AvailableMoves::from(&coord, grid);
            tree.populate_available_moves(moves, grid, depth, previous);
            self.set_no(tree, counter);
        }
    }
}

#[derive(Debug, Clone)]
struct TwoProngedTree {
    first: Slot<Box<TwoProngedTree>>,
    second: Slot<Box<TwoProngedTree>>,
    value: Slot<Coord>,
}

impl TwoProngedTree {
    fn new() -> TwoProngedTree {
        TwoProngedTree {
            first: Unchecked,
            second: Unchecked,
            value: Unchecked,
        }
    }
    fn from(coord: Slot<Coord>) -> TwoProngedTree {
        TwoProngedTree {
            first: Unchecked,
            second: Unchecked,
            value: coord,
        }
    }
    fn set_no(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Available(Box::new(tree)),
            2 => self.second = Available(Box::new(tree)),
            _ => panic!("Invalid no for TwoProngedTree"),
        }
    }
    fn populate_available_moves(
        &mut self,
        moves: AvailableMoves,
        grid: &IGrid,
        depth: i8,
        previous: &mut Vec<Coord>,
    ) {
        let depth = depth - 1;
        if depth != 0 {
            let mut counter = 0;
            for coord in moves {
                counter += 1;
                if previous.contains(&coord) {
                    counter -= 1;
                    continue;
                }
                let mut tree = TwoProngedTree::from(Available(coord));
                previous.push(coord);
                tree.populate_available_moves(moves, grid, depth, previous);
                self.set_no(tree, counter);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AvailableMoves {
    move_one: Option<Coord>,
    move_two: Option<Coord>,
    move_three: Option<Coord>,
}

impl AvailableMoves {
    fn new() -> AvailableMoves {
        AvailableMoves {
            move_one: Some(Coord::new()),
            move_two: Some(Coord::new()),
            move_three: Some(Coord::new()),
        }
    }
    fn from(coord: &Coord, grid: &IGrid) -> AvailableMoves {
        let mut one = Some(coord.clone());
        let mut two = Some(coord.clone());
        let mut three = Some(coord.clone());
        if !coord.clone().get_y() % 2 == 0 {
            if coord.clone().get_x() % 2 == 0 {
                three.as_mut().unwrap().add_y(1)
            } else {
                three.as_mut().unwrap().sub_y(1)
            }
        } else {
            if coord.clone().get_x() % 2 == 0 {
                three.as_mut().unwrap().sub_y(1)
            } else {
                three.as_mut().unwrap().add_y(1)
            }
        }
        one.as_mut().unwrap().add_x(1);
        two.as_mut().unwrap().sub_x(1);

        if !grid.is_valid(one.unwrap()) {
            one = None;
        }

        if !grid.is_valid(two.unwrap()) {
            two = None;
        }

        if !grid.is_valid(three.unwrap()) {
            three = None;
        }

        AvailableMoves {
            move_one: one,
            move_two: two,
            move_three: three,
        }
    }
}

impl IntoIterator for AvailableMoves {
    type Item = Coord;
    type IntoIter = AvailableMovesIterator;
    fn into_iter(self) -> AvailableMovesIterator {
        AvailableMovesIterator {
            moves: self,
            index: 0,
        }
    }
}

struct AvailableMovesIterator {
    moves: AvailableMoves,
    index: usize,
}

impl Iterator for AvailableMovesIterator {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        match index {
            0 => self.moves.move_one.clone(),
            1 => self.moves.move_two.clone(),
            2 => self.moves.move_three.clone(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Slot<T> {
    Available(T),
    Occupied(T),
    Unchecked,
}
