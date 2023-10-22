use anyhow::Result;
use web_sys::HtmlLabelElement;
use crate::browser;
use super::DisplayElement;

#[derive(Debug, Clone)]
pub struct Label {
    element: HtmlLabelElement,
}

impl Label {
    pub fn new(element: HtmlLabelElement) -> Self {
        Self {
            element,
        }
    }

    pub fn new_from_id(id: &str) -> Result<Self> {
        let element = browser::label(id)?;
        Ok(Self::new(element))
    }
}

impl DisplayElement for Label {
    fn set_disabled(&self, disabled: bool) {
        if disabled {
            if let Err(err) = self.element.class_list()
                    .add_1("disabled") {
                log!("{:#?}", err)
            }
        } else {
            if let Err(err) = self.element.class_list()
                    .remove_1("disabled") {
                log!("{:#?}", err)
            }
        }
    }
}
