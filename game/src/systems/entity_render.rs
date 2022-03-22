use crate::ImageRender;
use crate::Player;
use crate::Position;
use crate::Render;
use engine::view::view::Screen;
use engine::world::EntityId;
use engine::world::View;
use engine::world::World;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::cmp::Ordering;

pub struct LerpingState {
    a: (f64, f64),
    b: (f64, f64),
    t: f64,
}

pub enum LerpState {
    Lerping(LerpingState),
    Paused((f64, f64)),
}

pub struct Renderer {
    pub camera_x: f64,
    pub camera_y: f64,
    pub lerp_state: LerpState,
}

fn tile_to_world(x: i32, y: i32) -> (i32, i32) {
    ((x - y) * (40 / 2) + 400, (x + y) * (20 / 2))
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t * 1.5
}

impl Renderer {
    pub fn tick<'a>(
        &mut self,
        world: &World,
        screen: &mut Box<dyn Screen>,
        textures: &Vec<Texture<'a>>,
        delta: f64,
    ) {
        for entity in View::<(Player, Position)>::new(world) {
            if let Some(pos) = world.get_component::<Position>(entity).unwrap() {
                let coords = tile_to_world(pos.x, pos.y);
                match self.lerp_state {
                    | LerpState::Paused((x, y)) => {
                        self.lerp_state = LerpState::Lerping(LerpingState {
                            a: (x, y),
                            b: ((-1 * coords.0 + 350) as f64, (-1 * coords.1 + 250) as f64),
                            t: 0.0,
                        })
                    }
                    | LerpState::Lerping(LerpingState { a, b, t }) => {
                        let new_t = t + delta;
                        if new_t >= 1.0 {
                            self.lerp_state = LerpState::Paused((self.camera_x, self.camera_y));
                        } else {
                            self.camera_x = lerp(a.0, b.0, new_t);
                            self.camera_y = lerp(a.1, b.1, new_t);

                            self.lerp_state = LerpState::Lerping(LerpingState { a, b, t: new_t });
                        }
                    }
                }
                //self.camera_x = -1 * coords.0 + 350;
                //self.camera_y = -1 * coords.1 + 250;
            }
        }
        let image_entities = View::<(Position, ImageRender)>::new(world).collect::<Vec<EntityId>>();

        let mut image_entities_sorted = image_entities.clone();
        image_entities_sorted.sort_by(|e1, e2| {
            match (
                world.get_component::<Position>(*e1).unwrap(),
                world.get_component::<Position>(*e2).unwrap(),
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
                world.get_component::<Position>(*entity).unwrap(),
                world.get_component::<ImageRender>(*entity).unwrap(),
            ) {
                | (Some(position), Some(render)) => {
                    let (x, y) = tile_to_world(position.x, position.y);
                    screen.copy(
                        &textures.get(render.texture_index).unwrap(),
                        None,
                        Some(Rect::new(
                            ((x as f64) + self.camera_x) as i32,
                            ((y + render.y_offset) as f64 + self.camera_y) as i32,
                            render.width,
                            render.height,
                        )),
                    );
                }
                | _ => (),
            }
        }
    }
}
