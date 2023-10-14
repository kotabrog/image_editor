use anyhow::Result;
use wasm_bindgen::prelude::*;

#[macro_use]
mod browser;
mod engine;
mod editor;

fn image_editor() -> Result<()> {
    editor::setup()?;
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
