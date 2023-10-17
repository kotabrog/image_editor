use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlButtonElement, Event,
};

use super::document;

pub fn button(id: &str) -> Result<HtmlButtonElement> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| anyhow!("No Input Element found with ID {}", id))?
        .dyn_into::<HtmlButtonElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlButtonElement", element))
}

pub fn event_current_target_to_button(event: &Event) -> Result<HtmlButtonElement> {
    event.current_target()
        .ok_or_else(|| anyhow!("No current target found"))?
        .dyn_into::<HtmlButtonElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlButtonElement", element))
}

pub fn add_event_listener_with_callback_button(
    element: &HtmlButtonElement,
    event_name: &str,
    closure: &super::EventClosure,
) -> Result<()> {
    element.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Error adding event listener with callback {:#?}", err))
}
