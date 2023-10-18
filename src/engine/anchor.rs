use anyhow::Result;
use web_sys::HtmlAnchorElement;
use crate::browser;

#[derive(Debug, Clone)]
pub struct Anchor {
    element: HtmlAnchorElement,
}

impl Anchor {
    pub fn new(element: HtmlAnchorElement) -> Self {
        Self {
            element,
        }
    }

    pub fn new_from_name() -> Result<Self> {
        let element = browser::create_anchor()?;
        Ok(Self::new(element))
    }

    pub fn get_element(&self) -> &HtmlAnchorElement {
        &self.element
    }

    pub fn set_href(&self, href: &str) {
        self.element.set_href(href);
    }

    pub fn set_download(&self, download: &str) {
        self.element.set_download(download);
    }

    // pub fn display_none(&self) -> Result<()> {
    //     browser::css_style_set_property(
    //         &self.element.style(),
    //         "display",
    //         "none",
    //     )
    // }

    pub fn click(&self) {
        self.element.click();
    }
}
