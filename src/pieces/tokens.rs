use std::io::Error;

use uuid::Uuid;

use crate::grid::isometric_grid::{Coord, IGrid};

use super::traits::*; // will be used to categorise pieces

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Empty {}

/* LII this is all the pieces that can be placed on the grid
we need to implement the appropriate traits for each piece
and get rid of the Piece enum */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scout {
    uuid: uuid::Uuid,
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
    pub fn new() -> Scout {
        Scout {
            uuid: Uuid::new_v4(),
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
}

impl Move for Scout {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tank {
    uuid: uuid::Uuid,
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
    pub fn new() -> Tank {
        Tank {
            uuid: Uuid::new_v4(),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Soldier {
    uuid: uuid::Uuid,
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
    pub fn new() -> Soldier {
        Soldier {
            uuid: Uuid::new_v4(),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Medic {
    uuid: uuid::Uuid,
    immune: bool,
    base_under: bool,
    pot_under: bool,
    health: i8,
    movement: i8,
    cost: i32,
}

impl Medic {
    pub fn new() -> Medic {
        Medic {
            uuid: Uuid::new_v4(),
            immune: true,
            base_under: true,
            pot_under: false,
            health: 1,
            movement: 5,
            cost: 2000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Base {
    uuid: uuid::Uuid,
    immune: bool,
}

impl Base {
    pub fn new() -> Base {
        Base {
            uuid: Uuid::new_v4(),
            immune: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wall {
    uuid: uuid::Uuid,
    immune: bool,
    health: i8,
}

impl Wall {
    pub fn new() -> Wall {
        Wall {
            uuid: Uuid::new_v4(),
            immune: false,
            health: 4,
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
}
