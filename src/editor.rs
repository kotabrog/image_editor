use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Result;
use crate::engine::{
    self, Image, Renderer,
};

mod input;

#[derive(Debug)]
pub struct Editor {
    image: Option<Image>,
    renderer: Renderer,
}

impl Editor {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            image: None,
            renderer,
        }
    }

    pub fn draw_image_fit_canvas(&self) -> Result<()> {
        if let Some(image) = &self.image {
            self.renderer.clear();
            self.renderer.draw_image_fit_canvas(&image)?;
        }
        Ok(())
    }

    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }
}

pub fn setup() -> Result<()> {
    let editor = Rc::new(
        Mutex::new(Editor::new(engine::Renderer::new()?
    )));
    input::setup_input_event(editor.clone())?;
    Ok(())
}
