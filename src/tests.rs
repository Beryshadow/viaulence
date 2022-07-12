use uuid::Uuid;

use crate::{pieces::traits::Move, player::player::Player, *};

#[cfg(test)]
#[test]
fn test_player() {
    let mut player = Player::new();
    let piece = Piece::Scout(Scout::new(Uuid::new_v4()));
    player.add_piece(&piece);
    assert_eq!(player.pieces().len(), 1);
}
#[test]
fn test_grid() {
    let mut grid = IGrid::new();
    let piece = Piece::Scout(Scout::new(Uuid::new_v4()));
    grid.add_piece(piece.clone(), Coord::from(9, 9));
    let coords = grid.get_coord(&piece);
    assert_eq!(coords, Some(Coord::from(9, 9)));
}
#[test]
fn test_movement() {
    let mut grid = IGrid::from(Coord::from(0, 0), Coord::from(20, 20));

    // create a uuid for team 1 and 2
    let team_1 = Uuid::new_v4();
    let team_2 = Uuid::new_v4();

    grid.add_piece(Piece::GoldPot(GoldPot::new()), Coord::from(0, 0));
    grid.add_piece(Piece::Scout(Scout::new(team_1)), Coord::from(0, 0));
    grid.add_piece(Piece::Empty, Coord::from(0, 0));
    grid.add_piece(Piece::Tank(Tank::new(team_2)), Coord::from(0, 0));
    let medic = Piece::Medic(Medic::new(team_1));
    let base = Base::new(team_2);
    let mut scout = Scout::new(team_2);
    scout.set_coords(Coord::from(9, 9));
    grid.add_piece(medic, Coord::from(9, 9));
    grid.add_piece(Piece::Base(Base::new(team_2)), Coord::from(0, 0));
    // grid.add_piece(Piece::Wall(Wall::new()), Coord::from(10, 9));
    // grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(8, 9));
    // grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(9, 8));

    // println!("{grid:?}");

    let tree = populate_tree(&Coord::from(9, 9), scout, &grid);
    // println!("{:#?}", tree);
    let errored = tree.get_path_to_coord(&mut Coord::from(7, 6));

    let valid = tree.get_path_to_coord(&mut Coord::from(9, 10));
    println!("this is tree {:?}", tree);
    println!("this is valid: {:?}", valid);
    println!("{}", valid.is_ok());
    assert_eq!(errored.is_err(), true);
    assert_eq!(valid.is_ok(), true);
}
