extern crate engine;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

mod components;
use crate::component::Component;
use components::*;
use engine::component;
use engine::world::View;
use engine::world::World;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("dungeon", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let width = 400;
    let height = 400;

    let white = Color::RGB(255, 255, 255);
    let black = Color::RGB(0, 0, 0);

    canvas.set_draw_color(black);

    let mut world = World::new();
    world.add_entity(vec![
        Box::new(components::Player {}),
        Box::new(components::Position { x: 5, y: 9 }),
    ]);

    world.add_entity(vec![
        Box::new(components::Enemy {}),
        Box::new(components::Position { x: 10, y: 20 }),
    ]);

    world.add_entity(vec![
        Box::new(components::Enemy {}),
        Box::new(components::Position { x: 500, y: 1230 }),
    ]);

    /*
    world.add_entity(vec![Box::new(components::Enemy {}), Box::new(components::Position { x: 11, y: 11 })]);
    world.add_entity(vec![Box::new(components::Enemy {}), Box::new(components::Position { x: 14, y: 14 })]);
    world.add_entity(vec![Box::new(components::Enemy {}), Box::new(components::Position { x: 15, y: 15 })]);
    world.add_entity(vec![Box::new(components::Enemy {}), Box::new(components::Position { x: 2, y: 6 })]);
    world.add_entity(vec![Box::new(components::Enemy {}), Box::new(components::Position { x: 3, y: 1 })]);
    world.add_entity(vec![
        Box::new(components::Enemy {}),
        Box::new(components::Position { x: 123, y: 124 }),
    ]);
    world.add_entity(vec![
        Box::new(components::Enemy {}),
        Box::new(components::Position { x: 200, y: 400 }),
    ]);
    world.add_entity(vec![
        Box::new(components::Enemy {}),
        Box::new(components::Position { x: 321, y: 541 }),
    ]);
    world.add_entity(vec![
        Box::new(components::Enemy {}),
        Box::new(components::Position { x: 213, y: 123 }),
    ]);
    */

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //for (_, position) in View::<(Player, Position)>::new(&world) {
            //println!("player pos: {:?}", position);
        //}

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
