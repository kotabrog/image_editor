use anyhow::{Result, anyhow};
use web_sys::{
    HtmlInputElement, Event, File,
};
use crate::browser;

pub struct Input {
    element: HtmlInputElement,
}

impl Input {
    pub fn new(element: HtmlInputElement) -> Self {
        Self {
            element,
        }
    }

    pub fn new_from_event(event: Event) -> Result<Self> {
        let element = browser::event_current_target(&event)?;
        Ok(Self::new(element))
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
