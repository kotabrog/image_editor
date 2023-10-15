use anyhow::{anyhow, Result};
use wasm_bindgen::closure::{Closure, WasmClosure, WasmClosureFnOnce};
use web_sys::{
    Window, Document, HtmlImageElement, Event,
};

mod input;
mod file_reader;
mod canvas;
mod image;

pub use input::{
    input, event_current_target,
};
pub use file_reader::{
    file_reader, file_reader_result, file_reader_read_as_data_url,
};
pub use canvas::{
    canvas, context_from_canvas, get_canvas_size,
};
pub use image::{
    get_context_image_data, image_data,
};

pub type EventClosure = Closure<dyn FnMut(Event)>;

#[allow(unused_macros)]
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into())
    };
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No Document Found"))
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

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
}

pub fn create_event_closure(f: impl FnMut(Event) + 'static) -> EventClosure {
    closure_wrap(Box::new(f))
}
