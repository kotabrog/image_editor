use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement,
};

use super::document;

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn create_canvas(width: u32, height: u32) -> Result<HtmlCanvasElement> {
    let canvas = document()?
        .create_element("canvas")
        .map_err(|err| anyhow!("Error creating canvas element {:#?}", err))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))?;
    canvas.set_width(width);
    canvas.set_height(height);
    Ok(canvas)
}

pub fn get_canvas_display_size(canvas: &HtmlCanvasElement) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    (rect.width(), rect.height())
}

pub fn get_canvas_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    let width = canvas.width();
    let height = canvas.height();
    (width, height)
}

pub fn canvas_to_data_url(canvas: &HtmlCanvasElement) -> Result<String> {
    canvas
        .to_data_url()
        .map_err(|err| anyhow!("Error converting canvas to data url {:#?}", err))
}

pub fn context_from_canvas(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|element|
            anyhow!("Error converting {:#?} to CanvasRenderingContext2d", element)
        )
}
