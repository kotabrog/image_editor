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
mod input;

pub use renderer::Renderer;
pub use image::Image;
pub use input::Input;

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

fn setup_input_event_closure_reader_closure(reader: &FileReader) -> Result<()> {
    let result = browser::file_reader_result(&reader)?;
    draw_image_fit_canvas_from_source(result)?;
    Ok(())
}

fn setup_input_event_closure(event: Event) -> Result<()> {
    let input = Input::new_from_event(event)?;
    if let Some(file) = input.get_first_image_file()? {
        let reader = browser::file_reader()?;

        let reader_ref = Rc::new(RefCell::new(reader));
        let reader_clone = reader_ref.clone();

        let onload_closure = browser::closure_wrap(Box::new(move |_event: Event| {
            if let Err(err) = setup_input_event_closure_reader_closure(&reader_clone.borrow()) {
                error!("{:#?}", err);
            }
        }) as Box<dyn FnMut(_)>);

        reader_ref.borrow_mut().set_onload(Some(onload_closure.as_ref().unchecked_ref()));
        onload_closure.forget();

        browser::file_reader_read_as_data_url(&reader_ref.borrow(), &file)?;
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
