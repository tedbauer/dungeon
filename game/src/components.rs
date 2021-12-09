use sdl2::pixels::Color;

extern crate engine;

use component_derive::Component;
use engine::component::Component;
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

#[derive(Component, Debug)]
pub struct Enemy {}

#[derive(Component, Debug)]
pub struct Render {
    pub color: Color,
}
