use sdl2::{render::Canvas, video::Window};

use crate::Matrix;

pub struct View {
    canvas: Canvas<Window>,
}

impl View {
    pub fn new() -> Result<Self, String> {
        let sdl_ctx = sdl2::init()?;
        let video_subsys = sdl_ctx.video()?;

        let window = video_subsys
            .window("Filler", 800, 800)
            .position_centered()
            .build()
            .map_err(|e| format!("{e}"))?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self { canvas })
    }

    pub fn display(&mut self, matrix: &Matrix) -> Result<(), String> {
        Ok(())
    }
}
