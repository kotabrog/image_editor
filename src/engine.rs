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

pub fn clear_canvas() -> Result<()> {
    let ctx = browser::context()?;
    let (width, height) = get_canvas_size()?;
    ctx.clear_rect(0.0, 0.0, width, height);
    Ok(())
}

pub fn draw_image(image: HtmlImageElement, dw: f64, dh: f64) -> Result<()> {
    let ctx = browser::context()?;
    ctx.draw_image_with_html_image_element_and_dw_and_dh(&image, 0.0, 0.0, dw, dh)
        .map_err(|err| anyhow!("Could not draw image {:#?}", err))
        .map(|_| ())
}

pub fn calculate_fitted_size(
    image: &HtmlImageElement,
    max_width: f64,
    max_height: f64,
) -> (f64, f64) {
    let width = image.width() as f64;
    let height = image.height() as f64;
    let ratio = width / height;
    let (dw, dh) = if width > height {
        (max_width, max_width / ratio)
    } else {
        (max_height * ratio, max_height)
    };
    (dw, dh)
}

pub fn get_canvas_size() -> Result<(f64, f64)> {
    let canvas = browser::canvas()?;
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    Ok((width, height))
}

pub fn draw_image_fit_canvas(image: HtmlImageElement) -> Result<()> {
    let (dw, dh) = {
        let (width, height) = get_canvas_size()?;
        calculate_fitted_size(&image, width, height)
    };
    draw_image(image, dw, dh)
}

pub fn draw_image_fit_canvas_from_source(source: String) -> Result<()> {
    browser::spawn_local(async move {
        let image = load_image(source.as_str())
            .await
            .expect("Could not load image");
        clear_canvas().expect("Could not clear canvas");
        draw_image_fit_canvas(image)
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
