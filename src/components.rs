use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use rltk::{RGB};

pub enum MealType {
    Breakfast, Lunch, Dinner
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Name {
    pub name: String
}

#[derive(Component, Debug)]
pub struct PlayerStats {
    pub total_money: i32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct GameClock {
    pub duration: i32
}

#[derive(Component, Debug, Clone)]
pub struct SeedClock {
    pub duration: i32
}

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug, Clone)]
pub struct Seed {
    pub value : i32,
    pub time_to_maturation: i32,
    pub lifespan: i32,
    pub name : String
}

#[derive(Component, Debug, Clone)]
pub struct SownSeed {
    pub value : i32,
    pub time_to_maturation: i32,
    pub lifespan: i32,
    pub name : String
}

#[derive(Component, Debug, Clone)]
pub struct DayPlan {
    pub breakfast : bool,
    pub lunch : bool,
    pub dinner : bool
}

#[derive(Component, Debug, Clone)]
pub struct Meal {
    pub meal_type : i32
}

#[derive(Component, Debug, Clone)]
pub struct WantsToSowSeed {
    pub seed : Entity,
    pub x1 : i32,
    pub y1 : i32
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct InPlayerInventory {
}
