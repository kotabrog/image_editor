use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;
use anyhow::Result;
use wasm_bindgen::JsCast;
use web_sys::{
    Event, FileReader,
};
use crate::browser;
use crate::engine::{
    Image, Input,
};
use super::Editor;

async fn internal_draw_image_fit_canvas_from_source(editor: Rc<Mutex<Editor>>, source: String) -> Result<()> {
    let image = Image::load_image(source.as_str())
         .await?;
    match editor.try_lock() {
        Ok(mut editor) => {
            editor.set_image(image);
            editor.draw_image_fit_canvas()?; 
            editor.setup_image_data()?;
        },
        Err(_) => {
            log!("Editor is locked");
        },
    }
    Ok(())
}

pub fn draw_image_fit_canvas_from_source(editor: Rc<Mutex<Editor>>, source: String) -> Result<()> {
    browser::spawn_local(async move {
        if let Err(err) =
                internal_draw_image_fit_canvas_from_source(editor, source).await {
            error!("{:#?}", err);
        }
    });
    Ok(())
}

fn setup_input_event_closure_reader_closure(editor: Rc<Mutex<Editor>>, reader: &FileReader) -> Result<()> {
    let result = browser::file_reader_result(&reader)?;
    draw_image_fit_canvas_from_source(editor, result)?;
    Ok(())
}

fn setup_input_event_closure(editor: Rc<Mutex<Editor>>, event: Event) -> Result<()> {
    let input = Input::new_from_event(event)?;
    if let Some(file) = input.get_first_image_file()? {
        let reader = browser::file_reader()?;

        let reader_ref = Rc::new(RefCell::new(reader));
        let reader_clone = reader_ref.clone();

        let onload_closure = browser::create_event_closure(move |_event: Event| {
            let editor_clone = editor.clone();
            if let Err(err) = setup_input_event_closure_reader_closure(editor_clone, &reader_clone.borrow()) {
                error!("{:#?}", err);
            }
        });

        reader_ref.borrow_mut().set_onload(Some(onload_closure.as_ref().unchecked_ref()));
        onload_closure.forget();

        browser::file_reader_read_as_data_url(&reader_ref.borrow(), &file)?;
    }
    Ok(())
}

pub fn setup_input_event(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let input_element = browser::input("file_input")?;

    let closure = browser::create_event_closure(move |event: Event| {
        let editor_clone = editor.clone();
        if let Err(err) = setup_input_event_closure(editor_clone, event) {
            error!("{:#?}", err);
        }
    });

    input_element.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}
