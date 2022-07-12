use std::io::Error;

use uuid::Uuid;

use crate::grid::isometric_grid::{Coord, IGrid};

use super::{
    movement::{in_range, not_blocked},
    traits::{Attack, Attackable, Move},
}; // will be used to categorise pieces

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Empty {}

/* LII this is all the pieces that can be placed on the grid
we need to implement the appropriate traits for each piece
*/

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scout {
    coord: Coord,
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
    base_under: bool,
    pot_under: bool,
    health: i8,
    attack: i8,
    range: i8,
    movement: i8,
    cost: i32,
}

// for example a scout can attack, move, cary gold, be placed, die

impl Scout {
    pub fn new(team_uuid: Uuid) -> Scout {
        Scout {
            coord: Coord::new(),
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: true,
            base_under: true,
            pot_under: false,
            health: 1,
            attack: 2,
            range: 3,
            movement: 4,
            cost: 300,
        }
    }
    pub fn set_coords(&mut self, coord: Coord) {
        self.coord = coord;
    }
}

impl Move for Scout {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        not_blocked(self, grid)
    }
}

impl Attack for Scout {
    fn attack(&self, attacked: &mut dyn Attackable, grid: &IGrid) -> Result<(), Error> {
        // we get the pieces in range and check the attacked piece is not immune and is not the same team and is in range
        Ok(())
    }
    fn can_attack(&self, grid: &IGrid) -> bool {
        // check if any pieces are in range and are not immune and are not the same team
        in_range(self, grid)
    }
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_range(&self) -> i8 {
        self.range
    }
    fn get_damage(&self) -> i8 {
        self.attack
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tank {
    coord: Coord,
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
    base_under: bool,
    pot_under: bool,
    health: i8,
    attack: i8,
    range: i8,
    movement: i8,
    cost: i32,
}

impl Tank {
    pub fn new(team_uuid: Uuid) -> Tank {
        Tank {
            coord: Coord::new(),
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: true,
            base_under: true,
            pot_under: false,
            health: 5,
            attack: 3,
            range: 8,
            movement: 2,
            cost: 1000,
        }
    }
}

impl Move for Tank {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        //LII this will need to be changed apropriately for the tank
        not_blocked(self, grid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Soldier {
    coord: Coord,
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
    base_under: bool,
    pot_under: bool,
    health: i8,
    attack: i8,
    range: i8,
    range_increase_cost: i32,
    max_range: i8,
    movement: i8,
    cost: i32,
}

impl Soldier {
    pub fn new(team_uuid: Uuid) -> Soldier {
        Soldier {
            coord: Coord::new(),
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: true,
            base_under: true,
            pot_under: false,
            health: 3,
            attack: 3,
            range: 5,
            range_increase_cost: 20,
            max_range: 15,
            movement: 3,
            cost: 400,
        }
    }
}

impl Move for Soldier {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        not_blocked(self, grid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Medic {
    coord: Coord,
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
    base_under: bool,
    pot_under: bool,
    health: i8,
    movement: i8,
    cost: i32,
}

impl Medic {
    pub fn new(team_uuid: Uuid) -> Medic {
        Medic {
            coord: Coord::new(),
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: true,
            base_under: true,
            pot_under: false,
            health: 1,
            movement: 5,
            cost: 2000,
        }
    }
}

impl Move for Medic {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        not_blocked(self, grid)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wall {
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
    health: i8,
}

impl Wall {
    pub fn new(team_uuid: Uuid) -> Wall {
        Wall {
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: false,
            health: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Base {
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
}

impl Base {
    pub fn new(team_uuid: Uuid) -> Base {
        Base {
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GoldPot {
    uuid: uuid::Uuid,
    immune: bool,
}

impl GoldPot {
    pub fn new() -> GoldPot {
        GoldPot {
            uuid: Uuid::new_v4(),
            immune: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Piece {
    Empty,
    Scout(Scout),
    Tank(Tank),
    Soldier(Soldier),
    Medic(Medic),
    Wall(Wall),
    Base(Base),
    GoldPot(GoldPot),
}

impl AsMut<Piece> for Piece {
    fn as_mut(&mut self) -> &mut Piece {
        self
    }
}

impl Piece {
    pub fn change_pot_state(&mut self, state: bool) -> &mut Piece {
        match self {
            Piece::Scout(scout) => scout.pot_under = state,
            Piece::Tank(tank) => tank.pot_under = state,
            Piece::Soldier(soldier) => soldier.pot_under = state,
            Piece::Medic(medic) => medic.pot_under = state,
            _ => {}
        }
        self
    }
    pub fn change_base_state(&mut self, state: bool) -> &mut Piece {
        match self {
            Piece::Scout(scout) => scout.base_under = state,
            Piece::Tank(tank) => tank.base_under = state,
            Piece::Soldier(soldier) => soldier.base_under = state,
            Piece::Medic(medic) => medic.base_under = state,
            _ => {}
        }
        self
    }
    pub fn change_immune_state(&mut self, state: bool) -> &mut Piece {
        match self {
            Piece::Scout(scout) => scout.immune = state,
            Piece::Tank(tank) => tank.immune = state,
            Piece::Soldier(soldier) => soldier.immune = state,
            Piece::Medic(medic) => medic.immune = state,
            Piece::Wall(wall) => wall.immune = state,
            Piece::Base(base) => base.immune = state,
            Piece::GoldPot(gold_pot) => gold_pot.immune = state,
            _ => {}
        }
        self
    }
    pub fn change_health(&mut self, health: i8) -> &mut Piece {
        match self {
            Piece::Scout(scout) => scout.health = health,
            Piece::Tank(tank) => tank.health = health,
            Piece::Soldier(soldier) => soldier.health = health,
            Piece::Medic(medic) => medic.health = health,
            Piece::Wall(wall) => wall.health = health,
            _ => {}
        }
        self
    }
    pub fn get_health(&self) -> Option<i8> {
        match self {
            Piece::Scout(scout) => Some(scout.health),
            Piece::Tank(tank) => Some(tank.health),
            Piece::Soldier(soldier) => Some(soldier.health),
            Piece::Medic(medic) => Some(medic.health),
            Piece::Wall(wall) => Some(wall.health),
            _ => None,
        }
    }
    pub fn get_uuid(&self) -> Option<uuid::Uuid> {
        match self {
            Piece::Scout(scout) => Some(scout.uuid),
            Piece::Tank(tank) => Some(tank.uuid),
            Piece::Soldier(soldier) => Some(soldier.uuid),
            Piece::Medic(medic) => Some(medic.uuid),
            Piece::Wall(wall) => Some(wall.uuid),
            Piece::Base(base) => Some(base.uuid),
            Piece::GoldPot(gold_pot) => Some(gold_pot.uuid),
            _ => None,
        }
    }
    pub fn get_moves(&self) -> Option<i8> {
        match self {
            Piece::Scout(scout) => Some(scout.movement),
            Piece::Tank(tank) => Some(tank.movement),
            Piece::Soldier(soldier) => Some(soldier.movement),
            Piece::Medic(medic) => Some(medic.movement),
            _ => None,
        }
    }
    pub fn get_cost(&self) -> Option<i32> {
        match self {
            Piece::Scout(scout) => Some(scout.cost),
            Piece::Tank(tank) => Some(tank.cost),
            Piece::Soldier(soldier) => Some(soldier.cost),
            Piece::Medic(medic) => Some(medic.cost),
            _ => None,
        }
    }
    pub fn movable(&self) -> bool {
        match self {
            Piece::Scout(scout) => true,
            Piece::Tank(tank) => true,
            Piece::Soldier(soldier) => true,
            Piece::Medic(medic) => true,
            Piece::Wall(wall) => false,
            Piece::Base(base) => false,
            Piece::GoldPot(gold_pot) => false,
            _ => false,
        }
    }
    pub fn can_move(&self, grid: &IGrid) -> bool {
        // match all pieces that implement Move and use the not_blocked method to check if they can move
        match self {
            Piece::Scout(scout) => scout.not_blocked(grid),
            Piece::Tank(tank) => tank.not_blocked(grid),
            Piece::Soldier(soldier) => soldier.not_blocked(grid),
            Piece::Medic(medic) => medic.not_blocked(grid),
            _ => false,
        }
    }
    pub fn attacking(&self, grid: &IGrid) -> bool {
        match self {
            Piece::Scout(scout) => true,
            Piece::Tank(tank) => true,
            Piece::Soldier(soldier) => true,
            Piece::Medic(medic) => false,
            Piece::Wall(wall) => false,
            Piece::Base(base) => false,
            Piece::GoldPot(gold_pot) => false,
            _ => false,
        }
    }
    pub fn can_attack(&self, grid: &IGrid) -> bool {
        match self {
            Piece::Scout(scout) => scout.can_attack(grid),
            Piece::Tank(tank) => tank.can_attack(grid),
            Piece::Soldier(soldier) => soldier.can_attack(grid),
            _ => false,
        }
    }
}
