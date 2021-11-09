use crate::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

const WIDTH_TILES: usize = 20;
const HEIGHT_TILES: usize = 30;
const NUM_TILES: usize = WIDTH_TILES * HEIGHT_TILES;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TileType> {
        self.tiles.get((y * (WIDTH_TILES as i32) + x) as usize)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) {
        if let Some(t) = self.tiles.get_mut((y * (WIDTH_TILES as i32) + x) as usize) {
            *t = tile;
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y < WIDTH_TILES.try_into().unwrap()
            && y >= 0
            && y < HEIGHT_TILES.try_into().unwrap()
    }

    pub fn can_enter_tile(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.get_tile(x, y) == Some(&TileType::Floor)
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let black = Color::RGB(0, 0, 0);
        let grey = Color::RGB(153, 135, 135);
        for x in 0..WIDTH_TILES {
            for y in 0..HEIGHT_TILES {
                match self
                    .get_tile(x.try_into().unwrap(), y.try_into().unwrap())
                    .unwrap()
                {
                    TileType::Floor => {
                        canvas.set_draw_color(black);
                        canvas.fill_rect(Rect::new((x * 50) as i32, (y * 50) as i32, 50, 50));
                    }
                    TileType::Wall => {
                        canvas.set_draw_color(grey);
                        canvas.fill_rect(Rect::new((x * 50) as i32, (y * 50) as i32, 50, 50));
                    }
                }
            }
        }
    }
}
