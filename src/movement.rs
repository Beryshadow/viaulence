use crate::movement::Slot::*;

use crate::{
    isometric_grid::{Coord, IGrid},
    tokens::Piece,
};

// here is where we will check available moves, make a ternary tree with binary tree as branches, and then check if the move is valid
// if it is valid we will have the leaf as a possible valid move, if not we will have a leaf with a specific value
// we will also check if the tree has multiple leaf with the same value (excluding None) before adding it to the list of possible moves
pub fn populate_tree(coord: &Coord, piece: &Piece, grid: &IGrid) -> ThreeProngedTree {
    let mut tree = ThreeProngedTree::from(Available(*coord));
    let depth = piece.get_moves().unwrap();
    let initial_moves = AvailableMoves::from(coord);
    let mut previous = vec![*coord];
    println!(
        "tree.get_bottom_branches(); {:?}",
        tree.get_bottom_branches() // TODO: this should work on an empty tree
    );
    tree.populate_first_layer(initial_moves, grid, &mut previous); // this is broken because it doesn't check if the move is valid

    for oof in 0..depth {
        println!("");
        println!("{oof}");
        println!(
            "this is tree.get_bottom_branches lenght: {:#?}",
            tree.get_bottom_branches().len()
        );
        println!("");
        println!("{:?}", previous);
        if tree.get_bottom_branches().len() == 0 {
            break;
        }
        for small_tree in tree.get_bottom_branches() {
            match small_tree {
                TwoProngedTree {
                    first: _,
                    second: _,
                    value: Available(_),
                    parent_coord: _,
                } => {
                    let available_moves = AvailableMoves::from(&small_tree.get_coord());
                    let coord_provenance = small_tree.get_coord();
                    small_tree.populate_first_layer(
                        available_moves,
                        grid,
                        &mut previous,
                        coord_provenance,
                    );
                }
                TwoProngedTree {
                    first: _,
                    second: _,
                    value: Unchecked,
                    parent_coord: _,
                } => {
                    // print error message
                    eprintln!("error: this should not happen");
                }
                _ => {}
            }
        }
    }
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
                    let tree = TwoProngedTree::from(Available(coord), self.get_root());
                    previous.push(coord);
                    self.set_no(tree, counter);
                }
                CoordType::UnAvailable(_) => {
                    let tree = TwoProngedTree::from(UnAvailable(coord), self.get_root());
                    previous.push(coord);
                    self.set_no(tree, counter);
                }
                CoordType::OutOfBounds(_) => {
                    let tree = TwoProngedTree::from(Blocked, self.get_root());
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
        if self.first.get_as_mut_ref().is_available() {
            layer.push(self.first.get_as_mut_ref());
        }
        if self.second.get_as_mut_ref().is_available() {
            layer.push(self.second.get_as_mut_ref());
        }
        if self.third.get_as_mut_ref().is_available() {
            layer.push(self.third.get_as_mut_ref());
        }
        // layer.push(self.first.get_as_mut_ref());
        // layer.push(self.second.get_as_mut_ref());
        // layer.push(self.third.get_as_mut_ref());
        layer
    }
    fn get_bottom_branches(&mut self) -> Vec<&mut TwoProngedTree> {
        let mut branches = Vec::new();
        // here we need to check if the tree is Unchecked Blocked Available or UnAvailable
        // let what_is_self_get_branches = self.get_branches();
        // println!(
        //     "what_is_self_get_branches jaueduawiuuuawuyuawyeduawueduiawudheaw {:#?}",
        //     what_is_self_get_branches
        // );
        for branchy in self.get_branches() {
            for branch in branchy.get_bottom_branches() {
                branches.push(branch);
            }
        }
        branches
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
    parent_coord: Coord,
}

