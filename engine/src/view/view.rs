use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::Sdl;

pub trait Screen {
    fn clear(&mut self);
    fn set_draw_color(&mut self, color: &RgbColor);
    fn texture_creator(&self) -> TextureCreator<WindowContext>; // TODO: remove/rework this
    fn present(&mut self);
    fn copy(&mut self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>); // TODO: remove/rework
    fn draw_rect(&mut self, rect: Rect);
    fn get_context(&self) -> &Sdl; // TODO: remove/rework this
}

//pub trait Texture {}

#[derive(Debug)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
