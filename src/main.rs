mod isometric_grid;
mod movement;
mod tokens;

use crate::{
    isometric_grid::{Coord, IGrid},
    movement::populate_tree,
    tokens::{Piece, *},
};

fn main() {
    let mut grid = IGrid::new(10, 20);

    // for i in 0..30 {
    //     for j in 0..30 {
    //         grid.add_piece(Piece::Empty, Coord::from(i, j));
    //     }
    // }

    grid.add_piece(Piece::GoldPot(GoldPot::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Scout(Scout::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Empty, Coord::from(0, 0));
    grid.add_piece(Piece::Tank(Tank::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(0, 0));
    let medic = Piece::Medic(Medic::new());
    let medic2 = medic.clone();
    grid.add_piece(medic, Coord::from(9, 9));
    grid.add_piece(Piece::Base(Base::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Wall(Wall::new()), Coord::from(0, 0));

    let tree = populate_tree(&Coord::from(9, 9), &medic2, &grid);

    let falsed = grid.is_valid(Coord::from(9, 9));

    // println!("{:#?}", tree);
}
