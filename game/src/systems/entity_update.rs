use sdl2::keyboard::Keycode;

use crate::Player;
use crate::Position;
use crate::RandomWalk;

use engine::world::EntityId;
use engine::world::View;
use engine::world::World;

use rand::Rng;

pub fn entity_update(world: &mut World) {
    for entity in View::<(RandomWalk, Position)>::new(world).collect::<Vec<EntityId>>() {
        let delta = Position {
            x: *vec![-1, 0, 1]
                .get(rand::thread_rng().gen_range(0..1))
                .unwrap(),
            y: *vec![-1, 0, 1]
                .get(rand::thread_rng().gen_range(0..1))
                .unwrap(),
        };

        let position: &mut Position = world.get_component_mut::<Position>(entity).unwrap();
        position.add(&delta);
    }
}
