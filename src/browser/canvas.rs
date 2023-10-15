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

pub fn get_canvas_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    let width = canvas.width();
    let height = canvas.height();
    (width, height)
}
