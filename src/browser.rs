use anyhow::{anyhow, Result};
use wasm_bindgen::{
    JsCast,
    closure::{Closure, WasmClosure, WasmClosureFnOnce}
};
use web_sys::{
    Window, Document, Event, HtmlLabelElement,
    HtmlElement,
};

mod input;
mod file_reader;
mod canvas;
mod image;
mod button;
mod anchor;
mod media_query_list;

pub use input::{
    input, event_current_target,
};
pub use file_reader::{
    file_reader, file_reader_result, file_reader_read_as_data_url,
};
pub use canvas::{
    canvas, context_from_canvas, get_canvas_size, create_canvas,
    canvas_to_data_url, get_canvas_display_size,
};
pub use image::{
    get_context_image_data, image_data, new_image,
};
pub use button::{
    button, event_current_target_to_button,
    add_event_listener_with_callback_button,
};
pub use anchor::create_anchor;
pub use media_query_list::{
    make_media_query_list, get_color_scheme_media_query_list,
    add_listener_with_opt_callback, event_current_target_to_media_query_list,
};

pub type EventClosure = Closure<dyn FnMut(Event)>;

#[allow(unused_macros)]
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
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

pub fn body() -> Result<HtmlElement> {
    document()?
        .body()
        .ok_or_else(|| anyhow!("No Body Found"))
}

pub fn set_attribute_to_body(name: &str, value: &str) -> Result<()> {
    body()?
        .set_attribute(name, value)
        .map_err(|err| anyhow!("Error setting attribute {:#?} to {:#?}: {:#?}", name, value, err))
        .map(|_| ())
}

pub fn get_attribute_from_body(name: &str) -> Result<Option<String>> {
    Ok(body()?.get_attribute(name))
}

pub fn label(id: &str) -> Result<HtmlLabelElement> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| anyhow!("No Label Element found with ID {}", id))?
        .dyn_into::<HtmlLabelElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlLabelElement", element))
}

// pub fn css_style_set_property(
//     style: &CssStyleDeclaration,
//     property: &str,
//     value: &str,
// ) -> Result<()> {
//     style
//         .set_property(property, value)
//         .map_err(|err| anyhow!("Error setting property {:#?} to {:#?}: {:#?}", property, value, err))
//         .map(|_| ())
// }

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

pub fn set_timeout_with_callback(
    window: &Window,
    callback: Closure<dyn FnMut()>,
) -> Result<()>
{
    window
        .set_timeout_with_callback(
            callback.as_ref().unchecked_ref(),
        )
        .map_err(|err| anyhow!("Could not set timeout {:#?}", err))
        .map(|_| ())?;
    callback.forget();
    Ok(())
}

pub fn set_callback_once<F>(fn_once: F) -> Result<()>
where
    F: 'static + FnOnce(),
{
    set_timeout_with_callback(
        &window().unwrap(),
        closure_once(fn_once),
    )
}
