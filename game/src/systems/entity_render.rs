use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::Player;
use crate::Position;
use crate::Render;
use crate::RenderImage;
use crate::sdl2::image::LoadTexture;

use engine::world::EntityId;
use engine::world::View;
use engine::world::World;

pub fn entity_render(world: &World, canvas: &mut Canvas<Window>) {
    for entity in View::<(Position, Render)>::new(world) {
        let position: &Position = world.get_component::<Position>(entity).unwrap();
        let render: &Render = world.get_component::<Render>(entity).unwrap();

        canvas.set_draw_color(render.color);
        canvas.draw_rect(Rect::new(position.x, position.y, 50, 50));
    }

		/*
		let texture_creator = canvas.texture_creator();
		for entity in View::<(Position, RenderImage)>::new(world) {
        let position: &Position = world.get_component::<Position>(entity).unwrap();
        let render: &RenderImage = world.get_component::<RenderImage>(entity).unwrap();
				let tex = texture_creator.load_texture(render.texture.clone()).unwrap();

        canvas.copy(&tex, None, None).unwrap();
		}
		*/
}
