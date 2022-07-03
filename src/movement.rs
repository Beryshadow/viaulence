use crate::movement::Slot::*;

use crate::{
    isometric_grid::{Coord, IGrid},
    tokens::Piece,
};

// here is where we will check available moves, make a ternary tree with binary tree as branches, and then check if the move is valid
// if it is valid we will have the leaf as a possible valid move, if not we will have a leaf with a value of None
// we will also check if the tree has multiple leaf with the same value (excluding None) before adding it to the list of possible moves
pub fn populate_tree(coord: &Coord, piece: &Piece, grid: &IGrid) -> ThreeProngedTree {
    let mut tree = ThreeProngedTree::from(Available(*coord));
    let depth = piece.get_moves().unwrap();
    let initial_moves = AvailableMoves::from(coord, grid);
    let mut previous = vec![*coord];
    tree.populate_first_layer(initial_moves, grid, &mut previous);

    for mut small_tree in tree.get_branches() {
        println!("the trees coord are {:?}", small_tree.get_coord());
        match small_tree {
            TwoProngedTree {
                first: _,
                second: _,
                value: Available(_),
            } => {
                let mut interior = TwoProngedTree::from(Available(*coord));
                let available_moves = AvailableMoves::from(&interior.get_coord(), grid);
                for tree_coord in available_moves {
                    println!("its available");
                    small_tree.populate_first_layer(
                        AvailableMoves::from(&tree_coord, grid),
                        grid,
                        &mut previous,
                        interior.get_coord(),
                    );
                }
            }
            _ => {
                println!("its blocked")
            }
        }
    }
    // let test_coords = Coord::from(-2, 4);
    // let validity_test = grid.is_valid(&test_coords);
    // let test = grid.type_of_slot(&test_coords); //should return available
    // println!("{:?} and is_valid results in {}", test, validity_test);
    tree
}

#[derive(Debug, Clone)]
pub struct ThreeProngedTree {
    first: Slot<TwoProngedTree>,
    second: Slot<TwoProngedTree>,
    third: Slot<TwoProngedTree>,
    value: Slot<Coord>,
}

