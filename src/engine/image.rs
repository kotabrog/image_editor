use std::rc::Rc;
use std::sync::Mutex;
use futures::channel::oneshot::channel;
use anyhow::{Result, anyhow};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::HtmlImageElement;
use crate::browser;

#[derive(Debug)]
pub struct Image {
    element: HtmlImageElement,
    size: (u32, u32),
}

impl Image {
    pub fn new(element: HtmlImageElement) -> Self {
        let (width, height) = (element.width(), element.height());
        Self {
            element,
            size: (width, height),
        }
    }

    pub fn element(&self) -> &HtmlImageElement {
        &self.element
    }

    pub fn calculate_fitted_size(
        &self,
        max_width: f64,
        max_height: f64,
    ) -> (f64, f64) {
        let width = self.size.0 as f64;
        let height = self.size.1 as f64;
        let ratio = width / height;
        let (dw, dh) = if width > height {
            (max_width, max_width / ratio)
        } else {
            (max_height * ratio, max_height)
        };
        (dw, dh)
    }

    pub async fn load_image(source: &str) -> Result<Self> {
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

        Ok(Self::new(image))
    }
}
