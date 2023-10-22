use anyhow::Result;
use web_sys::{
    ImageData, CanvasRenderingContext2d,
};
use crate::browser;
use super::{
    Image, Canvas, Renderer,
};

#[derive(Debug)]
pub struct ImageDataWrapper {
    image_data: ImageData,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl ImageDataWrapper {
    pub fn new(image_data: ImageData) -> Self {
        let width = image_data.width();
        let height = image_data.height();
        let data = image_data.data().to_vec();
        Self {
            image_data,
            width,
            height,
            data,
        }
    }

    pub fn new_from_context(context: &CanvasRenderingContext2d,
                            x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        browser::get_context_image_data(context, x, y, width, height)
            .map(Self::new)
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn image_data(&self) -> &ImageData {
        &self.image_data
    }

    pub fn set_image_data(&mut self) -> Result<()> {
        self.image_data = browser::image_data(&self.data, self.width, self.height)?;
        Ok(())
    }

    pub async fn to_image(&self) -> Result<Image> {
        let canvas = Canvas::new(self.width, self.height)?;
        let render = Renderer::create_from_canvas(&canvas)?;
        render.draw_image_data(&self)?;

        let data_url = canvas.to_data_url()?;

        Image::load_image(&data_url).await
    }
}
