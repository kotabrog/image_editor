use anyhow::{Result, anyhow};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlInputElement, Event, File,
};
use crate::browser;
use super::DisplayElement;

#[derive(Debug, Clone)]
pub struct Input {
    element: HtmlInputElement,
}

impl Input {
    pub fn new(element: HtmlInputElement) -> Self {
        Self {
            element,
        }
    }

    pub fn new_from_event(event: &Event) -> Result<Self> {
        let element = browser::event_current_target(event)?;
        Ok(Self::new(element))
    }

    pub fn new_from_id(id: &str) -> Result<Self> {
        let element = browser::input(id)?;
        Ok(Self::new(element))
    }

    // pub fn id(&self) -> String {
    //     self.element.id()
    // }

    pub fn set_onchange(&self, closure: &browser::EventClosure) {
        self.element.set_onchange(Some(closure.as_ref().unchecked_ref()));
    }

    pub fn get_first_image_file(&self) -> Result<Option<File>> {
        let files = self.element.files();
        if let Some(files) = files {
            if files.length() > 0 {
                let file = files.get(0)
                    .ok_or_else(|| anyhow!("No file found"))?;
                return Ok(Some(file))
            }
        }
        Ok(None)
    }
}

impl DisplayElement for Input {
    fn set_disabled(&self, disabled: bool) {
        self.element.set_disabled(disabled);
    }
}
