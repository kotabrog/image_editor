use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;
use futures::channel::oneshot::channel;
use anyhow::{anyhow, Result};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{
    HtmlImageElement, Event, FileReader,
};
use crate::browser;

mod renderer;

pub use renderer::Renderer;

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;

    let (complete_tx, complete_rx) =
        channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);

    let success_callback =
        browser::closure_once(move || {
            if let Some(success_tx) = success_tx
                    .lock()
                    .ok()
                    .and_then(|mut opt| opt.take()) {
                        success_tx.send(Ok(())).unwrap();
        }
    });

    let error_callback: Closure<dyn FnMut(JsValue)> =
        browser::closure_once(move |err| {
            if let Some(error_tx) = error_tx
                    .lock()
                    .ok()
                    .and_then(|mut opt| opt.take()) {
                        error_tx.send(Err(anyhow!("Error loading image: {:#?}", err))).unwrap();
        }
    });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);

    complete_rx.await??;

    Ok(image)
}

pub fn draw_image_fit_canvas_from_source(source: String) -> Result<()> {
    browser::spawn_local(async move {
        let renderer = Renderer::new()
            .expect("Could not create renderer");
        let image = load_image(source.as_str())
            .await
            .expect("Could not load image");
        renderer.clear();
        renderer.draw_image_fit_canvas(&image)
            .expect("Could not draw image");
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
            
            reader_ref.borrow().read_as_data_url(&file).unwrap();
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
