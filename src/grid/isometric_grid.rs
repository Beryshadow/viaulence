use crate::pieces::{
    movement::CoordType::{self, *},
    tokens::Piece,
};

use core::hash::Hash;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct IGrid {
    grid_pieces: HashMap<Coord, Piece>,
    top_left: Coord,
    bottom_right: Coord,
}

impl IGrid {
    pub fn new() -> IGrid {
        IGrid {
            grid_pieces: HashMap::new(),
            top_left: Coord::new(),
            bottom_right: Coord::new(),
        }
    }

    pub fn from(top_left_coord: Coord, bottom_right_coord: Coord) -> IGrid {
        IGrid {
            grid_pieces: HashMap::new(),
            top_left: top_left_coord,
            bottom_right: bottom_right_coord,
        }
    }

    pub fn add_piece(&mut self, piece: Piece, coord: Coord) {
        if !self.grid_pieces.contains_key(&coord) {
            self.grid_pieces.insert(coord, piece);
        } else {
            match self.grid_pieces.get(&coord) {
                Some(Piece::Empty) => {
                    self.grid_pieces.insert(coord, piece);
                }
                Some(Piece::GoldPot(_)) => {
                    self.grid_pieces.insert(coord, piece);
                    self.grid_pieces
                        .get_mut(&coord)
                        .unwrap()
                        .as_mut()
                        .change_pot_state(true)
                        .change_base_state(false)
                        .change_immune_state(true);
                }
                Some(Piece::Base(_)) => {
                    self.grid_pieces.insert(coord, piece);
                    self.grid_pieces
                        .get_mut(&coord)
                        .unwrap()
                        .as_mut()
                        .change_pot_state(false)
                        .change_base_state(true)
                        .change_immune_state(true);
                }
                _ => {}
            }
        }
    }

    pub fn is_valid(&self, coord: &Coord) -> bool {
        if !self.coord_in_grid(coord) {
            return false;
        } else if self.grid_pieces.contains_key(&coord) {
            match self.grid_pieces.get(&coord) {
                Some(Piece::Empty) => true,
                Some(Piece::Scout(_)) => false,
                Some(Piece::Tank(_)) => false,
                Some(Piece::Soldier(_)) => false,
                Some(Piece::Medic(_)) => false,
                Some(Piece::Wall(_)) => false,
                Some(Piece::Base(_)) => true,
                Some(Piece::GoldPot(_)) => true,
                _ => false,
            }
        } else {
            true
        }
    }

    fn coord_in_grid(&self, coord: &Coord) -> bool {
        if coord.get_x() < self.top_left.get_x()
            || coord.get_y() < self.top_left.get_y()
            || coord.get_x() > self.bottom_right.get_x()
            || coord.get_y() > self.bottom_right.get_y()
        {
            return false;
        } else {
            true
        }
    }

    pub fn coord_type(&self, coord: &Coord) -> CoordType {
        if self.is_valid(coord) {
            Available(*coord)
        } else if self.coord_in_grid(coord) {
            UnAvailable(*coord)
        } else {
            OutOfBounds(*coord)
        }
    }

    pub fn get_hight(&self) -> i32 {
        self.bottom_right.get_y() - self.top_left.get_y() + 1
    }
    pub fn get_width(&self) -> i32 {
        self.bottom_right.get_x() - self.top_left.get_x() + 1
    }
    pub fn get_coord(&self, piece: &Piece) -> Option<Coord> {
        for (coord, piece_in_grid) in self.grid_pieces.iter() {
            if piece_in_grid == piece {
                return Some(*coord);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new() -> Coord {
        Coord { x: 0, y: 0 }
    }
    pub fn from(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn add_x(&mut self, x: i32) {
        self.x += x;
    }
    pub fn add_y(&mut self, y: i32) {
        self.y += y;
    }
    pub fn sub_x(&mut self, x: i32) {
        self.x -= x;
    }
    pub fn sub_y(&mut self, y: i32) {
        self.y -= y;
    }
}
