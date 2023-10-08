use anyhow::{anyhow, Result};
use wasm_bindgen::{
    JsCast,
    closure::{Closure, WasmClosureFnOnce}
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Window, Document, HtmlImageElement,
};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

#[allow(unused_macros)]
macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    };
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No Document Found"))
}

pub fn create_img_element() -> Result<HtmlImageElement> {
    document()?.create_element("img")
        .map_err(|err| anyhow!("Could not create img element {:#?}", err))?
        .dyn_into::<HtmlImageElement>()
        .map_err(|err| anyhow!("Could not convert to HtmlImageElement {:#?}", err))
}

pub fn append_child(child: &HtmlImageElement) -> Result<()> {
    document()?.body()
        .ok_or_else(|| anyhow!("No Body Found"))?
        .append_child(child)
        .map_err(|err| anyhow!("Could not append child {:#?}", err))
        .map(|_| ())
}

pub fn new_image() -> Result<HtmlImageElement> {
    HtmlImageElement::new()
        .map_err(|err| anyhow!("Could not create HtmlImageElement {:#?}", err))
}

pub fn spawn_local<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}
