use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlInputElement, Event,
};

use super::document;

pub fn input(id: &str) -> Result<HtmlInputElement> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| anyhow!("No Input Element found with ID {}", id))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlInputElement", element))
}

pub fn event_current_target(event: &Event) -> Result<HtmlInputElement> {
    event.current_target()
        .ok_or_else(|| anyhow!("No current target found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlInputElement", element))
}
