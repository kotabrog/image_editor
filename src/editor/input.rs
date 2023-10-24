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

async fn internal_draw_image_fit_canvas_from_source(editor: Rc<Mutex<Editor>>, source: String, id: u16) -> Result<()> {
    if !Editor::try_run_id(&editor, id) {
        draw_image_fit_canvas_from_source(editor, source, id);
        return Ok(());
    }
    let image = Image::load_image(source.as_str())
         .await?;
    if let Some(mut editor) = Editor::try_lock(&editor) {
        editor.set_image(image);
        editor.update_canvas_size()?;
        editor.draw_image_fit_canvas()?;
        editor.setup_image_data()?;
        editor.to_idle();
    }
    Ok(())
}

pub fn draw_image_fit_canvas_from_source(editor: Rc<Mutex<Editor>>, source: String, id: u16) {
    browser::spawn_local(async move {
        if let Err(err) =
                internal_draw_image_fit_canvas_from_source(editor, source, id).await {
            error!("{:#?}", err);
        }
    });
}

fn setup_input_event_closure_reader_closure(editor: Rc<Mutex<Editor>>, reader: &FileReader, id: u16) -> Result<()> {
    let result = browser::file_reader_result(&reader)?;
    draw_image_fit_canvas_from_source(editor, result, id);
    Ok(())
}

fn setup_input_event_closure(editor: Rc<Mutex<Editor>>, event: Event) -> Result<()> {
    let id = if let Some(id) = Editor::try_run(&editor) {
        id
    } else {
        return Ok(());
    };
    let input = Input::new_from_event(&event)?;
    if let Some(file) = input.get_first_image_file()? {
        let reader = browser::file_reader()?;

        let reader_ref = Rc::new(RefCell::new(reader));
        let reader_clone = reader_ref.clone();

        let onload_closure = browser::create_event_closure(move |_event: Event| {
            let editor_clone = editor.clone();
            if let Err(err) = setup_input_event_closure_reader_closure(editor_clone, &reader_clone.borrow(), id) {
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
    let input_element = Input::new_from_id("file_input")?;

    let closure = browser::create_event_closure(move |event: Event| {
        let editor_clone = editor.clone();
        if let Err(err) = setup_input_event_closure(editor_clone, event) {
            error!("{:#?}", err);
        }
    });

    input_element.set_onchange(&closure);
    closure.forget();

    Ok(())
}
