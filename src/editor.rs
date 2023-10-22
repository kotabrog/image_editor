use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};
use anyhow::Result;
use crate::engine::{
    self, Image, Renderer, ImageDataWrapper, Canvas, DisplayElement,
    Input, Button, Rect,
};

mod input;
mod binarization;
mod save;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorElement {
    Input,
    Binarization,
    Save,
}

#[derive(Debug)]
pub struct Editor {
    image: Option<Image>,
    renderer: Renderer,
    image_data: Option<ImageDataWrapper>,
    display_elements: HashMap<EditorElement, Box<dyn DisplayElement>>,
}

impl Editor {
    pub fn new(renderer: Renderer) -> Result<Self> {
        Ok(Self {
            image: None,
            renderer,
            image_data: None,
            display_elements: Self::make_display_elements()?,
        })
    }

    fn make_display_elements() -> Result<HashMap<EditorElement, Box<dyn DisplayElement>>> {
        let mut display_elements: HashMap<EditorElement, Box<dyn DisplayElement>> = HashMap::new();
        display_elements.insert(
            EditorElement::Input,
            Box::new(Input::new_from_id("file_input")?)
        );
        display_elements.insert(
            EditorElement::Binarization,
            Box::new(Button::new_from_id("binarization")?)
        );
        display_elements.insert(
            EditorElement::Save,
            Box::new(Button::new_from_id("save")?)
        );
        Ok(display_elements)
    }

    pub fn try_lock<'a>(editor: &'a Rc<Mutex<Self>>) -> Option<MutexGuard<'a, Editor>> {
        match editor.try_lock() {
            Ok(editor) => Some(editor),
            Err(_) => {
                log!("Editor is locked");
                None
            },
        }
    }

    pub fn get_image_data(&self) -> Option<&ImageDataWrapper> {
        self.image_data.as_ref()
    }

    pub fn get_image_data_mut(&mut self) -> Option<&mut [u8]> {
        if let Some(image_data) = &mut self.image_data {
            Some(image_data.data_mut())
        } else {
            None
        }
    }

    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    pub fn setup_image_data(&mut self) -> Result<()> {
        if let Some(image) = &self.image {
            let (width, height) = image.size();
            let save_canvas = Canvas::new(width, height)?;
            let render = Renderer::create_from_canvas(&save_canvas)?;
            render.draw_image(&image, &Rect::new(0.0, 0.0, width as f64, height as f64))?;

            let image_data = ImageDataWrapper::new_from_context(
                &render.context(), 0, 0, width, height)?;
            self.image_data = Some(image_data);
        }
        Ok(())
    }

    pub fn set_image_data(&mut self) -> Result<()> {
        if let Some(image_data) = &mut self.image_data {
            image_data.set_image_data()?;
        }
        Ok(())
    }

    pub fn update_canvas_size(&mut self) -> Result<()> {
        self.renderer.update_canvas_size()
    }

    pub fn draw_image_fit_canvas(&self) -> Result<()> {
        if let Some(image) = &self.image {
            self.renderer.clear();
            self.renderer.draw_image_fit_canvas(&image)?;
        }
        Ok(())
    }

    pub async fn draw_image_data(&self) -> Result<()> {
        if let Some(image_data) = &self.image_data {
            self.renderer.draw_image_data_fit_canvas(&image_data).await?;
        }
        Ok(())
    }

    pub fn set_disabled(&self, disabled: bool) {
        for display_element in self.display_elements.values() {
            display_element.set_disabled(disabled);
        }
    }
}

pub fn setup() -> Result<()> {
    let editor = Rc::new(
        Mutex::new(Editor::new(engine::Renderer::new()?
    )?));
    input::setup_input_event(editor.clone())?;
    binarization::setup_binarization_event(editor.clone())?;
    save::setup_save_event(editor.clone())?;
    Ok(())
}
