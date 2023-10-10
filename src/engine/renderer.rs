use anyhow::{Result, anyhow};
use web_sys::{
    CanvasRenderingContext2d, HtmlImageElement,
};
use crate::browser;

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

    pub fn draw_image(&self, image: &HtmlImageElement, dw: f64, dh: f64) -> Result<()> {
        self.context.draw_image_with_html_image_element_and_dw_and_dh(
            image, 0.0, 0.0, dw, dh)
            .map_err(|err| anyhow!("Could not draw image {:#?}", err))
            .map(|_| ())
    }

    pub fn draw_image_fit_canvas(&self, image: &HtmlImageElement) -> Result<()> {
        let (dw, dh) = {
            let (width, height) = self.size;
            calculate_fitted_size(image, width as f64, height as f64)
        };
        self.draw_image(image, dw, dh)
    }
}

pub fn calculate_fitted_size(
    image: &HtmlImageElement,
    max_width: f64,
    max_height: f64,
) -> (f64, f64) {
    let width = image.width() as f64;
    let height = image.height() as f64;
    let ratio = width / height;
    let (dw, dh) = if width > height {
        (max_width, max_width / ratio)
    } else {
        (max_height * ratio, max_height)
    };
    (dw, dh)
}
