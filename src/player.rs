use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use crate::map::Map;

pub struct Player {
    pub position: (usize, usize),
}

impl Player {
    pub fn new(pos: (usize, usize)) -> Self {
        Self { position: pos }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let green = Color::RGB(0, 255, 25);
        canvas.set_draw_color(green);
        canvas.fill_rect(Rect::new(
            self.position.0.try_into().unwrap(),
            self.position.1.try_into().unwrap(),
            50,
            50,
        ));
    }

    pub fn update(&mut self, event: &Event, map: &Map) {
        let delta: (i32, i32) = match event {
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => (-1, 0),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => (1, 0),
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => (0, -1),
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => (0, 1),
            _ => (0, 0),
        };

        let new_position: (i32, i32) = (
            (self.position.0 + (delta.0 as usize)).try_into().unwrap(),
            (self.position.1 + (delta.1 as usize)).try_into().unwrap(),
        );
        if map.can_enter_tile(
            new_position.0.try_into().unwrap(),
            new_position.1.try_into().unwrap(),
        ) {
            self.position = (new_position.0 as usize, new_position.1 as usize);
        }
    }
}
