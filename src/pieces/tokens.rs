use std::io::Error;

use uuid::Uuid;

use crate::grid::isometric_grid::{Coord, IGrid};

use super::{
    // attack::in_range,
    movement::not_blocked,
    traits::{Attack, Attackable, Move, Piece},
}; // will be used to categorise pieces

/* LII this is all the pieces that can be placed on the grid
we need to implement the appropriate traits for each piece
*/

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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

impl Piece for Scout {
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn change_immune_state(&mut self, immune: bool) {
        self.immune = immune;
    }
    fn is_immune(&self) -> bool {
        self.immune
    }
    fn get_on_base(&self) -> bool {
        self.base_under
    }
    fn set_on_base(&mut self, on_base: bool) {
        self.base_under = on_base;
    }
    fn get_on_pot(&self) -> bool {
        self.pot_under
    }
    fn set_on_pot(&mut self, on_pot: bool) {
        self.pot_under = on_pot;
    }
    fn can_host_piece(&self) -> bool {
        false
    }
    fn get_name(&self) -> &str {
        "Scout"
    }
    fn movable(&self) -> bool {
        true
    }
    fn can_attack(&self) -> bool {
        true
    }
    fn can_be_attacked(&self) -> bool {
        true
    }
}

impl Move for Scout {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        not_blocked(self, grid)
    }
}

impl Attackable for Scout {
    fn can_be_attacked(&self) -> bool {
        if self.immune {
            return false;
        }
        true
    }
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
}

