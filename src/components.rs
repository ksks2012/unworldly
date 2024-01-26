use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

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
pub struct Name {
    pub name : String
}

#[derive(Component, Debug)]
pub struct Biotechnology {}

#[derive(Component, Debug)]
pub struct BiotechnologyState {
    pub lifespan : i32,
    pub life : i32,
    pub level : i32,
    pub max_exp : i32,
    pub exp : i32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ExpState {
    Training,
    Ready,
}

#[derive(Component, Debug)]
pub struct ExpClock {
    pub state : ExpState,
    pub duration : i32
}
