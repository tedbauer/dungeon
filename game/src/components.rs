extern crate engine;

use component_derive::Component;
use engine::component::Component;
use std::any::Any;
use std::any::TypeId;

#[derive(Component, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Clone)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Enemy {}

#[derive(Component, Debug)]
pub struct Render {}
