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
    pub fn new_from_name(width: u32, height: u32) -> Result<Self> {
        let canvas = browser::create_canvas(width, height)?;
        Ok(Self {
            canvas,
        })
    }

    pub fn get_element(&self) -> &HtmlCanvasElement {
        &self.canvas
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
