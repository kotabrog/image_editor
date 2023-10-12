use std::rc::Rc;
use std::cell::RefCell;
use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{
    Event, FileReader,
};
use crate::browser;

mod renderer;
mod image;

pub use renderer::Renderer;
pub use image::Image;

async fn internal_draw_image_fit_canvas_from_source(source: String) -> Result<()> {
    let renderer = Renderer::new()?;
    let image = Image::load_image(source.as_str())
        .await?;
    renderer.clear();
    renderer.draw_image_fit_canvas(&image)?;
    Ok(())
}

pub fn draw_image_fit_canvas_from_source(source: String) -> Result<()> {
    browser::spawn_local(async move {
        if let Err(err) =
                internal_draw_image_fit_canvas_from_source(source).await {
            error!("{:#?}", err);
        }
    });
    Ok(())
}

fn setup_input_event_closure(event: Event) -> Result<()> {
    let input = browser::event_current_target(&event)?;
    if let Some(files) = input.files() {
        if files.length() > 0 {
            let file = files.get(0)
                .ok_or_else(|| anyhow!("No file found"))?;
            let reader = FileReader::new()
                .map_err(|err| anyhow!("Could not create FileReader {:#?}", err))?;

            let reader_ref = Rc::new(RefCell::new(reader));
            let reader_clone = reader_ref.clone();
        
            let onload_closure = browser::closure_wrap(Box::new(move |_event: Event| {
                let result = reader_clone.borrow().result().unwrap().as_string().unwrap();
                draw_image_fit_canvas_from_source(result).unwrap();
            }) as Box<dyn FnMut(_)>);

            reader_ref.borrow_mut().set_onload(Some(onload_closure.as_ref().unchecked_ref()));
            onload_closure.forget();

            reader_ref.borrow().read_as_data_url(&file)
                .map_err(|err| anyhow!("Could not read file {:#?}", err))?;
        }
    }
    Ok(())
}

pub fn setup_input_event() -> Result<()> {
    let input_element = browser::file_input()?;

    let closure = browser::closure_wrap(Box::new(move |event: Event| {
        if let Err(err) = setup_input_event_closure(event) {
            error!("{:#?}", err);
        }
    }) as Box<dyn FnMut(_)>);

    input_element.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}
