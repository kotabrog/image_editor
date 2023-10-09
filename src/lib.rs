use anyhow::Result;
use wasm_bindgen::prelude::*;

#[macro_use]
mod browser;
mod engine;

fn image_editor() -> Result<()> {
    engine::setup_input_event()?;
    Ok(())
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    match image_editor() {
        Ok(_) => (),
        Err(err) => error!("{:#?}", err),
    }
    Ok(())
}
