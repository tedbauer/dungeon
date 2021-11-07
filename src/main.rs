extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

mod map;
mod player;
use map::Map;
use player::Player;

// TODO:
// Frame rate throttling

struct State {
    map: Map,
    player: Player,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new((0, 0)),
        }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.map.render(canvas);
        self.player.render(canvas);
    }

    pub fn update(&mut self, event: &Event) {
        self.player.update(event, &self.map)
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("dungeon", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let width = 400;
    let height = 400;

    let white = Color::RGB(255, 255, 255);
    let black = Color::RGB(0, 0, 0);

    canvas.set_draw_color(black);

    let mut state = State::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        state.render(&mut canvas);
        canvas.present();

        for event in event_pump.poll_iter() {
            state.update(&event);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
