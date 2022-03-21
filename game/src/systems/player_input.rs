use sdl2::keyboard::Keycode;

use crate::Player;
use crate::Position;

use engine::world::EntityId;
use engine::world::View;
use engine::world::World;

pub fn process_player_input(world: &mut World, keycode: Keycode) {
    let delta = match keycode {
        | Keycode::Up => Position { x: 0, y: -1 },
        | Keycode::Down => Position { x: 0, y: 1 },
        | Keycode::Left => Position { x: -1, y: 0 },
        | Keycode::Right => Position { x: 1, y: 0 },
        | _ => Position { x: 0, y: 0 },
    };

    for entity in View::<(Player, Position)>::new(world).collect::<Vec<EntityId>>() {
        let position: &mut Position = world.get_component_mut::<Position>(entity).unwrap().unwrap();
        position.add(&delta);
        println!("player position: {}, {}", position.x, position.y);
    }
}
