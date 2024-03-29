extern crate engine;

use component_derive::Component;
use engine::component::Component;
use engine::view::view::RgbColor;
use std::any::Any;
use std::any::TypeId;

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn add(&mut self, other: &Position) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Component, Debug, Clone)]
pub struct Player {}

#[derive(Component, Debug, Clone)]
pub struct RandomWalk {}

#[derive(Component, Debug)]
pub struct Enemy {}

#[derive(Component, Debug)]
pub struct Render {
    pub color: RgbColor,
}

#[derive(Component)]
pub struct ImageRender {
    pub texture_index: usize,
    pub height: u32,
    pub width: u32,
    pub y_offset: i32,
}
