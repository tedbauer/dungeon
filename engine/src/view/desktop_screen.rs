use anyhow::anyhow;
use anyhow::Result;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::surface::SurfaceContext;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::Sdl;

use crate::view::view::RgbColor;
use crate::view::view::Screen;

impl From<&RgbColor> for Color {
    fn from(color: &RgbColor) -> Color {
        Color::RGB(color.red, color.green, color.blue)
    }
}

/// A screen that appears on a desktop. Under the hood, this is implemented with SDL2.
pub struct DesktopScreen {
    canvas: Canvas<Window>,
    sdl_context: Sdl,
}

impl DesktopScreen {
    pub fn builder() -> DesktopScreenBuilder {
        DesktopScreenBuilder::new()
    }
}

impl Screen for DesktopScreen {
    fn clear(&mut self) {
        self.canvas.clear()
    }

    fn set_draw_color(&mut self, color: &RgbColor) {
        let color: Color = color.into();
        self.canvas.set_draw_color(color)
    }

    fn present(&mut self) {
        self.canvas.present()
    }

    fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }

    fn copy(&mut self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>) {
        self.canvas.copy(texture, src, dst).unwrap()
    }

    fn draw_rect(&mut self, rect: Rect) {
        self.canvas.draw_rect(rect).unwrap()
    }

    fn get_context(&self) -> &Sdl {
        &self.sdl_context
    }
}

pub struct DesktopScreenBuilder {
    title: Option<String>,
    size: Option<(u32, u32)>,
}

impl DesktopScreenBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            size: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_size(mut self, size: (u32, u32)) -> Self {
        self.size = Some(size);
        self
    }

    pub fn build(self) -> anyhow::Result<DesktopScreen> {
        let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
        let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;

        let title = self.title.ok_or_else(|| anyhow!("title not set"))?;
        let (width, height) = self.size.ok_or_else(|| anyhow!("screen width not set"))?;

        let window = video_subsystem
            .window(&title, width, height)
            .position_centered()
            .build()?;

        let mut canvas: Canvas<Window> = window.into_canvas().build()?;

        sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

        Ok(DesktopScreen {
            canvas: canvas,
            sdl_context: sdl_context,
        })
    }
}
