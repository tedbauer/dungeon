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
use engine::world::EntityId;
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
    world.create_pool::<Player>();
    world.create_pool::<Enemy>();
    world.create_pool::<Position>();

    let player = world.create_entity();
    world.assign(player, Position { x: 5, y: 9 });
    world.assign(player, Player {});

    let enemy1 = world.create_entity();
    world.assign(enemy1, Position { x: 100, y: 200 });
    world.assign(enemy1, Enemy {});

    let enemy2 = world.create_entity();
    world.assign(enemy2, Position { x: 50, y: 40 });
    world.assign(enemy2, Enemy {});

    /*
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
        */

    //println!("{:?}", world);

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

        for entity in View::<(Player, Position)>::new(&mut world).collect::<Vec<EntityId>>() {
            let player: &Player = world.get_component::<Player>(entity).unwrap();
            println!("player: {:?}", player);

            let pos: &mut Position = world.get_component_mut::<Position>(entity).unwrap();
            println!("player pos: {:?}", pos);
            pos.x += 1;
        }

        for entity in View::<(Enemy, Position)>::new(&mut world).collect::<Vec<EntityId>>() {
            let enemy: &Enemy = world.get_component::<Enemy>(entity).clone().unwrap();
            println!("enemy: {:?}", enemy);

            let pos: &mut Position = world.get_component_mut::<Position>(entity).unwrap();
            println!("enemy pos: {:?}", pos);
            pos.x += 1;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
