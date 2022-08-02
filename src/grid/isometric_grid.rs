use crate::pieces::{
    traits::Piece,
    tree::CoordType::{self, *},
};

use core::hash::Hash;
use std::{collections::HashMap, fmt::Debug, io::Error};

// #[derive(Debug, Clone)]
pub struct IGrid {
    grid_pieces: HashMap<Coord, Box<dyn Piece>>, //LMKL
    top_left: Coord,
    bottom_right: Coord,
}

impl Debug for IGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IGrid {{ grid_pieces: {:#?}, top_left: {:#?}, bottom_right: {:#?} }}",
            self.grid_pieces, self.top_left, self.bottom_right
        )
    }
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

    pub fn add_piece<T>(&mut self, piece: T) -> Result<(), Error>
    where
        T: Piece + Debug + 'static + Copy,
    {
        let coord = piece.get_coord();
        println!("piece is {:?}", piece);
        if !self.grid_pieces.contains_key(coord.unwrap()) {
            let piece = Box::from(piece);
            self.grid_pieces.insert(*coord.unwrap(), piece);
        } else {
            match self.grid_pieces.get(&coord.unwrap()) {
                Some(GoldPot) => {
                    let piece = Box::new(piece);
                    self.grid_pieces.insert(*coord.unwrap(), piece);
                    let mut temp = self.grid_pieces.get_mut(&coord.unwrap()).unwrap().as_mut();
                    temp.set_on_pot(true);
                    temp.set_on_base(false);
                    temp.change_immune_state(true);
                }
                Some(Base) => {
                    let piece = Box::new(piece);
                    self.grid_pieces.insert(*coord.unwrap(), piece);
                    let mut temp = self.grid_pieces.get_mut(&coord.unwrap()).unwrap().as_mut();
                    temp.set_on_pot(false);
                    temp.set_on_base(true);
                    temp.change_immune_state(true);
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn remove_piece(&mut self, coord: &Coord) {
        self.grid_pieces.remove(coord);
    }

    // check if the coord is a valid place to put a piece
    pub fn is_valid(&self, coord: &Coord) -> bool {
        if !self.coord_in_grid(coord) {
            return false;
        } else if self.grid_pieces.contains_key(coord) {
            // get the piece at the coord
            let piece = self.grid_pieces.get(coord).unwrap();
            piece.can_host_piece()
        } else {
            true
        }
    }

    // old version of coord valid

    /* if !self.coord_in_grid(coord) {
        return false;
    } else if self.grid_pieces.contains_key(&coord) {
        match self.grid_pieces.get(&coord) {
            Some(PieceNeedsToGo::Empty) => true,
            Some(PieceNeedsToGo::Scout(_)) => false,
            Some(PieceNeedsToGo::Tank(_)) => false,
            Some(PieceNeedsToGo::Soldier(_)) => false,
            Some(PieceNeedsToGo::Medic(_)) => false,
            Some(PieceNeedsToGo::Wall(_)) => false,
            Some(PieceNeedsToGo::Base(_)) => true,
            Some(PieceNeedsToGo::GoldPot(_)) => true,
            _ => false,
        }
    } else {
        true
    } */

    // check if the coord is inside the grid
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

    // check the coord type
    pub fn coord_type(&self, coord: &Coord) -> CoordType {
        if self.is_valid(coord) {
            Available(*coord)
        } else if self.coord_in_grid(coord) {
            Occupied(*coord)
        } else {
            OutOfBounds(*coord)
        }
    }

    // get the height of the grid
    pub fn get_height(&self) -> i32 {
        self.bottom_right.get_y() - self.top_left.get_y() + 1
    }
    // get the width of the grid
    pub fn get_width(&self) -> i32 {
        self.bottom_right.get_x() - self.top_left.get_x() + 1
    }
    // get the coord of a piece
    pub fn get_coord<T>(&self, piece: &T) -> Option<Coord>
    where
        T: Piece,
    {
        for (coord, piece_) in &self.grid_pieces {
            if piece_.get_uuid() == piece.get_uuid() {
                return Some(*coord);
            }
        }
        None
    }

    // get the piece at a coord
    pub fn get_piece(&self, coord: &Coord) -> Option<&Box<dyn Piece>> {
        self.grid_pieces.get(coord)
    }
    // get a reference to a coord in the grid from a owned coord if it exists
    pub fn get_coord_ref(&self, coord: Coord) -> Option<&Coord> {
        let piece = self.grid_pieces.get(&coord);
        if piece.is_some() {
            Some(piece.unwrap().get_coord().unwrap())
        } else {
            None
        }
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
