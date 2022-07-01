use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Empty {}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Base {
    uuid: uuid::Uuid,
    immune: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wall {
    uuid: uuid::Uuid,
    immune: bool,
    health: i8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GoldPot {
    uuid: uuid::Uuid,
    immune: bool,
}

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

impl Wall {
    pub fn new() -> Wall {
        Wall {
            uuid: Uuid::new_v4(),
            immune: false,
            health: 4,
        }
    }
}

impl Base {
    pub fn new() -> Base {
        Base {
            uuid: Uuid::new_v4(),
            immune: true,
        }
    }
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
