mod isometric_grid;
mod movement;
mod tokens;

use crate::{
    isometric_grid::{Coord, IGrid},
    movement::populate_tree,
    tokens::{Piece, *},
};

fn main() {
    let mut grid = IGrid::from(Coord::from(0, 0), Coord::from(20, 20));
    let mut grid2 = IGrid::new();
    grid2.add_piece(Piece::Base(Base::new()), Coord::from(1, 1));
    for i in 0..30 {
        for j in 0..30 {
            grid.add_piece(Piece::Empty, Coord::from(i, j));
        }
    }

    grid.add_piece(Piece::GoldPot(GoldPot::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Scout(Scout::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Empty, Coord::from(0, 0));
    grid.add_piece(Piece::Tank(Tank::new()), Coord::from(0, 0));
    let medic = Piece::Medic(Medic::new());
    let medic2 = medic.clone();
    grid.add_piece(medic, Coord::from(9, 9));
    grid.add_piece(Piece::Base(Base::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Wall(Wall::new()), Coord::from(10, 9));
    grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(8, 9));
    grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(9, 8));

    let tree = populate_tree(&Coord::from(9, 9), &medic2, &grid);
    println!("{:#?}", tree);
}
