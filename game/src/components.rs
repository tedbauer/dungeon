extern crate engine;

use crate::util::Point;
use std::any::Any;
use engine::component::Component;
use component_derive::Component;

#[derive(Component)]
pub struct Position {
    pub pos: Point,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Render {}
