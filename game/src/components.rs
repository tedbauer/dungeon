extern crate engine;

use component_derive::Component;
use engine::component::Component;
use std::any::Any;

#[derive(Component, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Enemy {}

#[derive(Component, Debug)]
pub struct Render {}
