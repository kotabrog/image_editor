use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Result;
use web_sys::Event;

use crate::browser;
use crate::engine::{
    Button, Renderer, Canvas, Anchor,
};
use super::Editor;

fn setup_save_event_closure(editor: Rc<Mutex<Editor>>) -> Result<()> {
    if let Some(editor) = Editor::try_lock(&editor) {
        let image = if let Some(image) = editor.get_image_data() {
            image
        } else {
            log!("No image to save");
            return Ok(());
        };
        let (width, height) = image.size();
        let save_canvas = Canvas::new(width, height)?;
        let render = Renderer::create_from_canvas(&save_canvas)?;
        render.draw_image_data(&image)?;

        let data_url = save_canvas.to_data_url()?;

        let anchor = Anchor::new_from_name()?;
        anchor.set_href(&data_url);
        anchor.set_download("image.png");

        anchor.click();
    }
    Ok(())
}

pub fn setup_save_event(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let button_element = Button::new_from_id("save")?;

    let closure = browser::create_event_closure(move |_event: Event| {
        let editor_clone = editor.clone();
        if let Err(err) = setup_save_event_closure(editor_clone) {
            error!("{:#?}", err);
        }
    });

    button_element.add_event_listener_with_callback(&closure)?;
    closure.forget();

    Ok(())
}
