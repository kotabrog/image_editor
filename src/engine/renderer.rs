use anyhow::{Result, anyhow};
use web_sys::CanvasRenderingContext2d;
use crate::browser;
use super::{
    Image, ImageDataWrapper, Canvas, Rect,
};

#[derive(Debug)]
pub struct Renderer {
    context: CanvasRenderingContext2d,
    size: (u32, u32),
}

impl Renderer {
    pub fn new() -> Result<Self> {
        let canvas = Canvas::new_from_element(browser::canvas()?);
        canvas.set_canvas_size_from_display_size()?;
        let (width, height) = canvas.size();
        let context = canvas.to_context()?;
        Ok(Self {
            context,
            size: (width, height),
        })
    }

    pub fn create_from_canvas(canvas: &Canvas) -> Result<Self> {
        let context = canvas.to_context()?;
        let (width, height) = canvas.size();
        Ok(Self {
            context,
            size: (width, height),
        })
    }

    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.context
    }

    pub fn update_canvas_size(&mut self) -> Result<()> {
        let canvas = Canvas::new_from_element(browser::canvas()?);
        canvas.set_canvas_size_from_display_size()?;
        self.size = canvas.size();
        Ok(())
    }

    pub fn get_draw_image_rect(&self, width: f64, height: f64) -> Rect {
        let (canvas_w, canvas_h) = self.size;
        let mut rect = Rect::new(0.0, 0.0, width, height);
        rect.to_center(canvas_w as f64, canvas_h as f64);
        rect
    }

    pub fn clear(&self) {
        self.context.clear_rect(
            0.0, 0.0, self.size.0 as f64, self.size.1 as f64);
    }

    pub fn draw_image(&self, image: &Image, rect: &Rect) -> Result<()> {
        self.context.draw_image_with_html_image_element_and_dw_and_dh(
            image.element(), rect.x, rect.y, rect.width, rect.height)
            .map_err(|err| anyhow!("Could not draw image {:#?}", err))
            .map(|_| ())
    }

    pub fn draw_image_fit_canvas(&self, image: &Image) -> Result<()> {
        let (width, height) = self.size;
        let (dw, dh) = image
            .calculate_fitted_size(width as f64, height as f64);
        let rect = self.get_draw_image_rect(dw, dh);
        self.draw_image(image, &rect)
    }

    pub fn draw_image_data(&self, image_data: &ImageDataWrapper) -> Result<()> {
        let (width, height) = image_data.size();
        let rect = self.get_draw_image_rect(width as f64, height as f64);
        self.context.put_image_data(
            &image_data.image_data(), rect.x, rect.y)
            .map_err(|err| anyhow!("Could not draw image data {:#?}", err))
            .map(|_| ())
    }

    pub async fn draw_image_data_fit_canvas(&self, image_data: &ImageDataWrapper) -> Result<()> {
        let image = image_data.to_image().await?;
        self.clear();
        self.draw_image_fit_canvas(&image)
    }
}