impl TwoProngedTree {
    fn new() -> TwoProngedTree {
        TwoProngedTree {
            first: Unchecked,
            second: Unchecked,
            value: Unchecked,
            parent_coord: Coord::new(),
        }
    }
    fn from(slot: Slot<Coord>, parent: Coord) -> TwoProngedTree {
        TwoProngedTree {
            first: Unchecked,
            second: Unchecked,
            value: slot,
            parent_coord: parent,
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
            if coord == self.get_parent_coord() {
                // println!("previous coord is the same as the current coord");
                // println!("{:?}", coord);
                counter -= 1;
            } else if previous.contains(&coord) {
                // println!("");
                // println!("");
                // println!("the tree provenance value is {:?}", coord_provenance);
                // println!("the tree saved prov value is {:?}", self.get_parent_coord());
                // println!("moves available for it are {:?}", moves);
                // println!("");
                // println!("");
                // println!("the tree current root coords are {:?}", self.get_coord());
                // println!("this coord is in the previous list");
                // println!("checking for {:?}", coord);
                // println!("blocked path is {coord_provenance:?}");
                // println!("list of all cached{:?}", previous);
                let tree = TwoProngedTree::from(Blocked, coord_provenance);
                self.set_no(tree, counter);
            } else {
                match grid.type_of_slot(&coord) {
                    CoordType::Available(_) => {
                        let tree = TwoProngedTree::from(Available(coord), coord_provenance);
                        previous.push(coord);
                        self.set_no(tree, counter);
                    }
                    CoordType::UnAvailable(_) => {
                        // will never happen since we will never iterate over unavailable slots
                        let tree = TwoProngedTree::from(UnAvailable(coord), coord_provenance);
                        previous.push(coord);
                        self.set_no(tree, counter);
                    }
                    CoordType::OutOfBounds(_) => {
                        let tree = TwoProngedTree::from(Unchecked, coord_provenance);
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
    fn is_available(&mut self) -> bool {
        match self.value {
            Available(_) => true,
            _ => false,
        }
    }
    fn is_unchecked(&mut self) -> bool {
        match self.value {
            Unchecked => true,
            _ => false,
        }
    }
    fn is_blocked(&mut self) -> bool {
        match self.value {
            Blocked => true,
            _ => false,
        }
    }
    fn is_unavailable(&mut self) -> bool {
        match self.value {
            UnAvailable(_) => true,
            _ => false,
        }
    }
    fn dosent_have_any_branches(&mut self) -> bool {
        match self.first {
            Unchecked => match self.second {
                Unchecked => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn is_blocked_or_unavailable(&mut self) -> bool {
        match self.value {
            Blocked => true,
            UnAvailable(_) => true,
            _ => false,
        }
    }
    fn get_bottom_branches(&mut self) -> Vec<&mut TwoProngedTree> {
        let mut branches = Vec::new();
        // only push if it is unchecked stop and return empty if its blocked or unavailable
        if self.dosent_have_any_branches() {
            // println!("returning this branch {:?}", self);
            branches.push(self);
            return branches; // over here lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quidem.
        } else {
            let mut layer = Vec::new();
            layer.push(&mut **self.first.get_as_mut_ref());
            layer.push(&mut **self.second.get_as_mut_ref());
            for branch in layer {
                if branch.is_blocked_or_unavailable() {
                    branches.push(branch);
                } else {
                    branches.append(&mut branch.get_bottom_branches());
                }
            }
            // println!("returning this bigger layer {:?}", branches);
            return branches;
        }
    }
    fn get_parent_coord(&mut self) -> Coord {
        self.parent_coord
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
    fn from(coord: &Coord) -> AvailableMoves {
        let mut one = Some(coord.clone());
        let mut two = Some(coord.clone());
        let mut three = Some(coord.clone());
        if !coord.clone().get_y() % 2 == 0 {
            if coord.clone().get_x() % 2 == 0 {
                three.as_mut().unwrap().add_y(1) // if x is even and y is odd
            } else {
                three.as_mut().unwrap().sub_y(1) // if x is odd and y is odd
            }
        } else {
            if coord.clone().get_x() % 2 == 0 {
                three.as_mut().unwrap().sub_y(1) // if x is even and y is even
            } else {
                three.as_mut().unwrap().add_y(1); // if x is odd and y is even
            }
        }
        one.as_mut().unwrap().add_x(1);
        two.as_mut().unwrap().sub_x(1);

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
            Slot::Unchecked => panic!("Unchecked slot with the unwrap function"),
            Slot::Blocked => panic!("Blocked slot with the unwrap function"),
            Slot::Available(t) => t,
            Slot::UnAvailable(t) => t,
        }
    }
    fn get_as_mut_ref(&mut self) -> &mut T {
        match self {
            Slot::Unchecked => panic!("Unchecked slot with the get_as_mut_ref function"),
            Slot::Blocked => panic!("Blocked slot with the get_as_mut_ref function"),
            Slot::Available(t) => t,
            Slot::UnAvailable(t) => t,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CoordType {
    OutOfBounds(Coord),
    Available(Coord),
    UnAvailable(Coord),
}
