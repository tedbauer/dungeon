use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::path::PathBuf;

use crate::map::Map;
use crate::map::TileType;
use crate::sdl2::image::LoadTexture;
use crate::ImageRender;
use crate::Player;
use crate::Position;
use crate::Render;

use engine::world::EntityId;
use engine::world::View;
use engine::world::World;

fn tile_to_world(x: i32, y: i32) -> (i32, i32) {
    ((x - y) * (40 / 2) + 400, (x + y) * (20 / 2))
}

pub fn map_render(map: &Map, canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();

    let floor_tex = texture_creator
        .load_texture(PathBuf::from("game/assets/floor.png"))
        .unwrap();
    let wall_tex = texture_creator
        .load_texture(PathBuf::from("game/assets/wall.png"))
        .unwrap();

    for (x, row) in map.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            let (x_world, y_world) = tile_to_world(x as i32, y as i32);
            match tile {
                | TileType::Wall => {
                    canvas.copy(&wall_tex, None, Rect::new(x_world, y_world, 40, 60))
                }
                | TileType::Floor => {
                    canvas.copy(&floor_tex, None, Rect::new(x_world, y_world + 40, 40, 20))
                }
            };
            //canvas.present();
            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    /*
    for x in (0..20).rev() {
        for y in 0..20 {
            let (x_world, y_world) = tile_to_world(x, y);
            if x == 10 {

            } else {

            }
        }
    }
        */
}
