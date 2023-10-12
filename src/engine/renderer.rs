use anyhow::{Result, anyhow};
use web_sys::CanvasRenderingContext2d;
use crate::browser;
use super::Image;

pub struct Renderer {
    context: CanvasRenderingContext2d,
    size: (u32, u32),
}

impl Renderer {
    pub fn new() -> Result<Self> {
        let canvas = browser::canvas()?;
        let context = browser::context_from_canvas(&canvas)?;
        let (width, height) = browser::get_canvas_size(&canvas);
        Ok(Self {
            context,
            size: (width, height),
        })
    }

    pub fn clear(&self) {
        self.context.clear_rect(
            0.0, 0.0, self.size.0 as f64, self.size.1 as f64);
    }

    pub fn draw_image(&self, image: &Image, dw: f64, dh: f64) -> Result<()> {
        self.context.draw_image_with_html_image_element_and_dw_and_dh(
            image.element(), 0.0, 0.0, dw, dh)
            .map_err(|err| anyhow!("Could not draw image {:#?}", err))
            .map(|_| ())
    }

    pub fn draw_image_fit_canvas(&self, image: &Image) -> Result<()> {
        let (width, height) = self.size;
        let (dw, dh) = image
            .calculate_fitted_size(width as f64, height as f64);
        self.draw_image(image, dw, dh)
    }
}
