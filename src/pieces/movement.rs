use std::io::{Error, ErrorKind};

// use crate::pieces::movement::Slot::*;

use crate::{
    grid::isometric_grid::{Coord, IGrid},
    pieces::movement::Slot::*,
};

use super::traits::Move;

pub fn populate_tree<T>(coord: &Coord, movable: T, grid: &IGrid) -> ThreeProngedTree
where
    T: Move,
{
    let mut tree = ThreeProngedTree::from(Available(*coord));
    let depth = movable.get_moves().unwrap();
    let mut previous = vec![*coord];
    tree.populate_last_layer(grid, &mut previous, depth);
    tree.set_list_of_children(previous);
    tree
}

#[derive(Debug, Clone)]
pub struct ThreeProngedTree {
    first: Slot<TwoProngedTree>,
    second: Slot<TwoProngedTree>,
    third: Slot<TwoProngedTree>,
    value: Slot<Coord>,
    list_of_children: Vec<Coord>,
}

impl ThreeProngedTree {
    fn new() -> ThreeProngedTree {
        //
        ThreeProngedTree {
            first: Unchecked,
            second: Unchecked,
            third: Unchecked,
            value: Unchecked,
            list_of_children: Vec::new(),
        }
    }
    pub fn get_list_of_children(&self) -> &Vec<Coord> {
        &self.list_of_children
    }
    fn set_list_of_children(&mut self, list: Vec<Coord>) {
        self.list_of_children = list;
    }
    pub fn get_path_to_coord(&self, coord: &Coord) -> Result<Vec<&Coord>, Error> {
        if !self.get_list_of_children().contains(coord) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Coord not found in list of children",
            ));
        } else {
            let mut vector = vec![];
            let mut first_value = self.get_child_from_coord(&coord); // LII maybe make it a hash map instead of a vector?
            loop {
                let test = first_value.get_coord_ref();
                let root = self.get_root();
                vector.push(test);
                let coord = first_value.get_parent_coord();
                if root == coord {
                    vector.push(root);
                    break;
                }
                first_value = self.get_child_from_coord(coord);
            }
            Ok(vector)
        }
    }
    fn get_child_from_coord(&self, coord: &Coord) -> &TwoProngedTree {
        let mut value = self.first.get_ref().traverse_to_coord(coord);
        if let None = value {
            value = self.second.get_ref().traverse_to_coord(coord);
        }
        if let None = value {
            value = self.third.get_ref().traverse_to_coord(coord);
        }
        return value.unwrap();
    }
    fn get_value(&self) -> &Slot<Coord> {
        &self.value
    }
    fn from(coord: Slot<Coord>) -> ThreeProngedTree {
        ThreeProngedTree {
            first: Unchecked,
            second: Unchecked,
            third: Unchecked,
            value: coord,
            list_of_children: Vec::new(),
        }
    }
    fn set_no_available(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Available(tree),
            2 => self.second = Available(tree),
            3 => self.third = Available(tree),
            _ => panic!("Invalid no for three pronged tree"),
        }
    }
    fn set_no_unavailable(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = UnAvailable(tree),
            2 => self.second = UnAvailable(tree),
            3 => self.third = UnAvailable(tree),
            _ => panic!("Invalid no for three pronged tree"),
        }
    }
    fn set_no_blocked(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Blocked(tree),
            2 => self.second = Blocked(tree),
            3 => self.third = Blocked(tree),
            _ => panic!("Invalid no for three pronged tree"),
        }
    }
    fn set_no_unchecked(&mut self, no: i8) {
        match no {
            1 => self.first = Unchecked,
            2 => self.second = Unchecked,
            3 => self.third = Unchecked,
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
                    let tree = TwoProngedTree::from(Available(coord), *self.get_root());
                    previous.push(coord);
                    self.set_no_available(tree, counter);
                }
                CoordType::UnAvailable(_) => {
                    let tree = TwoProngedTree::from(UnAvailable(coord), *self.get_root());
                    previous.push(coord);
                    self.set_no_unavailable(tree, counter);
                }
                CoordType::OutOfBounds(_) => {
                    let tree = TwoProngedTree::from(Blocked(coord), *self.get_root());
                    previous.push(coord);
                    //HACK this might bite you in the ass later its supposed to be unchecked but its blocked (not gonna work if you want grid expansion)
                    self.set_no_unchecked(counter);
                }
            }
        }
    }
    fn get_available_branches(&mut self) -> Vec<&mut TwoProngedTree> {
        let mut layer = Vec::new();
        if let Available(ref mut tree) = self.first {
            layer.push(tree);
        }
        if let Available(ref mut tree) = self.second {
            layer.push(tree);
        }
        if let Available(ref mut tree) = self.third {
            layer.push(tree);
        }
        layer
    }
    fn get_bottom_branches(&mut self) -> Vec<&mut TwoProngedTree> {
        let mut branches = Vec::new();
        for branchy in self.get_available_branches() {
            if !branchy.is_blocked() {
                for branch in branchy.get_bottom_branches() {
                    branches.push(branch);
                }
            }
        }
        branches
    }

    fn get_root(&self) -> &Coord {
        self.value.get_ref()
    }
    fn populate_last_layer(&mut self, grid: &IGrid, previous: &mut Vec<Coord>, depth: i8) {
        for current_depth in 0..depth {
            println!("");
            println!("current depth is {}", current_depth);
            println!(
                "lenght of bottom branches is {}",
                self.get_bottom_branches().len()
            );
            println!("");
            if self.get_bottom_branches().len() == 0 {
                if current_depth != 0 {
                    break;
                } else {
                    let root = self.get_root();
                    self.populate_first_layer(AvailableMoves::from(&root), grid, previous);
                    continue;
                }
            }
            for small_tree in self.get_bottom_branches() {
                if let TwoProngedTree {
                    first: _,
                    second: _,
                    value: Available(_),
                    parent_coord: _,
                } = small_tree
                {
                    let available_moves = AvailableMoves::from(&small_tree.get_coord());
                    let coord_provenance = small_tree.get_coord();
                    small_tree.populate_first_layer(
                        available_moves,
                        grid,
                        previous,
                        coord_provenance,
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TwoProngedTree {
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
    fn traverse_to_coord(&self, coord: &Coord) -> Option<&TwoProngedTree> {
        //LII maybe could be shortened
        let mut return_value = None;
        if self.get_value() == coord {
            return_value = Some(self);
        } else {
            match self.first {
                Available(ref tree) => {
                    if let Some(tree) = tree.traverse_to_coord(coord) {
                        return_value = Some(tree);
                    }
                }
                _ => {}
            }
            match self.second {
                Available(ref tree) => {
                    if let Some(tree) = tree.traverse_to_coord(coord) {
                        return_value = Some(tree);
                    }
                }
                _ => {}
            }
        }
        return_value
    }
    fn set_no_available(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Available(Box::new(tree)),
            2 => self.second = Available(Box::new(tree)),
            _ => panic!(
                "Invalid no for TwoProngedTree wanted to add tree {:?}, with index {}",
                tree, no
            ),
        }
    }
    fn set_no_unavailable(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = UnAvailable(Box::new(tree)),
            2 => self.second = UnAvailable(Box::new(tree)),
            _ => panic!(
                "Invalid no for TwoProngedTree wanted to add tree {:?}, with index {}",
                tree, no
            ),
        }
    }
    fn set_no_blocked(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Blocked(Box::new(tree)),
            2 => self.second = Blocked(Box::new(tree)),
            _ => panic!(
                "Invalid no for TwoProngedTree wanted to add tree {:?}, with index {}",
                tree, no
            ),
        }
    }
    fn set_no_unchecked(&mut self, tree: TwoProngedTree, no: i8) {
        match no {
            1 => self.first = Unchecked,
            2 => self.second = Unchecked,
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
            if coord == *self.get_parent_coord() {
                counter -= 1;
            } else if previous.contains(&coord) {
                let tree = TwoProngedTree::from(Blocked(coord), coord_provenance);
                self.set_no_blocked(tree, counter);
            } else {
                match grid.type_of_slot(&coord) {
                    CoordType::Available(_) => {
                        let tree = TwoProngedTree::from(Available(coord), coord_provenance);
                        previous.push(coord);
                        self.set_no_available(tree, counter);
                    }
                    CoordType::UnAvailable(_) => {
                        let tree = TwoProngedTree::from(UnAvailable(coord), coord_provenance);
                        previous.push(coord);
                        self.set_no_unavailable(tree, counter);
                    }
                    CoordType::OutOfBounds(_) => {
                        let tree = TwoProngedTree::from(Unchecked, coord_provenance);
                        previous.push(coord);
                        // HACK this is supposed to be unchecked, but it is blocked because it would break the bottom layer fn
                        self.set_no_blocked(tree, counter);
                    }
                }
            }
        }
    }
    fn get_coord(&mut self) -> Coord {
        *self.value.unwrap()
    }
    fn get_coord_ref(&self) -> &Coord {
        self.value.get_ref()
    }
    fn get_value(&self) -> &Coord {
        self.value.get_ref()
    }
    fn is_available(&self) -> bool {
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
            Blocked(_) => true,
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
            Blocked(_) => true,
            UnAvailable(_) => true,
            _ => false,
        }
    }
    fn get_bottom_branches(&mut self) -> Vec<&mut TwoProngedTree> {
        let mut branches = Vec::new();
        if self.dosent_have_any_branches() && !self.is_blocked_or_unavailable() {
            branches.push(self);
            return branches;
        } else {
            let mut layer = Vec::new();
            layer.push(&mut **self.first.get_as_mut_ref());
            layer.push(&mut **self.second.get_as_mut_ref());
            for branch in layer {
                if !branch.is_blocked_or_unavailable() {
                    branches.append(&mut branch.get_bottom_branches());
                }
            }
            return branches;
        }
    }
    fn get_parent_coord(&self) -> &Coord {
        &self.parent_coord
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
        let mut one = Some(*coord);
        let mut two = Some(*coord);
        let mut three = Some(*coord);
        if coord.get_y() % 2 == coord.get_x() % 2 {
            // simplify this
            three.as_mut().unwrap().sub_y(1) // if x is odd and y is odd or even even
        } else {
            three.as_mut().unwrap().add_y(1); // if x is odd and y is even
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
    Blocked(T),
    Available(T),
    UnAvailable(T),
}

impl<T> Slot<T> {
    fn unwrap(&mut self) -> &mut T {
        match self {
            Slot::Unchecked => panic!("Unchecked slot with the unwrap function"),
            Slot::Blocked(t) => t,
            Slot::Available(t) => t,
            Slot::UnAvailable(t) => t,
        }
    }
    fn get_as_mut_ref(&mut self) -> &mut T {
        match self {
            Slot::Unchecked => panic!("Unchecked slot with the get_as_mut_ref function"),
            Slot::Blocked(t) => t,
            Slot::Available(t) => t,
            Slot::UnAvailable(t) => t,
        }
    }
    fn get_ref(&self) -> &T {
        match self {
            Slot::Unchecked => panic!("Unchecked slot with the get_tree function"),
            Slot::Blocked(t) => t,
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
