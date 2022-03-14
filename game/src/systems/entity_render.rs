use crate::ImageRender;
use crate::Position;
use crate::Render;
use engine::view::view::Screen;
use engine::world::EntityId;
use engine::world::View;
use engine::world::World;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::cmp::Ordering;

fn tile_to_world(x: i32, y: i32) -> (i32, i32) {
    ((x - y) * (40 / 2) + 400, (x + y) * (20 / 2))
}

pub fn entity_render<'a>(world: &World, screen: &mut Box<dyn Screen>, textures: &Vec<Texture<'a>>) {
    let image_entities = View::<(Position, ImageRender)>::new(world).collect::<Vec<EntityId>>();

    let mut image_entities_sorted = image_entities.clone();
    image_entities_sorted.sort_by(|e1, e2| {
        match (
            world.get_component::<Position>(*e1),
            world.get_component::<Position>(*e2),
        ) {
            | (Some(p1), Some(p2)) => {
                if p1.x == p2.x && p1.y == p2.y {
                    Ordering::Equal
                } else if p1.x > p2.x || (p1.x == p2.x && p1.y > p2.y) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            | (None, None) => {
                println!("None, None for e1 {} and e2 {}", e1, e2);
                Ordering::Equal
            }
            | (Some(a), None) => {
                println!("Some({:?}), None for e1 {} and e2 {}", a, e1, e2);
                Ordering::Greater
            }
            | (None, Some(b)) => {
                println!("None, Some({:?}) for e1 {} and e2 {}", b, e1, e2);
                Ordering::Equal
            }
        }
    });

    for entity in image_entities_sorted.iter() {
        match (
            world.get_component::<Position>(*entity),
            world.get_component::<ImageRender>(*entity),
        ) {
            | (Some(position), Some(render)) => {
                let (x, y) = tile_to_world(position.x, position.y);
                screen.copy(
                    &textures.get(render.texture_index).unwrap(),
                    None,
                    Some(Rect::new(
                        x,
                        y + render.y_offset,
                        render.width,
                        render.height,
                    )),
                );
            }
            | _ => (),
        }
    }
}
