// use crate::pieces::movement::populate_tree;
// use crate::pieces::tokens::Piece;
// use crate::pieces::tokens::*;
use grid::isometric_grid::{Coord, IGrid};

use crate::{
    game::gamestate::start_new_game,
    pieces::{movement::*, tokens::*},
};

mod game;
mod grid;
mod pieces;
mod player;
mod tests;

fn main() {
    // let mut grid = IGrid::from(Coord::from(0, 0), Coord::from(20, 20));

    // grid.add_piece(Piece::GoldPot(GoldPot::new()), Coord::from(0, 0));
    // grid.add_piece(Piece::Scout(Scout::new()), Coord::from(0, 0));
    // grid.add_piece(Piece::Empty, Coord::from(0, 0));
    // grid.add_piece(Piece::Tank(Tank::new()), Coord::from(0, 0));
    // let medic = Piece::Medic(Medic::new());
    // let base = Base::new();
    // let scout = Scout::new();
    // grid.add_piece(medic, Coord::from(9, 9));
    // grid.add_piece(Piece::Base(Base::new()), Coord::from(0, 0));
    // // grid.add_piece(Piece::Wall(Wall::new()), Coord::from(10, 9));
    // // grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(8, 9));
    // // grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(9, 8));

    // println!("{grid:?}");

    // let tree = populate_tree(&Coord::from(9, 9), scout, &grid);
    // // println!("{:#?}", tree);
    // let bob = tree.get_path_to_coord(&mut Coord::from(7, 6));
    // if let Ok(_) = bob {
    //     println!("");
    //     println!("");
    //     println!("bob {:#?}", bob);
    // } else {
    //     println!("cant go there");
    // }
    // // soldier.attack();
}
