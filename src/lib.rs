use anyhow::Result;
use wasm_bindgen::prelude::*;

#[macro_use]
mod browser;
mod engine;

fn image_editor() -> Result<()> {
    browser::spawn_local(async move {
        let image = engine::load_image("image_sample.PNG")
            .await
            .expect("Could not load image");
        browser::append_child(&image)
            .expect("Could not append child");
    });
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
