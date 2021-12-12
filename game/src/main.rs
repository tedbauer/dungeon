extern crate engine;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::path::PathBuf;
use std::time::Duration;

mod components;
mod map;
mod map_builder;
mod systems;

use crate::component::Component;
use crate::map_builder::create_map;
use crate::systems::entity_render::entity_render;
use crate::systems::map_render::map_render;
use crate::systems::player_input::process_player_input;
use components::*;
use engine::component;
use engine::world::EntityId;
use engine::world::View;
use engine::world::World;
use map::TileType;
use std::env;
use std::path::Path;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("dungeon", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let width = 400;
    let height = 400;

    let white = Color::RGB(255, 255, 255);
    let black = Color::RGB(0, 0, 0);

    canvas.set_draw_color(black);

    let mut world = World::new();
    world.create_pool::<Player>();
    world.create_pool::<Enemy>();
    world.create_pool::<Position>();
    world.create_pool::<Render>();
    world.create_pool::<ImageRender>();

    let player = world.create_entity();
    world.assign(player, Position { x: 4, y: 4 });
    world.assign(player, Player {});
    world.assign(
        player,
        Render {
            color: Color::RGB(255, 255, 255),
        },
    );
    world.assign(
        player,
        ImageRender {
            texture: PathBuf::from("game/assets/person.png"),
            width: 40,
            height: 60,
            y_offset: 0,
        },
    );

    let enemy1 = world.create_entity();
    world.assign(enemy1, Position { x: 5, y: 10 });
    world.assign(enemy1, Enemy {});
    world.assign(
        enemy1,
        Render {
            color: Color::RGB(255, 0, 0),
        },
    );

    let enemy2 = world.create_entity();
    world.assign(enemy2, Position { x: 100, y: 40 });
    world.assign(enemy2, Enemy {});
    world.assign(
        enemy2,
        Render {
            color: Color::RGB(255, 0, 0),
        },
    );

    let map = map_builder::create_map();

    for (x, row) in map.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            let t = world.create_entity();
            world.assign(
                t,
                Position {
                    x: x as i32,
                    y: y as i32,
                },
            );
            match tile {
                | TileType::Wall => {
                    world.assign(
                        t,
                        ImageRender {
                            texture: PathBuf::from("game/assets/wall.png"),
                            width: 40,
                            height: 60,
                            y_offset: 0,
                        },
                    );
                }
                | TileType::Floor => {
                    world.assign(
                        t,
                        ImageRender {
                            texture: PathBuf::from("game/assets/floor.png"),
                            width: 40,
                            height: 20,
                            y_offset: 40,
                        },
                    );
                }
            }
        }
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(white);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                | Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                | Event::KeyDown {
                    keycode: Some(key), ..
                } => process_player_input(&mut world, key),
                | _ => {}
            }
        }

        //map_render(&map, &mut canvas);
        entity_render(&world, &mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
