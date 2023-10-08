use std::rc::Rc;
use std::sync::Mutex;
use futures::channel::oneshot::channel;
use anyhow::{anyhow, Result};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::HtmlImageElement;
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
