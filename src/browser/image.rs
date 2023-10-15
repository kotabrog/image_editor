use anyhow::{anyhow, Result};
use wasm_bindgen::Clamped;
use web_sys::{
    CanvasRenderingContext2d, ImageData
};

pub fn get_context_image_data(context: &CanvasRenderingContext2d,
    x: u32, y: u32, width: u32, height: u32)
-> Result<ImageData> {
context
.get_image_data(x as f64, y as f64, width as f64, height as f64)
.map_err(|err| anyhow!("Could not get image data {:#?}", err))
}

pub fn image_data(data: &[u8], width: u32, height: u32) -> Result<ImageData> {
    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(data), width, height)
        .map_err(|err| anyhow!("Could not create image data {:#?}", err))
}