impl ThreeProngedTree {
    fn new() -> ThreeProngedTree {
        ThreeProngedTree {
            first: Unchecked,
            second: Unchecked,
            third: Unchecked,
            value: Unchecked,
        }
    }
    fn from(coord: Slot<Coord>) -> ThreeProngedTree {
        ThreeProngedTree {
            first: Unchecked,
            second: Unchecked,
            third: Unchecked,
            value: coord,
        }
    }
    fn set_no(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Available(tree),
            2 => self.second = Available(tree),
            3 => self.third = Available(tree),
            _ => panic!("Invalid no for three pronged tree"),
        }
    }
    fn populate_first_layer(
        &mut self,
        moves: AvailableMoves,
        grid: &IGrid,
        previous: &mut Vec<Coord>,
    ) {
        let mut counter = 0;
        for coord in moves {
            counter += 1;
            match grid.type_of_slot(&coord) {
                CoordType::Available(_) => {
                    let tree = TwoProngedTree::from(Available(coord));
                    previous.push(coord);
                    self.set_no(tree, counter);
                }
                CoordType::UnAvailable(_) => {
                    let tree = TwoProngedTree::from(UnAvailable(coord));
                    previous.push(coord);
                    self.set_no(tree, counter);
                }
                CoordType::OutOfBounds(_) => {
                    let tree = TwoProngedTree::from(Blocked);
                    previous.push(coord);
                    self.set_no(tree, counter);
                } // let mut tree = TwoProngedTree::from(Available(coord));
                  // previous.push(coord);
                  // let moves = AvailableMoves::from(&coord, grid);
                  // tree.populate_available_moves(moves, grid, previous, coord);
                  // self.set_no(tree, counter);
            }
        }
    }
    fn get_branches(&mut self) -> Vec<&mut TwoProngedTree> {
        let mut layer = Vec::new();
        layer.push(self.first.get_as_mut_ref());
        layer.push(self.second.get_as_mut_ref());
        layer.push(self.third.get_as_mut_ref());
        layer
    }
    fn get_root(&mut self) -> Coord {
        *self.value.unwrap()
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
    fn from(slot: Slot<Coord>) -> TwoProngedTree {
        TwoProngedTree {
            first: Unchecked,
            second: Unchecked,
            value: slot,
        }
    }
    fn set_no(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Available(Box::new(tree)),
            2 => self.second = Available(Box::new(tree)),
            _ => panic!(
                "Invalid no for TwoProngedTree wanted to add tree {:?}, with index {}",
                tree, no
            ),
        }
    }
    fn populate_first_layer(
        &mut self,
        moves: AvailableMoves,
        grid: &IGrid,
        previous: &mut Vec<Coord>,
        coord_provenance: Coord,
    ) {
        let mut counter = 0;
        for coord in moves {
            counter += 1;
            if coord == coord_provenance {
                // println!("previous coord is the same as the current coord");
                // println!("{:?}", coord);
                counter -= 1;
            } else if previous.contains(&coord) {
                // println!("this coord is in the previous list");
                // println!("checking for {:?}", coord);
                // println!("coming from {coord_provenance:?}");
                // println!("list of all cached{:?}", previous);
                let mut tree = TwoProngedTree::from(Blocked);
                self.set_no(tree, counter);
            } else {
                match grid.type_of_slot(&coord) {
                    CoordType::Available(_) => {
                        let tree = TwoProngedTree::from(Unchecked);
                        previous.push(coord);
                        self.set_no(tree, counter);
                    }
                    CoordType::UnAvailable(_) => {
                        // will never happen since we will never iterate over unavailable slots
                        let tree = TwoProngedTree::from(Blocked);
                        previous.push(coord);
                        self.set_no(tree, counter);
                    }
                    CoordType::OutOfBounds(_) => {
                        let tree = TwoProngedTree::from(Blocked);
                        previous.push(coord);
                        self.set_no(tree, counter);
                    }
                }
                // let mut tree = TwoProngedTree::from(Available(coord));
                // previous.push(coord);
                // tree.populate_available_moves(moves, grid, depth, previous, coord);
                // self.set_no(tree, counter);
            }
        }
    }
    fn get_coord(&mut self) -> Coord {
        *self.value.unwrap()
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

        // will remove iteration  but I would need to fix the code to make it work

        // if !grid.is_valid(&one.unwrap()) {
        //     one = None;
        // }

        // if !grid.is_valid(&two.unwrap()) {
        //     two = None;
        // }

        // if !grid.is_valid(&three.unwrap()) {
        //     three = None;
        // }

        AvailableMoves {
            move_one: one,
            move_two: two,
            move_three: three,
        }
    }
    fn move_(&self, no: usize) -> Option<Coord> {
        match no {
            0 => self.move_one,
            1 => self.move_two,
            2 => self.move_three,
            _ => panic!("Invalid no for AvailableMoves"),
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
        // get the next value that is available skip the None and stop at 3
        while self.index < 3 {
            let coord = self.moves.move_(self.index);
            self.index += 1;
            if coord.is_some() {
                return coord;
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Slot<T> {
    Unchecked,
    Blocked,
    Available(T),
    UnAvailable(T),
}

impl<T> Slot<T> {
    fn unwrap(&mut self) -> &mut T {
        match self {
            Slot::Unchecked => panic!("Unchecked slot"),
            Slot::Blocked => panic!("Blocked slot"),
            Slot::Available(T) => T,
            Slot::UnAvailable(T) => T,
        }
    }
    fn get_as_mut_ref(&mut self) -> &mut T {
        match self {
            Slot::Unchecked => panic!("Unchecked slot"),
            Slot::Blocked => panic!("Blocked slot"),
            Slot::Available(T) => T,
            Slot::UnAvailable(T) => T,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CoordType {
    OutOfBounds(Coord),
    Available(Coord),
    UnAvailable(Coord),
}
