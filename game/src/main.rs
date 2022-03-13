// goal for today: get all screen/drawing functionality behind "screen" trait; no drawing-related SDL imports
// in game, only engine screen-traits
// also if time: Load textures once, not on every frame
extern crate engine;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::EventPump;
use std::path::PathBuf;
use std::time::Duration;

mod components;
mod map;
mod map_builder;
mod systems;

use crate::sdl2::image::LoadTexture;
use crate::systems::entity_render::entity_render;
use crate::systems::player_input::process_player_input;
use components::*;
use engine::view::desktop_screen::DesktopScreen;
use engine::view::view::RgbColor;
use engine::view::view::Screen;
use engine::world::World;
use map::TileType;
use sdl2::render::Texture;
use sdl2::video::WindowContext;

fn start_game_loop<'a>(
    world: &mut World,
    mut screen: Box<dyn Screen>,
    mut event_pump: EventPump,
    textures: Vec<Texture<'a>>,
) {
    'running: loop {
        screen.set_draw_color(&RgbColor {
            red: 255,
            green: 255,
            blue: 255,
        });
        screen.clear();
        screen.set_draw_color(&RgbColor {
            red: 0,
            green: 0,
            blue: 0,
        });

        for event in event_pump.poll_iter() {
            match event {
                | Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                | Event::KeyDown {
                    keycode: Some(key), ..
                } => process_player_input(world, key),
                | _ => {}
            }
        }

        //map_render(&map, &mut canvas);
        entity_render(&world, &mut screen, &textures);
        screen.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn build_map(world: &mut World) {
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
                            texture_index: 0,
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
                            texture_index: 1,
                            width: 40,
                            height: 20,
                            y_offset: 40,
                        },
                    );
                }
            }
        }
    }
}

fn create_entities(world: &mut World) {
    let player = world.create_entity();
    world.assign(player, Position { x: 4, y: 4 });
    world.assign(player, Player {});
    world.assign(
        player,
        ImageRender {
            texture_index: 2,
            width: 40,
            height: 60,
            y_offset: 0,
        },
    );

    /*
    let enemy1 = world.create_entity();
    world.assign(enemy1, Position { x: 5, y: 10 });
    world.assign(enemy1, Enemy {});
    world.assign(
        enemy1,
        Render {
            color: RgbColor {
                red: 255,
                green: 0,
                blue: 0,
            },
        },
    );

    let enemy2 = world.create_entity();
    world.assign(enemy2, Position { x: 100, y: 40 });
    world.assign(enemy2, Enemy {});
    world.assign(
        enemy2,
        Render {
            color: RgbColor {
                red: 255,
                green: 0,
                blue: 0,
            },
        },
    );
        */
}

fn init_world() -> World {
    let mut world = World::new();
    world.create_pool::<Player>();
    world.create_pool::<Enemy>();
    world.create_pool::<Position>();
    world.create_pool::<Render>();
    world.create_pool::<ImageRender>();
    world
}

fn run_game(screen: Box<dyn Screen>) {
    let mut world: World = init_world();
    let texture_creator = screen.texture_creator();
    let wall_tex = texture_creator
        .load_texture(PathBuf::from("game/assets/wall.png"))
        .unwrap();

    let floor_tex = texture_creator
        .load_texture(PathBuf::from("game/assets/wall.png"))
        .unwrap();

    let person_tex = texture_creator
        .load_texture(PathBuf::from("game/assets/person.png"))
        .unwrap();

    let textures = vec![wall_tex, floor_tex, person_tex];

    create_entities(&mut world);
    build_map(&mut world);

    let event_pump = screen.get_context().event_pump().unwrap();
    start_game_loop(&mut world, screen, event_pump, textures)
}

pub fn main() {
    let screen = DesktopScreen::builder()
        .with_title("dungeon")
        .with_size((800, 600))
        .build()
        .unwrap();

    run_game(Box::new(screen));
}
