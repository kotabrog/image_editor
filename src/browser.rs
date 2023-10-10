use anyhow::{anyhow, Result};
use wasm_bindgen::{
    JsCast,
    closure::{Closure, WasmClosure, WasmClosureFnOnce}
};
use web_sys::{
    Window, Document, HtmlImageElement, HtmlCanvasElement,
    CanvasRenderingContext2d, HtmlInputElement, Event,
};

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

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn context_from_canvas(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|element|
            anyhow!("Error converting {:#?} to CanvasRenderingContext2d", element)
        )
}

pub fn get_canvas_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    let width = canvas.width();
    let height = canvas.height();
    (width, height)
}

pub fn file_input() -> Result<HtmlInputElement> {
    document()?
        .get_element_by_id("file_input")
        .ok_or_else(|| anyhow!("No Input Element found with ID 'file_input'"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlInputElement", element))
}

pub fn event_current_target(event: &Event) -> Result<HtmlInputElement> {
    event.current_target()
        .ok_or_else(|| anyhow!("No current target found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlInputElement", element))
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
