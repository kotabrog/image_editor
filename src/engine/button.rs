use anyhow::Result;
use web_sys::{
    HtmlButtonElement, Event,
};
use crate::browser;
use super::DisplayElement;

#[derive(Debug, Clone)]
pub struct Button {
    element: HtmlButtonElement,
}

impl Button {
    pub fn new(element: HtmlButtonElement) -> Self {
        Self {
            element,
        }
    }

    pub fn new_from_event(event: &Event) -> Result<Self> {
        let element = browser::event_current_target_to_button(event)?;
        Ok(Self::new(element))
    }

    pub fn new_from_id(id: &str) -> Result<Self> {
        let element = browser::button(id)?;
        Ok(Self::new(element))
    }

    pub fn id(&self) -> String {
        self.element.id()
    }

    pub fn add_event_listener_with_callback(
        &self,
        closure: &browser::EventClosure,
    ) -> Result<()> {
        browser::add_event_listener_with_callback_button(
            &self.element,
            "click",
            closure,
        )
    }
}

impl DisplayElement for Button {
    fn set_disabled(&self, disabled: bool) {
        self.element.set_disabled(disabled);
    }
}
