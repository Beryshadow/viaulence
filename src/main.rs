mod tokens;

use crate::tokens::*;
use core::hash::Hash;
use std::{collections::HashMap, fmt::Debug};

type Piece = tokens::Piece;
fn main() {
    let mut grid = IGrid::new();

    grid.add_piece(Piece::Empty, Coord::from(0, 0));
    grid.add_piece(Piece::Scout(Scout::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Tank(Tank::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Medic(Medic::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Base(Base::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Wall(Wall::new()), Coord::from(0, 0));
    grid.add_piece(Piece::GoldPot(GoldPot::new()), Coord::from(0, 0));

    println!("{:#?}", grid);
}

#[derive(Debug, Clone)]
struct IGrid {
    grid_pieces: HashMap<Coord, Piece>,
}

impl IGrid {
    fn new() -> IGrid {
        IGrid {
            grid_pieces: HashMap::new(),
        }
    }

    fn add_piece(&mut self, piece: Piece, coord: Coord) {
        if !self.grid_pieces.contains_key(&coord) {
            self.grid_pieces.insert(coord, piece);
        } else {
            match self.grid_pieces.get(&coord) {
                Some(Piece::GoldPot(_)) => {
                    self.grid_pieces.insert(coord, piece);
                }
                Some(Piece::Base(_)) => {
                    println!("Base {:?}", piece);
                    println!("");
                }
                _ => {
                    self.grid_pieces.insert(coord, piece);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn from(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}
