use anyhow::Result;
use web_sys::{
    HtmlCanvasElement, CanvasRenderingContext2d
};
use crate::browser;

#[derive(Debug)]
pub struct Canvas {
    canvas: HtmlCanvasElement,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let canvas = browser::create_canvas(width, height)?;
        Ok(Self {
            canvas,
        })
    }

    pub fn new_from_element(canvas: HtmlCanvasElement) -> Self {
        Self {
            canvas,
        }
    }

    pub fn set_canvas_size_from_display_size(&self) -> Result<()> {
        let (width, height) = browser::get_canvas_display_size(&self.canvas);
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        Ok(())
    }

    pub fn to_context(&self) -> Result<CanvasRenderingContext2d> {
        browser::context_from_canvas(&self.canvas)
    }

    pub fn size(&self) -> (u32, u32) {
        browser::get_canvas_size(&self.canvas)
    }

    pub fn to_data_url(&self) -> Result<String> {
        browser::canvas_to_data_url(&self.canvas)
    }
}
