use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;

use super::document;

pub fn create_anchor() -> Result<HtmlAnchorElement> {
    document()?
        .create_element("a")
        .map_err(|err| anyhow!("Error creating anchor element {:#?}", err))?
        .dyn_into::<HtmlAnchorElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlAnchorElement", element))
}
