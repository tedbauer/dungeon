use crate::sdl2::image::LoadTexture;
use crate::ImageRender;
use crate::Player;
use crate::Position;
use crate::Render;
use engine::world::EntityId;
use engine::world::View;
use engine::world::World;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp::Ordering;
use std::path::PathBuf;

fn tile_to_world(x: i32, y: i32) -> (i32, i32) {
    ((x - y) * (40 / 2) + 400, (x + y) * (20 / 2))
}

pub fn entity_render(world: &World, canvas: &mut Canvas<Window>) {
    for entity in View::<(Position, Render)>::new(world) {
        let position: &Position = world.get_component::<Position>(entity).unwrap();
        let render: &Render = world.get_component::<Render>(entity).unwrap();

        canvas.set_draw_color(render.color);
        canvas.draw_rect(Rect::new(position.x, position.y, 50, 50));
    }

    let mut image_entities = View::<(Position, ImageRender)>::new(world).collect::<Vec<EntityId>>();
    image_entities.sort_by(|e1, e2| {
        match (
            world.get_component::<Position>(*e1),
            world.get_component::<Position>(*e2),
        ) {
            | (Some(p1), Some(p2)) => {
                if p1.x == p2.x && p1.y == p2.y {
                    Ordering::Equal
                } else if p1.x >= p2.x && p1.y >= p2.y {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }

            | _ => Ordering::Equal,
        }
    });

    let texture_creator = canvas.texture_creator();
    for entity in image_entities.iter() {
        match (
            world.get_component::<Position>(*entity),
            world.get_component::<ImageRender>(*entity),
        ) {
            | (Some(position), Some(render)) => {
                let tex = texture_creator
                    .load_texture(render.texture.clone())
                    .unwrap();

                let (x, y) = tile_to_world(position.x, position.y);
                canvas.copy(
                    &tex,
                    None,
                    Rect::new(x, y + render.y_offset, render.width, render.height),
                );
            }
            | _ => (),
        }
        //let position: &Position = world.get_component::<Position>(entity).unwrap();
        //let render: &ImageRender = world.get_component::<ImageRender>(entity).unwrap();
    }
}
