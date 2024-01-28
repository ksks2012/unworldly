use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, ConvertSaveload, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Biotechnology {}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct BiotechnologyState {
    pub lifespan: i32,
    pub life: i32,
    pub level: i32,
    pub max_exp: i32,
    pub exp: i32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum ExpState {
    Training,
    Ready,
    Bottleneck,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct ExpClock {
    pub state: ExpState,
    pub duration: i32,
}

pub struct SerializeMe;