impl Attack for Scout {
    fn attack(&self, attacked: &mut dyn Attackable, grid: &IGrid) -> Result<(), Error> {
        // we get the pieces in range and check the attacked piece is not immune and is not the same team and is in range
        Ok(())
    }
    fn can_attack(&self, grid: &IGrid) -> bool {
        //LMKL check if any pieces are in range and are not immune and are not the same team
        // let joe = in_range(self, grid);
        // if joe.is_empty() {
        //     return false;
        // } else {
        //     return true;
        // }
        true
    }
    fn get_range(&self) -> i8 {
        self.range
    }
    fn get_damage(&self) -> i8 {
        self.attack
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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

impl Piece for Tank {
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn change_immune_state(&mut self, immune: bool) {
        self.immune = immune;
    }
    fn is_immune(&self) -> bool {
        self.immune
    }
    fn get_on_base(&self) -> bool {
        self.base_under
    }
    fn set_on_base(&mut self, on_base: bool) {
        self.base_under = on_base;
    }
    fn get_on_pot(&self) -> bool {
        self.pot_under
    }
    fn set_on_pot(&mut self, on_pot: bool) {
        self.pot_under = on_pot;
    }
    fn can_host_piece(&self) -> bool {
        false
    }
    fn get_name(&self) -> &str {
        "Tank"
    }
    fn movable(&self) -> bool {
        true
    }
    fn can_attack(&self) -> bool {
        true
    }
    fn can_be_attacked(&self) -> bool {
        true
    }
}

impl Move for Tank {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        //LII this will need to be changed apropriately for the tank
        not_blocked(self, grid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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

impl Piece for Soldier {
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn change_immune_state(&mut self, immune: bool) {
        self.immune = immune;
    }
    fn is_immune(&self) -> bool {
        self.immune
    }
    fn get_on_base(&self) -> bool {
        self.base_under
    }
    fn set_on_base(&mut self, on_base: bool) {
        self.base_under = on_base;
    }
    fn get_on_pot(&self) -> bool {
        self.pot_under
    }
    fn set_on_pot(&mut self, on_pot: bool) {
        self.pot_under = on_pot;
    }
    fn can_host_piece(&self) -> bool {
        false
    }
    fn get_name(&self) -> &str {
        "Soldier"
    }
    fn movable(&self) -> bool {
        true
    }
    fn can_attack(&self) -> bool {
        true
    }
    fn can_be_attacked(&self) -> bool {
        true
    }
}

impl Move for Soldier {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        not_blocked(self, grid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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

impl Piece for Medic {
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn change_immune_state(&mut self, immune: bool) {
        self.immune = immune;
    }
    fn is_immune(&self) -> bool {
        self.immune
    }
    fn get_on_base(&self) -> bool {
        self.base_under
    }
    fn set_on_base(&mut self, on_base: bool) {
        self.base_under = on_base;
    }
    fn get_on_pot(&self) -> bool {
        self.pot_under
    }
    fn set_on_pot(&mut self, on_pot: bool) {
        self.pot_under = on_pot;
    }
    fn can_host_piece(&self) -> bool {
        false
    }
    fn get_name(&self) -> &str {
        "Medic"
    }
    fn movable(&self) -> bool {
        true
    }
    fn can_attack(&self) -> bool {
        false
    }
    fn can_be_attacked(&self) -> bool {
        true
    }
}

impl Move for Medic {
    fn move_(&self, grid: &IGrid, coord: Coord) -> Result<(), Error> {
        unimplemented!();
    }
    fn get_moves(&self) -> Option<i8> {
        Some(self.movement)
    }
    fn not_blocked(&self, grid: &IGrid) -> bool {
        not_blocked(self, grid)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Wall {
    coord: Coord,
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
    health: i8,
}

impl Wall {
    pub fn new(team_uuid: Uuid, coord: Coord) -> Wall {
        Wall {
            coord: coord,
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: false,
            health: 4,
        }
    }
}

impl Piece for Wall {
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn change_immune_state(&mut self, immune: bool) {
        self.immune = immune;
    }
    fn is_immune(&self) -> bool {
        self.immune
    }
    fn get_on_base(&self) -> bool {
        false
    }
    fn set_on_base(&mut self, on_base: bool) {}
    fn get_on_pot(&self) -> bool {
        false
    }
    fn set_on_pot(&mut self, on_pot: bool) {}
    fn can_host_piece(&self) -> bool {
        false
    }
    fn get_name(&self) -> &str {
        "Wall"
    }
    fn movable(&self) -> bool {
        false
    }
    fn can_attack(&self) -> bool {
        false
    }
    fn can_be_attacked(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Base {
    coord: Coord,
    uuid: uuid::Uuid,
    team: uuid::Uuid,
    immune: bool,
}

impl Base {
    pub fn new(team_uuid: Uuid, coord: Coord) -> Base {
        Base {
            coord: coord,
            uuid: Uuid::new_v4(),
            team: team_uuid,
            immune: true,
        }
    }
}

impl Piece for Base {
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn change_immune_state(&mut self, immune: bool) {
        self.immune = immune;
    }
    fn is_immune(&self) -> bool {
        self.immune
    }
    fn get_on_base(&self) -> bool {
        false
    }
    fn set_on_base(&mut self, on_base: bool) {}
    fn get_on_pot(&self) -> bool {
        false
    }
    fn set_on_pot(&mut self, on_pot: bool) {}
    fn can_host_piece(&self) -> bool {
        true
    }
    fn get_name(&self) -> &str {
        "Base"
    }
    fn movable(&self) -> bool {
        false
    }
    fn can_attack(&self) -> bool {
        false
    }
    fn can_be_attacked(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct GoldPot {
    coord: Coord,
    uuid: uuid::Uuid,
    immune: bool,
}

impl GoldPot {
    pub fn new(coord: Coord) -> GoldPot {
        GoldPot {
            coord: coord,
            uuid: Uuid::new_v4(),
            immune: true,
        }
    }
}

impl Piece for GoldPot {
    fn get_coord(&self) -> &Coord {
        &self.coord
    }
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn change_immune_state(&mut self, immune: bool) {
        self.immune = immune;
    }
    fn is_immune(&self) -> bool {
        self.immune
    }
    fn get_on_base(&self) -> bool {
        false
    }
    fn set_on_base(&mut self, on_base: bool) {}
    fn get_on_pot(&self) -> bool {
        false
    }
    fn set_on_pot(&mut self, on_pot: bool) {}
    fn can_host_piece(&self) -> bool {
        true
    }
    fn get_name(&self) -> &str {
        "GoldPot"
    }
    fn movable(&self) -> bool {
        false
    }
    fn can_attack(&self) -> bool {
        false
    }
    fn can_be_attacked(&self) -> bool {
        false
    }
}
