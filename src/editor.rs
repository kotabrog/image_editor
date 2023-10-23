use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};
use anyhow::Result;
use crate::engine::{
    self, Image, Renderer, ImageDataWrapper, Canvas, DisplayElement,
    Input, Button, Rect, Label,
};

mod input;
mod binarization;
mod save;
mod image_data_list;
mod back_and_forward;

pub use image_data_list::ImageDataList;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorElement {
    InputLabel,
    Input,
    Binarization,
    Save,
    Undo,
    Redo,
}

#[derive(Debug)]
pub struct Editor {
    image: Option<Image>,
    renderer: Renderer,
    image_data: ImageDataList,
    display_elements: HashMap<EditorElement, Box<dyn DisplayElement>>,
}

impl Editor {
    pub fn new(renderer: Renderer) -> Result<Self> {
        Ok(Self {
            image: None,
            renderer,
            image_data: ImageDataList::new(),
            display_elements: Self::make_display_elements()?,
        })
    }

    fn make_display_elements() -> Result<HashMap<EditorElement, Box<dyn DisplayElement>>> {
        let mut display_elements: HashMap<EditorElement, Box<dyn DisplayElement>> = HashMap::new();
        display_elements.insert(
            EditorElement::InputLabel,
            Box::new(Label::new_from_id("file_input_label")?)
        );
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
        display_elements.insert(
            EditorElement::Undo,
            Box::new(Button::new_from_id("back")?)
        );
        display_elements.insert(
            EditorElement::Redo,
            Box::new(Button::new_from_id("forward")?)
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
        self.image_data.get_image_data()
    }

    pub fn get_image_data_mut(&mut self) -> Option<&mut [u8]> {
        self.image_data.get_image_data_inner_mut()
    }

    pub fn have_image_data(&self) -> bool {
        !self.image_data.is_empty()
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
            self.image_data.push(image_data);
        }
        Ok(())
    }

    pub fn data_to_image_data(&mut self) -> Result<()> {
        self.image_data.data_to_image_data()
    }

    pub fn update_canvas_size(&mut self) -> Result<()> {
        self.renderer.update_canvas_size()
    }

    pub fn clone_push(&mut self) {
        self.image_data.clone_push();
    }

    pub fn undo(&mut self) -> Option<&ImageDataWrapper> {
        self.image_data.undo()
    }

    pub fn redo(&mut self) -> Option<&ImageDataWrapper> {
        self.image_data.redo()
    }

    pub fn draw_image_fit_canvas(&self) -> Result<()> {
        if let Some(image) = &self.image {
            self.renderer.clear();
            self.renderer.draw_image_fit_canvas(&image)?;
        }
        Ok(())
    }

    pub async fn draw_image_data(&self) -> Result<()> {
        if let Some(image_data) = self.image_data.get_image_data() {
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
    back_and_forward::setup_back_and_forward_event(editor.clone())?;
    Ok(())
}
