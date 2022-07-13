use uuid::Uuid;

use crate::{
    pieces::{traits::Move, tree::populate_tree},
    player::player::Player,
    *,
};

#[cfg(test)]
#[test]
fn test_player() {
    let mut player = Player::new();
    let piece = Scout::new(Uuid::new_v4());
    player.add_piece(&piece);
    assert_eq!(player.pieces().len(), 1);
}
#[test]
fn test_grid() {
    let mut grid = IGrid::new();
    let piece = Scout::new(Uuid::new_v4());
    grid.add_piece(piece.clone());
    let coords = grid.get_coord(&piece);
    assert_eq!(coords, Some(Coord::from(9, 9)));
}
#[test]
fn test_tree() {
    let mut grid = IGrid::from(Coord::from(0, 0), Coord::from(20, 20));

    // create a uuid for team 1 and 2
    let team_1 = Uuid::new_v4();
    let team_2 = Uuid::new_v4();

    grid.add_piece(GoldPot::new(Coord::from(0, 0)));
    grid.add_piece(Scout::new(team_1));
    grid.add_piece(Tank::new(team_2));
    let medic = Medic::new(team_1);
    let base = Base::new(team_2, Coord::from(0, 0));
    let mut scout = Scout::new(team_2);
    scout.set_coords(Coord::from(9, 9));
    grid.add_piece(medic);
    grid.add_piece(Base::new(team_2, Coord::from(9, 9)));
    // grid.add_piece(Piece::Wall(Wall::new()), Coord::from(10, 9));
    // grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(8, 9));
    // grid.add_piece(Piece::Soldier(Soldier::new()), Coord::from(9, 8));

    // println!("{grid:?}");

    let tree = populate_tree(scout, &grid, 3);
    // println!("{:#?}", tree);
    let errored = tree.get_path_to_coord(&mut Coord::from(7, 6));

    let valid = tree.get_path_to_coord(&mut Coord::from(9, 10));
    println!("this is tree {:?}", tree);
    println!("this is valid: {:?}", valid);
    println!("{}", valid.is_ok());
    assert_eq!(errored.is_err(), true);
    assert_eq!(valid.is_ok(), true);
}

#[test]
fn test_movement() {
    let mut scout1 = Scout::new(Uuid::new_v4());
    scout1.set_coords(Coord::from(9, 9));
    let mut scout2 = Scout::new(Uuid::new_v4());
    scout2.set_coords(Coord::from(9, 9));
    let mut scout3 = Scout::new(Uuid::new_v4());
    scout3.set_coords(Coord::from(9, 9));

    let mut grid = IGrid::from(Coord::from(0, 0), Coord::from(20, 20));
    grid.add_piece(scout1);
    grid.add_piece(scout2);
    grid.add_piece(scout3);

    let mut scout4 = Scout::new(Uuid::new_v4());
    scout4.set_coords(Coord::from(9, 9));
}
